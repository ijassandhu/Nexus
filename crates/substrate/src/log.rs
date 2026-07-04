//! Append-only segment log per specs/record.md §6.
//!
//! Frame layout: `u32 LE length ‖ frame bytes ‖ 32-byte BLAKE3(frame bytes)`.
//! Frame bytes are CBOR of { header_bytes, envelope }. Header bytes are stored
//! verbatim and reused as AAD on read. A torn final frame (crash mid-append)
//! is detected and skipped, never repaired in place.

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::crypto::Envelope;
use crate::event::EventHeader;

pub const SEGMENT_MAGIC: &[u8; 4] = b"NXL1";
pub const SEGMENT_ROLL_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Serialize, Deserialize)]
struct Frame {
    #[serde(with = "serde_bytes")]
    header_bytes: Vec<u8>,
    envelope: Envelope,
}

/// One decoded log record: parsed header, verbatim header bytes (AAD),
/// envelope, and the content address (BLAKE3 of frame bytes, §6).
pub struct Record {
    pub header: EventHeader,
    pub header_bytes: Vec<u8>,
    pub envelope: Envelope,
    pub content_address: [u8; 32],
}

pub struct SegmentLog {
    dir: PathBuf,
}

impl SegmentLog {
    pub fn open(dir: &Path) -> Result<Self> {
        fs::create_dir_all(dir)?;
        Ok(Self { dir: dir.to_path_buf() })
    }

    fn segment_paths(&self) -> Result<Vec<PathBuf>> {
        let mut paths: Vec<PathBuf> = fs::read_dir(&self.dir)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().map(|e| e == "nxl").unwrap_or(false))
            .collect();
        paths.sort();
        Ok(paths)
    }

    fn active_segment(&self) -> Result<PathBuf> {
        if let Some(last) = self.segment_paths()?.last() {
            if fs::metadata(last)?.len() < SEGMENT_ROLL_BYTES {
                return Ok(last.clone());
            }
        }
        let path = self
            .dir
            .join(format!("segment-{}.nxl", ulid::Ulid::new()));
        let mut f = fs::File::create(&path)?;
        f.write_all(SEGMENT_MAGIC)?;
        f.write_all(&[0u8])?; // format version 0
        f.sync_all()?;
        Ok(path)
    }

    /// Append one frame; fsyncs before returning (v0: every append — the
    /// spec's batched-fsync window for observations is an optimization for
    /// later, correctness first).
    pub fn append(&self, header_bytes: &[u8], envelope: &Envelope) -> Result<[u8; 32]> {
        let frame = Frame {
            header_bytes: header_bytes.to_vec(),
            envelope: envelope.clone(),
        };
        let mut frame_bytes = Vec::new();
        ciborium::ser::into_writer(&frame, &mut frame_bytes)?;
        let checksum = blake3::hash(&frame_bytes);

        let path = self.active_segment()?;
        let mut f = fs::OpenOptions::new().append(true).open(&path)?;
        f.write_all(&(frame_bytes.len() as u32).to_le_bytes())?;
        f.write_all(&frame_bytes)?;
        f.write_all(checksum.as_bytes())?;
        f.sync_all()?;
        Ok(*checksum.as_bytes())
    }

    /// Read every record across all segments, oldest first. Checksum
    /// mismatches fail hard; a torn final frame is skipped with a count.
    pub fn read_all(&self) -> Result<(Vec<Record>, usize)> {
        let mut records = Vec::new();
        let mut torn = 0usize;
        for path in self.segment_paths()? {
            let data = fs::read(&path)?;
            if data.len() < 5 || &data[0..4] != SEGMENT_MAGIC {
                bail!("bad segment header: {}", path.display());
            }
            let mut pos = 5usize;
            while pos < data.len() {
                if pos + 4 > data.len() {
                    torn += 1;
                    break;
                }
                let len = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
                let frame_end = pos + 4 + len;
                let rec_end = frame_end + 32;
                if rec_end > data.len() {
                    torn += 1;
                    break;
                }
                let frame_bytes = &data[pos + 4..frame_end];
                let stored: [u8; 32] = data[frame_end..rec_end].try_into().unwrap();
                let computed = blake3::hash(frame_bytes);
                if *computed.as_bytes() != stored {
                    bail!(
                        "checksum mismatch in {} at offset {pos} — record tampered or corrupt",
                        path.display()
                    );
                }
                let frame: Frame =
                    ciborium::de::from_reader(frame_bytes).context("frame decode")?;
                let header = EventHeader::from_bytes(&frame.header_bytes)?;
                records.push(Record {
                    header,
                    header_bytes: frame.header_bytes,
                    envelope: frame.envelope,
                    content_address: *computed.as_bytes(),
                });
                pos = rec_end;
            }
        }
        Ok((records, torn))
    }
}
