//! Deterministic PRNG for renderers.
//!
//! SplitMix64: tiny, fast, fully deterministic across platforms and releases.
//! The stream depends only on the spec's `seed` — the same spec renders the
//! same artifact, byte for byte, on every machine. (Seeds are NOT compatible
//! with the retired Python implementation's Mersenne Twister stream.)

/// Small deterministic RNG (SplitMix64).
pub struct Rng {
    state: u64,
}

impl Rng {
    /// Create a generator from a non-negative integer seed.
    pub fn new(seed: u64) -> Self {
        Rng { state: seed }
    }

    /// Read the raw `seed` value out of a spec object (default 0).
    pub fn from_spec(spec: &serde_json::Value) -> Self {
        let seed = spec.get("seed").and_then(|v| v.as_u64()).unwrap_or(0);
        Rng::new(seed)
    }

    /// Next raw 64-bit value (SplitMix64 finalizer step).
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform integer in `0..n` (n > 0). Unbiased via rejection on the high bits.
    pub fn below(&mut self, n: usize) -> usize {
        debug_assert!(n > 0, "below() needs n > 0");
        let n = n as u64;
        let zone = u64::MAX - (u64::MAX % n);
        loop {
            let v = self.next_u64();
            if v < zone {
                return (v % n) as usize;
            }
        }
    }

    /// Uniform element of a non-empty slice.
    pub fn choice<'a, T>(&mut self, v: &'a [T]) -> &'a T {
        &v[self.below(v.len())]
    }

    /// Uniform f64 in `[lo, hi)` with 53-bit precision (`f64 / 2^53`).
    pub fn uniform(&mut self, lo: f64, hi: f64) -> f64 {
        let unit = (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64;
        lo + (hi - lo) * unit
    }

    /// Uniform f64 in `[0, 1)`.
    pub fn random(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_stream() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn below_in_range() {
        let mut r = Rng::new(7);
        for _ in 0..1000 {
            assert!(r.below(5) < 5);
        }
    }

    #[test]
    fn uniform_in_bounds() {
        let mut r = Rng::new(9);
        for _ in 0..1000 {
            let x = r.uniform(1.0, 999.0);
            assert!((1.0..999.0).contains(&x));
        }
    }
}
