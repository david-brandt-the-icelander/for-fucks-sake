use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Player commands. The log of these plus the seed is the whole save game.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "type")]
#[ts(export)]
pub enum Command {
    /// Resolve a ticket with option index (0-based).
    Resolve {
        ticket: u32,
        option: u8,
    },
    /// Spend a piece of dirt on the ticket's requester.
    UseDirt {
        ticket: u32,
    },
    /// Write the unsent email. `send` = actually press send. (Text is never part of the log.)
    Rant {
        send: bool,
    },
    Coffee,
    /// Cover your tracks: tidy logs and paperwork to bring Suspicion down.
    CoverTracks,
    Scheme {
        id: String,
    },
    Boast {
        story: u32,
    },
    /// F1 was opened. Costs a nerd point.
    HelpOpened,
    /// Go home early.
    EndDay,
    /// F10, confirmed.
    Retire,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(tag = "type")]
#[ts(export)]
pub enum Event {
    Toast { text: String },
    DayEnded { day: u32, quip: String },
    RunEnded { kind: EndKind },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum EndKind {
    Meltdown,
    Fired,
    Retired,
    Ran,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: u32,
    pub incident: String,
    pub age: u32,
    pub nth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub id: u32,
    pub text: String,
    pub value: u8,
    pub risky: bool,
    pub day: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RivalState {
    pub id: String,
    pub np: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ChatLine {
    pub who: String,
    pub text: String,
    /// "rival" | "you" | "system"
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NpEntry {
    pub day: u32,
    pub delta: i32,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holding {
    pub who: String,
    pub label: String,
    pub use_line: String,
    pub idx: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ended {
    pub kind: EndKind,
    pub ending: String,
    pub cause: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub seed: u64,
    pub rng_n: u64,
    pub step: u32,
    pub day: u32,
    pub minute: u32,
    pub fucks: i32,
    pub susp: i32,
    pub cash: i32,
    pub np: i32,
    pub queue: Vec<Ticket>,
    pub next_ticket: u32,
    pub used: BTreeSet<String>,
    pub chain_count: BTreeMap<String, u32>,
    pub dirt: Vec<Holding>,
    /// Dirt-pool indices already handed out (so each item appears once).
    pub dirt_given: BTreeSet<usize>,
    pub stories: Vec<Story>,
    pub next_story: u32,
    pub rivals: Vec<RivalState>,
    pub chat: Vec<ChatLine>,
    pub np_log: Vec<NpEntry>,
    pub fucks_history: Vec<i32>,
    pub schemes_today: BTreeSet<String>,
    pub rants_today: u32,
    pub coffees_today: u32,
    pub covers_today: u32,
    pub boasts_today: u32,
    pub resolved: u32,
    pub toast: String,
    pub last_cause: String,
    pub ended: Option<Ended>,
}
