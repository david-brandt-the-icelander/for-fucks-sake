use std::sync::Arc;

use ffs_content::Pack;
use ffs_core::{Command, Engine};

/// Every recorded run in tests/replays/ must reproduce its state hash.
/// Regenerate on purpose with `make replays` whenever rules or content change.
#[test]
fn replay_corpus_verifies() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/replays");
    let pack = Arc::new(Pack::base().expect("pack"));
    let mut n = 0;
    for entry in std::fs::read_dir(dir).expect("replays dir") {
        let path = entry.expect("entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let v: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&path).expect("read")).expect("json");
        let seed = v["seed"].as_u64().expect("seed");
        let log: Vec<Command> = serde_json::from_value(v["log"].clone()).expect("log");
        let e = Engine::replay(pack.clone(), seed, &log).expect("replay");
        assert_eq!(
            v["hash"].as_str().expect("hash"),
            e.state_hash(),
            "{}",
            path.display()
        );
        n += 1;
    }
    assert!(n >= 10, "replay corpus too small: {n}");
}
