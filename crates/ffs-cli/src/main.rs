//! `ffs`: headless host for FFS! (play, JSONL, bot, replay, content validate).

mod bot;

use std::io::{BufRead, Write};
use std::sync::Arc;

use ffs_content::Pack;
use ffs_core::{Command, Engine};

use bot::Strategy;

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

fn load_pack() -> Arc<Pack> {
    match Pack::base() {
        Ok(p) => Arc::new(p),
        Err(e) => {
            eprintln!("content error: {e}");
            std::process::exit(2);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let seed: u64 = arg(&args, "--seed")
        .and_then(|s| s.parse().ok())
        .unwrap_or(42);
    match args.first().map(String::as_str) {
        Some("content") => content(&args),
        Some("play") => play(seed),
        Some("jsonl") => jsonl(seed),
        Some("bot") => bot_report(&args),
        Some("record") => record(&args, seed),
        Some("replay") => replay(&args),
        _ => {
            eprintln!(
                "usage: ffs <content validate | play [--seed N] | jsonl [--seed N] | \
                 bot [--runs N] | record --strategy S --seed N --out FILE | replay --verify FILE...>"
            );
            std::process::exit(1);
        }
    }
}

fn content(args: &[String]) {
    let pack = load_pack();
    let errs = pack.validate();
    if args.get(1).map(String::as_str) == Some("stats") || errs.is_empty() {
        let normal = pack.incidents.iter().filter(|i| !i.system).count();
        println!(
            "incidents: {normal} (+{} system), rivals: {}, dirt: {}, endings: {}, schemes: {}",
            pack.incidents.len() - normal,
            pack.rivals.len(),
            pack.dirt.len(),
            pack.endings.len(),
            pack.schemes.len()
        );
    }
    if errs.is_empty() {
        println!("content OK");
    } else {
        for e in &errs {
            eprintln!("ERROR: {e}");
        }
        std::process::exit(1);
    }
}

fn show(e: &Engine) {
    let v = e.view();
    let h = &v.header;
    println!(
        "\n=== DAY {} {} | FUCKS {}/100 | SUSP {} | ${} | NP {} ({}, #{}) ===",
        h.day, h.clock, h.fucks, h.susp, h.cash, h.np, h.title, h.rank
    );
    if !h.toast.is_empty() {
        println!("> {}", h.toast);
    }
    for t in &v.tickets {
        let nth = if t.nth > 1 {
            format!(" ({}x)", t.nth)
        } else {
            String::new()
        };
        println!(
            "\n#{} {}{} from {}, {}",
            t.id, t.title, nth, t.who_name, t.who_role
        );
        println!("   \"{}\"", t.quote);
        for o in &t.options {
            let hint: Vec<String> = o.hint.iter().map(|c| c.text.clone()).collect();
            println!("   [{}] {}   ({})", o.n + 1, o.text, hint.join(", "));
        }
        if let Some(d) = &t.dirt {
            println!("   [d] play dirt on {}: {}", d.name, d.label);
        }
    }
    println!(
        "\nschemes: {}",
        v.schemes
            .iter()
            .enumerate()
            .map(|(i, s)| format!(
                "s{} {}{}",
                i + 1,
                s.name,
                if s.available { "" } else { " (done)" }
            ))
            .collect::<Vec<_>>()
            .join(" | ")
    );
    println!(
        "stories: {}",
        v.stories
            .iter()
            .map(|s| format!("b{} I {}", s.id, s.text))
            .collect::<Vec<_>>()
            .join(" | ")
    );
    println!("commands: <ticket> <option> | d <ticket> | r | rs | c | s<n> | b<id> | h | e | q(retire) | x(quit)");
}

fn play(seed: u64) {
    let mut e = Engine::new(load_pack(), seed);
    let stdin = std::io::stdin();
    loop {
        if let Some(end) = e.view().ended {
            println!("\n##### {} #####", end.title);
            for l in &end.lines {
                println!("{l}");
            }
            println!("\n{}", end.share);
            return;
        }
        show(&e);
        print!("> ");
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
            return;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let v = e.view();
        let cmd = match parts.as_slice() {
            ["x"] => return,
            ["r"] => Some(Command::Rant { send: false }),
            ["rs"] => Some(Command::Rant { send: true }),
            ["c"] => Some(Command::Coffee),
            ["h"] => Some(Command::HelpOpened),
            ["e"] => Some(Command::EndDay),
            ["q"] => Some(Command::Retire),
            ["d", t] => t.parse().ok().map(|ticket| Command::UseDirt { ticket }),
            [s] if s.starts_with('s') => s[1..]
                .parse::<usize>()
                .ok()
                .and_then(|n| v.schemes.get(n.wrapping_sub(1)))
                .map(|s| Command::Scheme { id: s.id.clone() }),
            [b] if b.starts_with('b') => b[1..].parse().ok().map(|story| Command::Boast { story }),
            [t, o] => match (t.parse(), o.parse::<u8>()) {
                (Ok(ticket), Ok(o)) if o >= 1 => Some(Command::Resolve {
                    ticket,
                    option: o - 1,
                }),
                _ => None,
            },
            _ => None,
        };
        match cmd {
            Some(c) => {
                if let Err(err) = e.apply(c) {
                    println!("! {err}");
                }
            }
            None => println!("? try again"),
        }
    }
}

fn jsonl(seed: u64) {
    let mut e = Engine::new(load_pack(), seed);
    let out = |v: serde_json::Value| println!("{v}");
    out(serde_json::json!({"view": e.view()}));
    for line in std::io::stdin().lock().lines().map_while(Result::ok) {
        match serde_json::from_str::<Command>(&line) {
            Ok(c) => match e.apply(c) {
                Ok(ev) => out(serde_json::json!({"ok": true, "events": ev, "view": e.view()})),
                Err(err) => out(serde_json::json!({"ok": false, "error": err.to_string()})),
            },
            Err(err) => out(serde_json::json!({"ok": false, "error": err.to_string()})),
        }
    }
}

fn median(v: &mut [u32]) -> u32 {
    v.sort_unstable();
    v.get(v.len() / 2).copied().unwrap_or(0)
}

fn bot_report(args: &[String]) {
    let runs: u64 = arg(args, "--runs")
        .and_then(|s| s.parse().ok())
        .unwrap_or(300);
    let pack = load_pack();
    println!(
        "{:<8} {:>6} {:>6} {:>6} {:>7} {:>7} {:>7} {:>6} {:>7} {:>8}",
        "strategy",
        "med.d",
        "min.d",
        "max.d",
        "meltdn%",
        "fired%",
        "retire%",
        "ran%",
        "actions",
        "maxcash"
    );
    let assert_mode = args.iter().any(|a| a == "--assert");
    let mut problems: Vec<String> = vec![];
    for (name, strat) in [
        ("random", Strategy::Random),
        ("lazy", Strategy::Lazy),
        ("clever", Strategy::Clever),
    ] {
        let (mut days, mut acts) = (vec![], vec![]);
        let mut maxcash = 0;
        let (mut melt, mut fired, mut ret, mut ran) = (0u64, 0u64, 0u64, 0u64);
        for seed in 0..runs {
            let e = bot::play(pack.clone(), seed, strat, 2000);
            days.push(e.state().day);
            acts.push(e.log().len() as u32);
            maxcash = maxcash.max(e.state().cash);
            match e.state().ended.as_ref().map(|x| x.kind) {
                Some(ffs_core::EndKind::Meltdown) => melt += 1,
                Some(ffs_core::EndKind::Fired) => fired += 1,
                Some(ffs_core::EndKind::Retired) => ret += 1,
                Some(ffs_core::EndKind::Ran) => ran += 1,
                None => {}
            }
        }
        let pc = |n: u64| n as f64 * 100.0 / runs as f64;
        let med = median(&mut days.clone());
        let (mn, mx) = (
            *days.iter().min().unwrap_or(&0),
            *days.iter().max().unwrap_or(&0),
        );
        match name {
            "lazy" if med > 6 => problems.push(format!("lazy median {med} days should be <= 6")),
            "clever" if !(8..=16).contains(&med) => {
                problems.push(format!("clever median {med} days should be 8-16"))
            }
            "clever" if pc(melt) < 70.0 => {
                problems.push("clever meltdown rate should be >= 70%".to_string())
            }
            "random" if pc(ret) > 1.0 => problems.push("random retires too often".to_string()),
            _ => {}
        }
        println!(
            "{:<8} {:>6} {:>6} {:>6} {:>7.1} {:>7.1} {:>7.1} {:>6.1} {:>7} {:>8}",
            name,
            median(&mut days),
            mn,
            mx,
            pc(melt),
            pc(fired),
            pc(ret),
            pc(ran),
            median(&mut acts),
            maxcash
        );
    }
    if assert_mode && !problems.is_empty() {
        for p in &problems {
            eprintln!("BALANCE: {p}");
        }
        std::process::exit(1);
    }
}

fn record(args: &[String], seed: u64) {
    let strat = arg(args, "--strategy")
        .and_then(|s| Strategy::parse(&s))
        .unwrap_or(Strategy::Clever);
    let Some(out) = arg(args, "--out") else {
        eprintln!("--out FILE required");
        std::process::exit(1);
    };
    let e = bot::play(load_pack(), seed, strat, 2000);
    let j = serde_json::json!({"seed": seed, "log": e.log(), "hash": e.state_hash()});
    std::fs::write(&out, serde_json::to_string_pretty(&j).unwrap_or_default()).expect("write");
    println!("recorded {} commands -> {out}", e.log().len());
}

fn replay(args: &[String]) {
    let files: Vec<&String> = args
        .iter()
        .skip(1)
        .filter(|a| !a.starts_with("--"))
        .collect();
    let pack = load_pack();
    let mut bad = 0;
    for f in &files {
        let ok = std::fs::read_to_string(f)
            .ok()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
            .and_then(|v| {
                let seed = v["seed"].as_u64()?;
                let log: Vec<Command> = serde_json::from_value(v["log"].clone()).ok()?;
                let e = Engine::replay(pack.clone(), seed, &log).ok()?;
                Some(v["hash"].as_str()? == e.state_hash())
            })
            .unwrap_or(false);
        println!("{} {f}", if ok { "OK  " } else { "FAIL" });
        if !ok {
            bad += 1;
        }
    }
    if bad > 0 {
        std::process::exit(1);
    }
}
