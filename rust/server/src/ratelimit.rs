//! Per-IP token-bucket rate limiter. Lock-free hot path via DashMap.
//! `allow(ip)` consumes one token; returns false if the caller is over
//! their budget. `gc()` evicts buckets idle longer than `idle_after` so
//! a wide spray of source IPs can't pin memory.

use std::net::IpAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use dashmap::DashMap;

pub struct RateLimiter {
    rate: f64,
    burst: f64,
    epoch: Instant,
    buckets: DashMap<IpAddr, Bucket>,
}

struct Bucket {
    // Tokens and timestamp packed as f64/u64 atomics so concurrent
    // requests from the same IP don't have to take a per-IP lock.
    tokens_micros: AtomicU64,
    last_ms: AtomicU64,
}

impl RateLimiter {
    pub fn new(rate: f64, burst: f64) -> Self {
        Self {
            rate,
            burst,
            epoch: Instant::now(),
            buckets: DashMap::new(),
        }
    }

    pub fn allow(&self, ip: IpAddr) -> bool {
        let now_ms = self.epoch.elapsed().as_millis() as u64;
        let entry = self.buckets.entry(ip).or_insert_with(|| Bucket {
            tokens_micros: AtomicU64::new((self.burst * 1_000_000.0) as u64),
            last_ms: AtomicU64::new(now_ms),
        });
        let last = entry.last_ms.swap(now_ms, Ordering::Relaxed);
        let elapsed_s = now_ms.saturating_sub(last) as f64 / 1000.0;
        let refill = elapsed_s * self.rate;
        let max_tokens_micros = (self.burst * 1_000_000.0) as u64;
        let cur = entry.tokens_micros.load(Ordering::Relaxed) as f64 / 1_000_000.0;
        let new_tokens = (cur + refill).min(self.burst);
        if new_tokens < 1.0 {
            entry
                .tokens_micros
                .store((new_tokens * 1_000_000.0) as u64, Ordering::Relaxed);
            return false;
        }
        let after = ((new_tokens - 1.0) * 1_000_000.0) as u64;
        entry
            .tokens_micros
            .store(after.min(max_tokens_micros), Ordering::Relaxed);
        true
    }

    /// Drop bucket entries that haven't been touched in `idle_after`.
    pub fn gc(&self, idle_after: Duration) {
        let cutoff_ms = self
            .epoch
            .elapsed()
            .saturating_sub(idle_after)
            .as_millis() as u64;
        self.buckets
            .retain(|_, b| b.last_ms.load(Ordering::Relaxed) > cutoff_ms);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_after_burst_exhausted() {
        let rl = RateLimiter::new(1.0, 5.0);
        let ip: IpAddr = "1.2.3.4".parse().unwrap();
        for _ in 0..5 {
            assert!(rl.allow(ip));
        }
        assert!(!rl.allow(ip));
    }

    #[test]
    fn independent_per_ip() {
        let rl = RateLimiter::new(1.0, 1.0);
        let a: IpAddr = "1.1.1.1".parse().unwrap();
        let b: IpAddr = "2.2.2.2".parse().unwrap();
        assert!(rl.allow(a));
        assert!(rl.allow(b));
        assert!(!rl.allow(a));
    }
}
