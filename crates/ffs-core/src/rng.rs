use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};

/// A fresh ChaCha stream per decision: seeded from (run seed, decision counter, label).
pub fn stream(seed: u64, counter: u64, label: &str) -> ChaCha8Rng {
    let mut h = blake3::Hasher::new();
    h.update(&seed.to_le_bytes());
    h.update(&counter.to_le_bytes());
    h.update(label.as_bytes());
    let out = h.finalize();
    ChaCha8Rng::from_seed(*out.as_bytes())
}

/// Uniform integer in `lo..=hi` without float maths.
pub fn range(r: &mut ChaCha8Rng, lo: i64, hi: i64) -> i64 {
    if hi <= lo {
        return lo;
    }
    let span = (hi - lo + 1) as u64;
    lo + (r.next_u64() % span) as i64
}
