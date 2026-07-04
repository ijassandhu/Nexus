//! `nx` — NEXUS CLI, Phase 0 walking skeleton.
//!
//! Passphrase comes from --passphrase or NX_PASSPHRASE. Dev-grade UX;
//! interactive prompt + OS keychain integration are Phase 1 concerns.

use std::io::IsTerminal;
use std::io::Write as _;
use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use substrate::event::{Kind, Origin, Privacy, Trust};
use substrate::{hex, BodyState, Substrate};

#[derive(Parser)]
#[command(name = "nx", about = "NEXUS personal intelligence substrate", version)]
struct Cli {
    /// Record directory
    #[arg(long, default_value = ".nexus", global = true)]
    data: PathBuf,
    /// Root passphrase (or set NX_PASSPHRASE)
    #[arg(long, global = true)]
    passphrase: Option<String>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Create a new record (prompts for a model provider in a terminal)
    Init {
        /// Skip the provider-onboarding prompt
        #[arg(long)]
        no_configure: bool,
    },
    /// Choose a model provider; the API key is sealed under your passphrase
    Configure {
        /// anthropic | openai | gemini | openrouter | ollama | lmstudio | openai-compat
        #[arg(long)]
        provider: Option<String>,
        #[arg(long)]
        model: Option<String>,
        /// API key (prefer --api-key-stdin or the interactive prompt: flags
        /// leak into shell history)
        #[arg(long)]
        api_key: Option<String>,
        /// Read the API key from stdin (first line)
        #[arg(long)]
        api_key_stdin: bool,
        /// Endpoint override (required for openai-compat; optional for
        /// ollama/lmstudio on non-default ports)
        #[arg(long)]
        base_url: Option<String>,
    },
    /// Append an event (dev command)
    Append {
        #[arg(long, default_value = "observation")]
        kind: String,
        #[arg(long, default_value = "dev.note/1")]
        schema: String,
        #[arg(long, default_value = "local")]
        trust: String,
        #[arg(long, default_value = "P1")]
        privacy: String,
        /// Body text
        body: String,
    },
    /// List events
    Log {
        /// Decrypt and show bodies
        #[arg(long)]
        decrypt: bool,
    },
    /// Crypto-shred an event's key and tombstone it
    Forget { id: String },
    /// Verify segment checksums and keyring coverage
    Verify,
    /// Workspace transactions (D-005): speculate, diff, commit or abort
    #[command(subcommand)]
    Txn(TxnCmd),
    /// Show the effects ledger (txn lifecycle + per-effect entries)
    Ledger {
        /// Verify Ed25519 signatures on every entry
        #[arg(long)]
        verify: bool,
    },
    /// Delegate a task: Steward → Worker (in a transaction) → Critic → inbox
    Do {
        /// The intent, in your words
        intent: String,
        /// Target directory the task operates on
        #[arg(long, default_value = ".")]
        target: PathBuf,
        /// Use the offline mock backend instead of a real model
        #[arg(long)]
        mock: bool,
    },
    /// Review pending work: effect lists + Critic verdicts
    Inbox,
    /// Approve a pending transaction's effect list and apply it
    Approve { txn: String },
    /// Reject a pending transaction (aborts; target untouched)
    Reject {
        txn: String,
        /// Why — recorded as training signal
        #[arg(long)]
        reason: String,
    },
    /// Per-reasoner track records: consultants are measured
    Scorecard,
    /// Query your own record: ranked recalls with provenance
    Recall {
        query: String,
        #[arg(long, default_value_t = 8)]
        budget: usize,
    },
    /// Appraise recent episodes into bets (the night shift, run once per sitting)
    Consolidate {
        /// Use the offline mock backend
        #[arg(long)]
        mock: bool,
    },
    /// List bets: positions with earned scores, never asserted confidence
    Bets {
        /// The litmus test: only lost positions — what died, and what killed it
        #[arg(long)]
        lost: bool,
    },
    /// Place or resolve a bet (specs/record.md §11)
    #[command(subcommand)]
    Bet(BetCmd),
    /// Re-derive the SQLite index from the log (the D-013 escape hatch)
    Rebuild,
    /// Query the derived index
    Query {
        /// SQL LIKE pattern, e.g. "txn.%"
        #[arg(long)]
        schema: Option<String>,
        #[arg(long)]
        trust: Option<String>,
        #[arg(long)]
        kind: Option<String>,
        /// Only events referencing this event id
        #[arg(long)]
        references: Option<String>,
        #[arg(long)]
        limit: Option<u32>,
    },
}

#[derive(Subcommand)]
enum BetCmd {
    /// Place a bet (admission rule: at least one --falsifier)
    Place {
        statement: String,
        /// Observable condition that would kill this bet (repeatable)
        #[arg(long = "falsifier", required = true)]
        falsifiers: Vec<String>,
        #[arg(long, default_value = "R1")]
        stakes: String,
        #[arg(long, default_value = "general")]
        scope: String,
        /// belief | prediction | premortem
        #[arg(long, default_value = "belief")]
        kind: String,
        /// Days until this bet must be re-earned
        #[arg(long)]
        horizon_days: Option<i64>,
        /// Bet id this one depends on (repeatable; retraction cascades)
        #[arg(long = "premise")]
        premises: Vec<String>,
    },
    /// Resolve a bet: held | falsified | expired | retracted
    Resolve {
        id: String,
        outcome: String,
        #[arg(long, default_value = "")]
        note: String,
    },
}

#[derive(Subcommand)]
enum TxnCmd {
    /// Snapshot a target directory into a workspace
    Begin { target: PathBuf },
    /// List transactions
    List,
    /// Show the truthful effect list (workspace vs snapshot) and any drift
    Diff { id: String },
    /// Apply the effect list to the target under capability checks
    Commit { id: String },
    /// Discard the workspace; the target was never touched
    Abort { id: String },
}

fn passphrase(cli: &Cli) -> Result<String> {
    if let Some(p) = &cli.passphrase {
        return Ok(p.clone());
    }
    if let Ok(p) = std::env::var("NX_PASSPHRASE") {
        return Ok(p);
    }
    bail!("no passphrase: pass --passphrase or set NX_PASSPHRASE");
}

fn parse_kind(s: &str) -> Result<Kind> {
    Ok(match s {
        "observation" => Kind::Observation,
        "action" => Kind::Action,
        "feedback" => Kind::Feedback,
        "system" => Kind::System,
        _ => bail!("kind must be observation|action|feedback|system"),
    })
}

fn parse_trust(s: &str) -> Result<Trust> {
    Ok(match s {
        "user" => Trust::User,
        "local" => Trust::Local,
        "derived" => Trust::Derived,
        "external" => Trust::External,
        _ => bail!("trust must be user|local|derived|external"),
    })
}

fn parse_privacy(s: &str) -> Result<Privacy> {
    Ok(match s.to_uppercase().as_str() {
        "P0" => Privacy::P0,
        "P1" => Privacy::P1,
        "P2" => Privacy::P2,
        "P3" => Privacy::P3,
        _ => bail!("privacy must be P0|P1|P2|P3"),
    })
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let pass = passphrase(&cli)?;

    match &cli.cmd {
        Cmd::Init { no_configure } => {
            let mut s = Substrate::init(&cli.data, &pass)?;
            println!("initialized record at {}", cli.data.display());
            if *no_configure {
                println!("provider not configured — run `nx configure` before `nx do`");
            } else if std::io::stdin().is_terminal() {
                println!("\nNEXUS needs a model provider (you can change this anytime with `nx configure`).");
                let choice = configure_interactive()?;
                runtime::onboarding::apply(&cli.data, &mut s, &choice)?;
                println!("configured: {} / {} (key sealed under your passphrase)", choice.provider, choice.model);
            } else {
                println!("non-interactive session — run `nx configure` to choose a model provider");
            }
        }
        Cmd::Configure { provider, model, api_key, api_key_stdin, base_url } => {
            let mut s = Substrate::open(&cli.data, &pass)?;
            let choice = match provider {
                None => {
                    if !std::io::stdin().is_terminal() {
                        anyhow::bail!("no terminal for prompts — pass --provider/--model (and --api-key-stdin)");
                    }
                    configure_interactive()?
                }
                Some(p) => {
                    let key = if *api_key_stdin {
                        let mut line = String::new();
                        std::io::stdin().read_line(&mut line)?;
                        Some(line.trim().to_string())
                    } else {
                        if api_key.is_some() {
                            eprintln!("note: --api-key can leak into shell history; prefer --api-key-stdin");
                        }
                        api_key.clone()
                    };
                    runtime::onboarding::ProviderChoice {
                        provider: p.clone(),
                        model: model.clone().unwrap_or_else(|| {
                            runtime::providers::suggested_model(p).to_string()
                        }),
                        api_key: key,
                        base_url: base_url.clone(),
                    }
                }
            };
            runtime::onboarding::apply(&cli.data, &mut s, &choice)?;
            println!("configured: {} / {} (key sealed under your passphrase)", choice.provider, choice.model);
        }
        Cmd::Append {
            kind,
            schema,
            trust,
            privacy,
            body,
        } => {
            let mut s = Substrate::open(&cli.data, &pass)?;
            let header = s.append(
                parse_kind(kind)?,
                schema,
                Origin {
                    adapter: "nx-cli".into(),
                    actor: "user".into(),
                    trust: parse_trust(trust)?,
                },
                parse_privacy(privacy)?,
                vec![],
                body.as_bytes(),
            )?;
            println!("{}", header.id);
        }
        Cmd::Log { decrypt } => {
            let s = Substrate::open(&cli.data, &pass)?;
            let (events, torn) = s.events(*decrypt)?;
            for e in &events {
                let body = match &e.body {
                    BodyState::Plain(b) => String::from_utf8_lossy(b).into_owned(),
                    BodyState::Forgotten => "[FORGOTTEN]".to_string(),
                    BodyState::Sealed => "[sealed]".to_string(),
                };
                println!(
                    "{}  {}  {:?}  {}  {:?}/{:?}  {}",
                    e.header.id,
                    e.header.ts.format("%Y-%m-%d %H:%M:%S"),
                    e.header.kind,
                    e.header.schema,
                    e.header.origin.trust,
                    e.header.privacy,
                    body
                );
            }
            eprintln!("{} events{}", events.len(), if torn > 0 { " (1+ torn frame skipped)" } else { "" });
        }
        Cmd::Forget { id } => {
            let mut s = Substrate::open(&cli.data, &pass)?;
            if s.forget(id)? {
                println!("forgotten: {id} (key shredded, tombstone appended)");
            } else {
                println!("nothing to forget: {id} (unknown or already forgotten)");
            }
        }
        Cmd::Verify => {
            let s = Substrate::open(&cli.data, &pass)?;
            let (count, torn, forgotten) = s.verify()?;
            println!("{count} events, checksums OK, {torn} torn frames, {forgotten} forgotten");
            // Prove content addresses are stable identities.
            let (events, _) = s.events(false)?;
            if let Some(last) = events.last() {
                println!("latest content address: {}", hex(&last.content_address));
            }
        }
        Cmd::Txn(sub) => {
            let mut s = Substrate::open(&cli.data, &pass)?;
            let mgr = kernel::txn::TxnManager::new(&cli.data)?;
            match sub {
                TxnCmd::Begin { target } => {
                    let meta = mgr.begin(&mut s, target)?;
                    println!("txn {} open", meta.id);
                    println!("  target:    {}", meta.target.display());
                    println!("  workspace: {}", mgr.workspace(&meta.id).display());
                    println!("  snapshot:  {} files", meta.manifest.len());
                    println!("work in the workspace, then `nx txn diff {}`", meta.id);
                }
                TxnCmd::List => {
                    for m in mgr.list()? {
                        println!(
                            "{}  {:?}  {}  {}",
                            m.id,
                            m.state,
                            m.created.format("%Y-%m-%d %H:%M:%S"),
                            m.target.display()
                        );
                    }
                }
                TxnCmd::Diff { id } => {
                    let (_, changes, drift) = mgr.diff(id)?;
                    print_changes(&changes);
                    if !drift.is_empty() {
                        println!("!! target drift (commit will refuse):");
                        for d in &drift {
                            println!("   {d}");
                        }
                    }
                }
                TxnCmd::Commit { id } => {
                    let changes = mgr.commit(&mut s, id)?;
                    print_changes(&changes);
                    println!("committed: {} effects applied and ledgered", changes.len());
                }
                TxnCmd::Abort { id } => {
                    mgr.abort(&mut s, id)?;
                    println!("aborted: workspace discarded, target untouched");
                }
            }
        }
        Cmd::Ledger { verify } => {
            let s = Substrate::open(&cli.data, &pass)?;
            let (events, _) = s.events(true)?;
            let mut bad = 0usize;
            for e in events.iter().filter(|e| {
                e.header.schema.starts_with("txn.") || e.header.schema.starts_with("ledger.")
            }) {
                let body = match &e.body {
                    BodyState::Plain(b) => String::from_utf8_lossy(b).into_owned(),
                    _ => "[unreadable]".to_string(),
                };
                let mark = if *verify {
                    match kernel::ledger::verify_body(&body) {
                        kernel::ledger::Verdict::Valid => "sig:OK ",
                        kernel::ledger::Verdict::Unsigned => "sig:-- ",
                        kernel::ledger::Verdict::Invalid => {
                            bad += 1;
                            "sig:BAD"
                        }
                    }
                } else {
                    ""
                };
                println!(
                    "{mark} {}  {}  {}  {}",
                    e.header.id,
                    e.header.ts.format("%Y-%m-%d %H:%M:%S"),
                    e.header.schema,
                    body
                );
            }
            if *verify {
                if bad > 0 {
                    anyhow::bail!("{bad} ledger entries failed signature verification");
                }
                println!("all signatures verified");
            }
        }
        Cmd::Do { intent, target, mock } => {
            let mut s = Substrate::open(&cli.data, &pass)?;
            let r = runtime::pipeline::run_intent(&cli.data, &mut s, intent, target, *mock)?;
            println!("task {}", r.task);
            for step in &r.plan {
                println!("  plan: {step}");
            }
            match &r.txn {
                Some(txn) => {
                    println!("  {} effects staged in txn {txn}", r.effects);
                    println!("  critic verdict: {}", r.verdict);
                    println!("review with `nx inbox`, then `nx approve {txn}` or `nx reject {txn} --reason ...`");
                }
                None => println!("  no operations proposed — nothing to review"),
            }
        }
        Cmd::Inbox => {
            let s = Substrate::open(&cli.data, &pass)?;
            let items = runtime::inbox::list(&cli.data, &s)?;
            if items.is_empty() {
                println!("inbox empty");
            }
            for i in &items {
                println!("txn {}  [critic: {}]", i.txn, i.verdict);
                println!("  intent: {}", i.intent);
                if !i.advisory_summary.is_empty() {
                    println!("  plan (advisory): {}", i.advisory_summary);
                }
                for e in &i.effects {
                    println!(
                        "    {} {}",
                        e.get("op").and_then(|o| o.as_str()).unwrap_or("?"),
                        e.get("path").and_then(|p| p.as_str()).unwrap_or("?")
                    );
                }
                for issue in &i.issues {
                    println!(
                        "  !! [{}] {}: {}",
                        issue.get("severity").and_then(|s| s.as_str()).unwrap_or("?"),
                        issue.get("target").and_then(|t| t.as_str()).unwrap_or("?"),
                        issue.get("argument").and_then(|a| a.as_str()).unwrap_or("")
                    );
                }
            }
        }
        Cmd::Approve { txn } => {
            let mut s = Substrate::open(&cli.data, &pass)?;
            let n = runtime::inbox::approve(&cli.data, &mut s, txn)?;
            println!("approved: {n} effects applied and ledgered");
        }
        Cmd::Reject { txn, reason } => {
            let mut s = Substrate::open(&cli.data, &pass)?;
            runtime::inbox::reject(&cli.data, &mut s, txn, reason)?;
            println!("rejected: transaction aborted, target untouched, reason recorded");
        }
        Cmd::Scorecard => {
            let s = Substrate::open(&cli.data, &pass)?;
            let rows = runtime::scorecard::reasoner_scorecard(&s)?;
            if rows.is_empty() {
                println!("no reasoner calls recorded yet");
            } else {
                println!(
                    "{:<14} {:<20} {:<12} {:>6} {:>8} {:>9} {:>9} {:>9}",
                    "provider", "model", "operator", "calls", "ok-rate", "avg-ms", "approved", "rejected"
                );
                for r in &rows {
                    println!(
                        "{:<14} {:<20} {:<12} {:>6} {:>7.0}% {:>9} {:>9} {:>9}",
                        r.provider, r.model, r.operator, r.calls, r.ok_rate() * 100.0, r.avg_ms(),
                        r.approved, r.rejected
                    );
                }
            }
        }
        Cmd::Recall { query, budget } => {
            let s = Substrate::open(&cli.data, &pass)?;
            let recalls = runtime::retrieval::recall_episodes(&s, query, *budget)?;
            if recalls.is_empty() {
                println!("nothing recalled");
            }
            for r in &recalls {
                println!("{:.3}  {}  {}  [{}]", r.score, r.when, r.id, r.source);
                println!("       {}", r.excerpt.replace('\n', " "));
            }
        }
        Cmd::Consolidate { mock } => {
            let mut s = Substrate::open(&cli.data, &pass)?;
            let r = runtime::consolidate::run(&cli.data, &mut s, *mock)?;
            println!(
                "scanned {} episodes → {} candidates → {} placed, {} duplicates, {} rejected",
                r.scanned, r.candidates, r.placed.len(), r.duplicates, r.rejected.len()
            );
            for id in &r.placed {
                println!("  placed: {id}");
            }
            for why in &r.rejected {
                println!("  rejected: {why}");
            }
        }
        Cmd::Bets { lost } => {
            let s = Substrate::open(&cli.data, &pass)?;
            let views = substrate::bets::views(&s)?;
            if *lost {
                let dead: Vec<_> = views.iter().filter(|v| v.status != "live").collect();
                if dead.is_empty() {
                    println!("no lost positions — nothing believed has died yet");
                }
                for v in dead {
                    println!("{}  LOST ({})", v.id, v.status);
                    println!("    believed: {}", v.bet.statement);
                    println!(
                        "    killed by: {}",
                        v.terminal_note.as_deref().unwrap_or("(unrecorded)")
                    );
                }
            } else {
                if views.is_empty() {
                    println!("no bets placed");
                }
                for v in &views {
                    let flag = if v.unjustified { "  [UNJUSTIFIED]" } else { "" };
                    println!(
                        "{}  {:?}/{}  {}  held:{} falsified:{}{}",
                        v.id, v.bet.kind, v.bet.stakes, v.status, v.held, v.falsified_count, flag
                    );
                    println!("    {}", v.bet.statement);
                    for f in &v.bet.falsifiers {
                        println!("    dies if: {f}");
                    }
                }
            }
        }
        Cmd::Bet(sub_cmd) => {
            let mut s = Substrate::open(&cli.data, &pass)?;
            match sub_cmd {
                BetCmd::Place { statement, falsifiers, stakes, scope, kind, horizon_days, premises } => {
                    let kind = match kind.as_str() {
                        "belief" => substrate::bets::BetKind::Belief,
                        "prediction" => substrate::bets::BetKind::Prediction,
                        "premortem" => substrate::bets::BetKind::Premortem,
                        _ => bail!("kind must be belief|prediction|premortem"),
                    };
                    let id = substrate::bets::place(
                        &mut s,
                        Origin { adapter: "nx-cli".into(), actor: "user".into(), trust: Trust::User },
                        Privacy::P1,
                        vec![],
                        &substrate::bets::Bet {
                            statement: statement.clone(),
                            scope: scope.clone(),
                            kind,
                            stakes: stakes.clone(),
                            falsifiers: falsifiers.clone(),
                            horizon: horizon_days.map(|d| chrono::Utc::now() + chrono::Duration::days(d)),
                            premises: premises.clone(),
                        },
                    )?;
                    println!("{id}");
                }
                BetCmd::Resolve { id, outcome, note } => {
                    let outcome = substrate::bets::Outcome::parse(outcome)?;
                    let cascaded = substrate::bets::resolve(&mut s, id, outcome, note, "user")?;
                    println!(
                        "resolved {id} as {outcome:?}{}",
                        if cascaded > 0 {
                            format!(" — {cascaded} dependent bets marked unjustified (RECONCILE)")
                        } else {
                            String::new()
                        }
                    );
                }
            }
        }
        Cmd::Rebuild => {
            let s = Substrate::open(&cli.data, &pass)?;
            let (store, n, n_refs) = substrate::derived::DerivedStore::rebuild(&cli.data, &s)?;
            let (total, forgotten, schemas) = store.stats()?;
            println!("rebuilt {} from the log", store.path.display());
            println!("  {n} events, {n_refs} refs, {schemas} schemas, {forgotten} forgotten");
            debug_assert_eq!(n as i64, total);
        }
        Cmd::Query {
            schema,
            trust,
            kind,
            references,
            limit,
        } => {
            let store = substrate::derived::DerivedStore::open(&cli.data)?;
            let rows = store.query(&substrate::derived::QueryFilter {
                schema: schema.clone(),
                trust: trust.clone(),
                kind: kind.clone(),
                references: references.clone(),
                limit: *limit,
            })?;
            for r in &rows {
                println!(
                    "{:>5}  {}  {}  {}  {}/{}{}",
                    r.seq,
                    r.id,
                    r.ts,
                    r.schema,
                    r.trust,
                    r.privacy,
                    if r.forgotten { "  [FORGOTTEN]" } else { "" }
                );
            }
            eprintln!("{} rows", rows.len());
        }
    }
    Ok(())
}

/// Interactive provider onboarding: numbered choice, model with suggested
/// default, masked key entry (never echoed, never in shell history).
fn configure_interactive() -> Result<runtime::onboarding::ProviderChoice> {
    let choices: Vec<&str> = runtime::providers::PROVIDER_IDS
        .iter()
        .copied()
        .filter(|p| *p != "mock")
        .collect();
    println!("model providers:");
    for (i, p) in choices.iter().enumerate() {
        let hint = match *p {
            "ollama" | "lmstudio" => " (local, no key)",
            "openai-compat" => " (any OpenAI-compatible endpoint)",
            _ => "",
        };
        println!("  {}. {p}{hint}", i + 1);
    }
    let n: usize = loop {
        let s = prompt(&format!("choose [1-{}]: ", choices.len()))?;
        match s.trim().parse::<usize>() {
            Ok(n) if (1..=choices.len()).contains(&n) => break n,
            _ => println!("enter a number between 1 and {}", choices.len()),
        }
    };
    let provider = choices[n - 1].to_string();

    let suggested = runtime::providers::suggested_model(&provider);
    let model_in = prompt(&if suggested.is_empty() {
        "model id: ".to_string()
    } else {
        format!("model id [{suggested}]: ")
    })?;
    let model = if model_in.trim().is_empty() { suggested.to_string() } else { model_in.trim().to_string() };

    let needs_key = runtime::providers::get(&provider).map(|p| p.needs_key()).unwrap_or(false);
    let api_key = if needs_key {
        let k = rpassword::prompt_password(format!("{provider} API key (input hidden): "))?;
        Some(k.trim().to_string())
    } else {
        None
    };

    let base_url = if provider == "openai-compat" {
        Some(prompt("base URL (e.g. http://localhost:8080/v1): ")?.trim().to_string())
    } else if provider == "ollama" || provider == "lmstudio" {
        let b = prompt("base URL [default port]: ")?;
        let b = b.trim().to_string();
        if b.is_empty() { None } else { Some(b) }
    } else {
        None
    };

    Ok(runtime::onboarding::ProviderChoice { provider, model, api_key, base_url })
}

fn prompt(msg: &str) -> Result<String> {
    print!("{msg}");
    std::io::stdout().flush()?;
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    Ok(line)
}

fn print_changes(changes: &[kernel::txn::Change]) {
    use kernel::txn::ChangeOp;
    if changes.is_empty() {
        println!("no changes");
        return;
    }
    for c in changes {
        let (tag, detail) = match c.op {
            ChangeOp::Created => ("A", format!("→ {}", short(&c.after))),
            ChangeOp::Modified => ("M", format!("{} → {}", short(&c.before), short(&c.after))),
            ChangeOp::Deleted => ("D", format!("{} →", short(&c.before))),
        };
        println!("  {tag} {}  {detail}", c.path);
    }
}

fn short(h: &Option<String>) -> String {
    h.as_deref().map(|s| s[..8].to_string()).unwrap_or_default()
}
