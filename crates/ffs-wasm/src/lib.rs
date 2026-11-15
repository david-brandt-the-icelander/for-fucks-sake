//! WASM facade: JSON in, JSON out. No rules here.

use std::sync::Arc;

use ffs_content::Pack;
use ffs_core::{Command, Engine};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    seed: u32,
    engine: Engine,
}

fn pack() -> Result<Arc<Pack>, JsError> {
    Pack::base()
        .map(Arc::new)
        .map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen]
impl Game {
    /// Start a new run with a seed.
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u32) -> Result<Game, JsError> {
        Ok(Game {
            seed,
            engine: Engine::new(pack()?, seed as u64),
        })
    }

    /// Restore a run from `save()` output. Fails if the replay hash does not match.
    pub fn load(save: &str) -> Result<Game, JsError> {
        let v: serde_json::Value =
            serde_json::from_str(save).map_err(|e| JsError::new(&e.to_string()))?;
        let seed = v["seed"]
            .as_u64()
            .ok_or_else(|| JsError::new("bad save: seed"))? as u32;
        let log: Vec<Command> =
            serde_json::from_value(v["log"].clone()).map_err(|e| JsError::new(&e.to_string()))?;
        let engine =
            Engine::replay(pack()?, seed as u64, &log).map_err(|e| JsError::new(&e.to_string()))?;
        if let Some(h) = v["hash"].as_str() {
            if h != engine.state_hash() {
                return Err(JsError::new("save is from a different version of the game"));
            }
        }
        Ok(Game { seed, engine })
    }

    /// Apply a command (JSON). Returns `{"ok":true,"events":[...]}` or `{"ok":false,"error":"..."}`.
    pub fn apply(&mut self, command_json: &str) -> String {
        let cmd: Command = match serde_json::from_str(command_json) {
            Ok(c) => c,
            Err(e) => return serde_json::json!({"ok": false, "error": e.to_string()}).to_string(),
        };
        match self.engine.apply(cmd) {
            Ok(ev) => serde_json::json!({"ok": true, "events": ev}).to_string(),
            Err(e) => serde_json::json!({"ok": false, "error": e.to_string()}).to_string(),
        }
    }

    /// The full view model as JSON.
    pub fn view(&self) -> String {
        serde_json::to_string(&self.engine.view()).unwrap_or_default()
    }

    /// Save game: seed + command log + state hash.
    pub fn save(&self) -> String {
        serde_json::json!({"seed": self.seed, "log": self.engine.log(), "hash": self.engine.state_hash()}).to_string()
    }

    pub fn seed(&self) -> u32 {
        self.seed
    }
}
