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
        let cmd = match next() % 10 {
            0 => Command::Coffee,
            9 => Command::CoverTracks,
            1 => Command::Rant {
                send: next() % 2 == 0,
            },
            2 => Command::EndDay,
            3 => Command::Boast {
                story: st.stories.first().map(|s| s.id).unwrap_or(999),
            },
            4 => Command::Scheme {
                id: st
                    .scheme_offers
                    .get((next() % 4) as usize)
                    .cloned()
                    .unwrap_or_else(|| "nope".to_string()),
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
    let first = e.state().scheme_offers[0].clone();
    e.apply(Command::Scheme { id: first.clone() })
        .expect("first scheme ok");
    assert_eq!(
        e.apply(Command::Scheme { id: first }),
        Err(CommandError::SchemeNotOffered),
        "a scheme works once per run"
    );
    let later = e.state().scheme_ladder.last().cloned().expect("ladder");
    assert_eq!(
        e.apply(Command::Scheme { id: later }),
        Err(CommandError::SchemeNotOffered),
        "schemes further up the ladder are not on offer yet"
    );
}

#[test]
fn help_costs_one_np_and_is_logged() {
    let mut e = Engine::new(pack(), 3);
    // earn some NP first so the cost is visible
    e.apply(Command::Scheme {
        id: e.state().scheme_offers[0].clone(),
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
        id: e.state().scheme_offers[0].clone(),
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
    // A greedy schemer who never covers their tracks climbs the ladder until fired.
    for seed in 0..20 {
        let mut e = Engine::new(pack(), seed);
        for _ in 0..200 {
            if e.is_over() {
                break;
            }
            let cmd = match e.state().scheme_offers.first() {
                Some(id) => Command::Scheme { id: id.clone() },
                None => Command::EndDay,
            };
            if e.apply(cmd).is_err() {
                let _ = e.apply(Command::EndDay);
            }
        }
        if let Some(end) = &e.state().ended {
            seen.insert(format!("{:?}", end.kind));
        }
    }
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

#[test]
fn covering_tracks_lowers_suspicion_less_each_time() {
    use ffs_core::consts::{COVER_FUCKS, COVER_SUSP};
    let mut e = Engine::new(pack(), 4);
    e.apply(Command::Scheme {
        id: e.state().scheme_offers[0].clone(),
    })
    .expect("scheme");
    e.apply(Command::Scheme {
        id: e.state().scheme_offers[0].clone(),
    })
    .expect("scheme");
    let day = e.state().day;
    assert!(e.state().susp > 0);
    for &want in &COVER_SUSP[..2] {
        let (s0, f0) = (e.state().susp, e.state().fucks);
        assert_eq!(e.view().today.cover_gain, want);
        e.apply(Command::CoverTracks).expect("cover");
        assert_eq!(e.state().day, day, "still the same day");
        // it takes off the full amount, or everything that was left
        assert_eq!(s0 - e.state().susp, want.min(s0));
        assert!(e.state().fucks <= f0 - COVER_FUCKS, "covering is tedious");
    }
    assert!(COVER_SUSP[1] < COVER_SUSP[0], "diminishing returns");
    // a new day resets the diminishing returns
    e.apply(Command::EndDay).expect("end day");
    if !e.is_over() {
        assert_eq!(e.view().today.cover_gain, COVER_SUSP[0]);
    }
}

#[test]
fn covering_tracks_never_goes_below_zero() {
    let mut e = Engine::new(pack(), 6);
    assert_eq!(e.state().susp, 0);
    e.apply(Command::CoverTracks).expect("cover");
    assert_eq!(e.state().susp, 0);
}

fn tier_of(p: &Pack, id: &str) -> u8 {
    p.schemes
        .iter()
        .find(|s| s.id == id)
        .map(|s| s.tier)
        .expect("scheme")
}

#[test]
fn scheme_ladder_starts_petty_and_escalates() {
    let p = pack();
    let min_tier = p.schemes.iter().map(|s| s.tier).min().expect("schemes");
    for seed in 0..30 {
        let e = Engine::new(p.clone(), seed);
        let st = e.state();
        assert_eq!(st.scheme_offers.len(), ffs_core::consts::SCHEME_SLOTS);
        for id in &st.scheme_offers {
            assert_eq!(
                tier_of(&p, id),
                min_tier,
                "seed {seed}: opens with petty schemes"
            );
        }
        // the ladder never steps down a tier
        let tiers: Vec<u8> = st.scheme_ladder.iter().map(|id| tier_of(&p, id)).collect();
        assert!(
            tiers.windows(2).all(|w| w[0] <= w[1]),
            "seed {seed}: {tiers:?}"
        );
        // every scheme appears exactly once between the offers and the ladder
        let mut all: Vec<String> = st
            .scheme_offers
            .iter()
            .chain(st.scheme_ladder.iter())
            .cloned()
            .collect();
        all.sort();
        let mut want: Vec<String> = p.schemes.iter().map(|s| s.id.clone()).collect();
        want.sort();
        assert_eq!(all, want);
    }
}

#[test]
fn taking_a_scheme_puts_the_next_rung_in_its_slot() {
    let mut e = Engine::new(pack(), 12);
    let taken = e.state().scheme_offers[1].clone();
    let next = e.state().scheme_ladder[0].clone();
    let others = (
        e.state().scheme_offers[0].clone(),
        e.state().scheme_offers[2].clone(),
    );
    e.apply(Command::Scheme { id: taken.clone() })
        .expect("scheme");
    let st = e.state();
    assert_eq!(st.scheme_offers[1], next, "same slot, next rung");
    assert_eq!(
        (st.scheme_offers[0].clone(), st.scheme_offers[2].clone()),
        others
    );
    assert!(!st.scheme_offers.contains(&taken) && !st.scheme_ladder.contains(&taken));
    assert_eq!(st.schemes_run, 1);
}

#[test]
fn scheme_order_varies_between_runs() {
    let p = pack();
    let orders: std::collections::BTreeSet<Vec<String>> = (0..20)
        .map(|seed| Engine::new(p.clone(), seed).state().scheme_ladder.clone())
        .collect();
    assert!(orders.len() > 1, "every run has the same scheme order");
}

#[test]
fn the_ladder_runs_dry() {
    let p = pack();
    let mut e = Engine::new(p.clone(), 3);
    let mut done = 0;
    while let Some(id) = e.state().scheme_offers.first().cloned() {
        if e.is_over() {
            break;
        }
        if e.apply(Command::Scheme { id }).is_err() {
            break;
        }
        done += 1;
        // keep the run alive: cool off and go home when needed
        let _ = e.apply(Command::CoverTracks);
    }
    if !e.is_over() {
        assert_eq!(done, p.schemes.len());
        assert!(e.state().scheme_offers.is_empty() && e.state().scheme_ladder.is_empty());
        assert_eq!(e.view().schemes_left, 0);
    }
}

#[test]
fn schemes_sometimes_hit_a_jackpot() {
    // Run an opening (tier-1) scheme across many seeds: anything above its cash range
    // is a jackpot, and jackpots must happen but stay rare.
    let p = pack();
    let mut jackpots = 0;
    let runs = 400;
    for seed in 0..runs {
        let mut e = Engine::new(p.clone(), seed);
        let id = e.state().scheme_offers[0].clone();
        let sc = p.schemes.iter().find(|s| s.id == id).expect("scheme");
        let c0 = e.state().cash;
        e.apply(Command::Scheme { id: id.clone() }).expect("scheme");
        let got = (e.state().cash - c0) as i64;
        assert!(
            (sc.cash[0]..=sc.cash[1] * sc.jackpot_mult[1]).contains(&got),
            "seed {seed}: {got}"
        );
        if got > sc.cash[1] {
            jackpots += 1;
            assert!(e.state().toast.starts_with("JACKPOT!"));
        }
    }
    assert!(jackpots > 0, "no jackpot in {runs} runs");
    assert!(jackpots < runs / 5, "too many jackpots: {jackpots}");
}
