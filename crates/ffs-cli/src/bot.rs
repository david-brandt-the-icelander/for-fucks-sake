//! Scripted players for balance testing and replay corpus generation.

use ffs_content::{Effect, Pack};
use ffs_core::consts::*;
use ffs_core::{Command, Engine};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Strategy {
    Random,
    Lazy,
    Clever,
}

impl Strategy {
    pub fn parse(s: &str) -> Option<Strategy> {
        match s {
            "random" => Some(Strategy::Random),
            "lazy" => Some(Strategy::Lazy),
            "clever" => Some(Strategy::Clever),
            _ => None,
        }
    }
}

/// Tiny deterministic PRNG for the bot (not part of the game's determinism).
pub struct Xs(pub u64);
impl Xs {
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next() % n as u64) as usize
        }
    }
}

fn score_effects(effs: &[Effect], p: f64, wf: f64, ws: f64) -> f64 {
    let mut s = 0.0;
    for e in effs {
        s += e.fucks.unwrap_or(0) as f64 * wf;
        s -= e.susp.unwrap_or(0) as f64 * ws;
        s += e.cash.unwrap_or(0) as f64 / 400.0;
        s += e.np.unwrap_or(0) as f64 * 1.0;
        if e.dirt == Some(true) {
            s += 3.0;
        }
        if e.spawn.is_some() {
            s -= 2.0;
        }
        if let (Some(c), Some(t)) = (e.chance, &e.then) {
            s += score_effects(t, c as f64 / 100.0, wf, ws);
        }
    }
    s * p
}

pub fn next_command(engine: &Engine, strat: Strategy, rng: &mut Xs) -> Command {
    let st = engine.state();
    let pack: &Pack = engine.pack();
    match strat {
        Strategy::Lazy => {
            let Some(t) = st.queue.first() else {
                return Command::EndDay;
            };
            Command::Resolve {
                ticket: t.id,
                option: 0,
            }
        }
        Strategy::Random => {
            if !st.queue.is_empty() && rng.below(10) < 8 {
                let t = &st.queue[rng.below(st.queue.len())];
                let n = pack
                    .incident(&t.incident)
                    .map(|i| i.options.len())
                    .unwrap_or(1);
                Command::Resolve {
                    ticket: t.id,
                    option: rng.below(n) as u8,
                }
            } else {
                match rng.below(3) {
                    0 => Command::Coffee,
                    1 => Command::Rant {
                        send: rng.below(2) == 0,
                    },
                    _ => Command::EndDay,
                }
            }
        }
        Strategy::Clever => {
            if st.cash >= RETIRE_GOAL && st.susp < RETIRE_MAX_SUSP - 10 {
                return Command::Retire;
            }
            if st.fucks < 35 && st.rants_today < 2 {
                return Command::Rant { send: false };
            }
            if st.fucks < 45 && st.coffees_today < 1 {
                return Command::Coffee;
            }
            if let Some(sc) = st.stories.last() {
                if st.boasts_today < 2 && !sc.risky {
                    return Command::Boast { story: sc.id };
                }
            }
            // cool off before scheming again, while the first cover of the day is worth it
            if st.susp >= 30 && st.covers_today == 0 && st.fucks > 30 {
                return Command::CoverTracks;
            }
            if st.susp < 25 {
                // the cheapest-risk offer first
                let best = st
                    .scheme_offers
                    .iter()
                    .filter_map(|id| pack.schemes.iter().find(|s| &s.id == id))
                    .filter(|s| st.minute + s.minutes <= DAY_MINUTES)
                    .min_by_key(|s| (s.susp, s.id.clone()));
                if let Some(sc) = best {
                    return Command::Scheme { id: sc.id.clone() };
                }
            }
            if st.susp > 55 {
                if let Some(story) = st.stories.last() {
                    if st.boasts_today < 2 && !story.risky {
                        return Command::Boast { story: story.id };
                    }
                }
            }
            if st.queue.is_empty() {
                return Command::EndDay;
            }
            // dirt play
            for t in &st.queue {
                if let Some(inc) = pack.incident(&t.incident) {
                    if st.dirt.iter().any(|d| d.who == inc.who) && st.fucks < 70 {
                        return Command::UseDirt { ticket: t.id };
                    }
                }
            }
            // best-scoring option across the queue; susp matters more as it climbs
            let mut best = (f64::MIN, st.queue[0].id, 0u8);
            for t in &st.queue {
                let Some(inc) = pack.incident(&t.incident) else {
                    continue;
                };
                for (i, o) in inc.options.iter().enumerate() {
                    let wf = 1.0 + ((50 - st.fucks).max(0) as f64) / 50.0;
                    let ws = 0.5 + st.susp as f64 / 25.0;
                    let sc = score_effects(&o.effects, 1.0, wf, ws);
                    if sc > best.0 {
                        best = (sc, t.id, i as u8);
                    }
                }
            }
            Command::Resolve {
                ticket: best.1,
                option: best.2,
            }
        }
    }
}

/// Play a whole run; returns the engine at the end.
pub fn play(pack: std::sync::Arc<Pack>, seed: u64, strat: Strategy, max_steps: usize) -> Engine {
    let mut e = Engine::new(pack, seed);
    let mut rng = Xs(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    for _ in 0..max_steps {
        if e.is_over() {
            break;
        }
        let c = next_command(&e, strat, &mut rng);
        if e.apply(c).is_err() {
            let _ = e.apply(Command::EndDay);
        }
    }
    e
}
