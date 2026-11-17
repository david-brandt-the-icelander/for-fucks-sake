//! FFS! game engine. All rules live here. No IO, no clock, deterministic.
//!
//! A run is a pure function of `(seed, command log)`. Hosts (browser via WASM, CLI) only
//! send [`Command`]s and render the [`View`].

mod engine;
mod rng;
mod state;
mod view;

pub use engine::{CommandError, Engine};
pub use state::{Command, EndKind, Event, State};
pub use view::*;

/// Game constants (MVP balance lives here; tune with `ffs bot`).
pub mod consts {
    pub const DAY_MINUTES: u32 = 480;
    pub const START_HOUR: u32 = 9;
    pub const START_FUCKS: i32 = 100;
    pub const MAX_FUCKS: i32 = 100;
    pub const EROSION_PER_DAY: i32 = 3;
    pub const SUSP_DECAY_PER_DAY: i32 = 8;
    pub const RETIRE_GOAL: i32 = 25_000;
    pub const RETIRE_MAX_SUSP: i32 = 60;
    pub const MAX_QUEUE: usize = 6;
    pub const INCIDENT_MINUTES: u32 = 120;
    pub const RANT_MINUTES: u32 = 60;
    pub const COFFEE_MINUTES: u32 = 60;
    pub const BOAST_MINUTES: u32 = 60;
    pub const DIRT_MINUTES: u32 = 60;
    pub const COVER_MINUTES: u32 = 120;
    /// Suspicion removed by the 1st, 2nd and later "cover your tracks" of a day.
    pub const COVER_SUSP: [i32; 3] = [6, 3, 1];
    /// Fucks it costs (shredding paperwork is tedious).
    pub const COVER_FUCKS: i32 = 4;
    /// How many schemes are on offer at once.
    pub const SCHEME_SLOTS: usize = 3;
    pub const START_CASH: i32 = 1_200;
}
