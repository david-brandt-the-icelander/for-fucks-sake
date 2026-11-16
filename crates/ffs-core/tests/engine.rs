use std::sync::Arc;

use ffs_content::Pack;
use ffs_core::{Command, CommandError, EndKind, Engine};

fn pack() -> Arc<Pack> {
    Arc::new(Pack::base().expect("pack loads"))
}

#[test]
fn content_validates() {
    let errs = pack().validate();
    assert!(errs.is_empty(), "{errs:?}");
}

/// Tiny deterministic driver (not the game's RNG).
fn drive(seed: u64, steps: usize) -> Engine {
    let mut e = Engine::new(pack(), seed);
    let mut x = seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407)
        | 1;
    let mut next = move || {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x
    };
    for _ in 0..steps {
        if e.is_over() {
            break;
        }
        let st = e.state().clone();
        let cmd = match next() % 9 {
            0 => Command::Coffee,
            1 => Command::Rant {
                send: next() % 2 == 0,
            },
            2 => Command::EndDay,
            3 => Command::Boast {
                story: st.stories.first().map(|s| s.id).unwrap_or(999),
            },
            4 => Command::Scheme {
                id: ["phantom_licences", "consulting", "creative_expenses"][(next() % 3) as usize]
                    .to_string(),
            },
            _ => match st.queue.first() {
                Some(t) => Command::Resolve {
                    ticket: t.id,
                    option: (next() % 4) as u8,
                },
                None => Command::EndDay,
            },
        };
        let before = e.state_hash();
        if e.apply(cmd).is_err() {
            assert_eq!(
                before,
                e.state_hash(),
                "a rejected command must not change state"
            );
        }
        let s = e.state();
        assert!((0..=100).contains(&s.fucks), "fucks in bounds: {}", s.fucks);
        assert!((0..=100).contains(&s.susp), "susp in bounds: {}", s.susp);
        assert!(s.cash >= 0 && s.np >= 0);
        assert!(s.queue.len() <= ffs_core::consts::MAX_QUEUE);
    }
    e
}

#[test]
fn meters_stay_in_bounds_and_rejections_do_not_mutate() {
    for seed in 0..60 {
        drive(seed, 400);
    }
}

#[test]
fn replay_reproduces_state_hash() {
    for seed in 0..25 {
        let e = drive(seed, 300);
        let r = Engine::replay(pack(), seed, e.log()).expect("replays");
        assert_eq!(e.state_hash(), r.state_hash(), "seed {seed}");
    }
}

#[test]
fn same_seed_same_run_different_seed_differs() {
    let a = Engine::new(pack(), 7);
    let b = Engine::new(pack(), 7);
    let c = Engine::new(pack(), 8);
    assert_eq!(a.state_hash(), b.state_hash());
    assert_ne!(a.state_hash(), c.state_hash());
}

#[test]
fn bad_commands_are_rejected() {
    let mut e = Engine::new(pack(), 1);
    assert_eq!(
        e.apply(Command::Resolve {
            ticket: 9999,
            option: 0
        }),
        Err(CommandError::NoSuchTicket)
    );
    let t = e.state().queue[0].id;
    assert_eq!(
        e.apply(Command::Resolve {
            ticket: t,
            option: 9
        }),
        Err(CommandError::NoSuchOption)
    );
    assert_eq!(
        e.apply(Command::UseDirt { ticket: t }),
        Err(CommandError::NoDirt)
    );
    assert_eq!(
        e.apply(Command::Scheme { id: "nope".into() }),
        Err(CommandError::NoSuchScheme)
    );
    assert_eq!(
        e.apply(Command::Boast { story: 1 }),
        Err(CommandError::NoSuchStory)
    );
    e.apply(Command::Scheme {
        id: "consulting".into(),
    })
    .expect("first scheme ok");
    assert_eq!(
        e.apply(Command::Scheme {
            id: "consulting".into()
        }),
        Err(CommandError::SchemeUsedToday)
    );
}

#[test]
fn help_costs_one_np_and_is_logged() {
    let mut e = Engine::new(pack(), 3);
    // earn some NP first so the cost is visible
    e.apply(Command::Scheme {
        id: "phantom_licences".into(),
    })
    .expect("scheme");
    let sid = e.state().stories[0].id;
    e.apply(Command::Boast { story: sid }).expect("boast");
    let np = e.state().np;
    e.apply(Command::HelpOpened).expect("help");
    assert_eq!(e.state().np, np - 1);
    assert_eq!(e.state().np_log.last().map(|l| l.delta), Some(-1));
}

#[test]
fn schemes_create_stories_and_boasting_spends_them() {
    let mut e = Engine::new(pack(), 5);
    e.apply(Command::Scheme {
        id: "phantom_licences".into(),
    })
    .expect("scheme");
    assert!(!e.state().stories.is_empty());
    let id = e.state().stories[0].id;
    let susp = e.state().susp;
    e.apply(Command::Boast { story: id }).expect("boast");
    assert!(
        e.state().stories.iter().all(|s| s.id != id),
        "story is spent"
    );
    assert!(e.state().susp >= susp, "boasting about a scheme is risky");
}

#[test]
fn every_ending_is_reachable() {
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..200 {
        let e = drive(seed, 600);
        if let Some(end) = &e.state().ended {
            seen.insert(format!("{:?}", end.kind));
        }
        let mut r = Engine::new(pack(), seed);
        if r.apply(Command::Retire).is_ok() {
            seen.insert(format!("{:?}", r.state().ended.as_ref().map(|x| x.kind)));
        }
    }
    assert!(
        seen.contains(&format!("{:?}", EndKind::Meltdown)),
        "{seen:?}"
    );
    assert!(
        seen.contains(&format!("{:?}", EndKind::Fired)) || seen.contains("Some(Fired)"),
        "{seen:?}"
    );
    assert!(seen.iter().any(|s| s.contains("Ran")), "{seen:?}");
}

#[test]
fn retirement_needs_cash_and_low_suspicion() {
    // A fresh run has too little cash: F10 is "ran", never a legend.
    let mut e = Engine::new(pack(), 2);
    e.apply(Command::Retire).expect("retire");
    assert_eq!(e.state().ended.as_ref().map(|x| x.kind), Some(EndKind::Ran));
    assert_eq!(e.apply(Command::Coffee), Err(CommandError::RunEnded));
}

#[test]
fn view_serialises_and_hints_are_honest() {
    let e = Engine::new(pack(), 11);
    let v = e.view();
    assert!(!v.tickets.is_empty());
    let json = serde_json::to_string(&v).expect("json");
    assert!(json.contains("\"header\""));
    // every non-system option exposes at least one chip
    for t in &v.tickets {
        for o in &t.options {
            assert!(!o.hint.is_empty(), "option {} has no hint", o.text);
        }
    }
}
