use std::time::{Duration, Instant};

use keepass::config::KdfConfig;

use crate::{CoreError, Result};

/// Argon2id parameters stored in the KDBX4 header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KdfParams {
    pub iterations: u64,
    pub memory_bytes: u64,
    pub parallelism: u32,
}

impl KdfParams {
    pub const DEFAULT_MEMORY_BYTES: u64 = 64 * 1024 * 1024;
    pub const DEFAULT_UNLOCK_TIME: Duration = Duration::from_secs(1);

    const MIN_ITERATIONS: u64 = 2;
    const MAX_ITERATIONS: u64 = 1000;
    const PROBE_ITERATIONS: u64 = 2;

    /// Picks the iteration count that makes unlocking take about `target` on
    /// this machine, using 64 MiB of memory. Blocks for roughly a fraction of
    /// a second, so call it off the UI thread.
    pub fn calibrate(target: Duration) -> Result<Self> {
        let probe = Self {
            iterations: Self::PROBE_ITERATIONS,
            memory_bytes: Self::DEFAULT_MEMORY_BYTES,
            parallelism: default_parallelism(),
        };

        let started = Instant::now();
        probe.derive_probe()?;
        let per_iteration = started.elapsed().as_secs_f64() / Self::PROBE_ITERATIONS as f64;

        let iterations = (target.as_secs_f64() / per_iteration.max(f64::EPSILON)).round() as u64;
        Ok(Self {
            iterations: iterations.clamp(Self::MIN_ITERATIONS, Self::MAX_ITERATIONS),
            ..probe
        })
    }

    /// Cheap parameters for tests. Never use them for a real vault.
    #[doc(hidden)]
    pub fn insecure_fast() -> Self {
        Self {
            iterations: 1,
            memory_bytes: 1024 * 1024,
            parallelism: 1,
        }
    }

    pub(crate) fn to_config(self) -> KdfConfig {
        KdfConfig::Argon2id {
            iterations: self.iterations,
            memory: self.memory_bytes,
            parallelism: self.parallelism,
            version: argon2::Version::Version13,
        }
    }

    pub(crate) fn from_config(config: &KdfConfig) -> Option<Self> {
        match *config {
            KdfConfig::Argon2id {
                iterations,
                memory,
                parallelism,
                ..
            } => Some(Self {
                iterations,
                memory_bytes: memory,
                parallelism,
            }),
            _ => None,
        }
    }

    /// Runs Argon2id with the same settings `keepass` uses on unlock.
    fn derive_probe(&self) -> Result<()> {
        let config = argon2::Config {
            ad: &[],
            hash_length: 32,
            lanes: self.parallelism,
            mem_cost: (self.memory_bytes / 1024) as u32,
            secret: &[],
            thread_mode: argon2::ThreadMode::Parallel,
            time_cost: self.iterations as u32,
            variant: argon2::Variant::Argon2id,
            version: argon2::Version::Version13,
        };
        argon2::hash_raw(&[0u8; 32], &[0u8; 32], &config)
            .map(drop)
            .map_err(|err| CoreError::Kdf(err.to_string()))
    }
}

fn default_parallelism() -> u32 {
    std::thread::available_parallelism()
        .map(|n| n.get().min(4) as u32)
        .unwrap_or(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_round_trip() {
        let params = KdfParams::insecure_fast();
        assert_eq!(KdfParams::from_config(&params.to_config()), Some(params));
    }

    #[test]
    fn calibrate_respects_bounds() {
        let params = KdfParams::calibrate(Duration::from_millis(1)).unwrap();
        assert_eq!(params.iterations, KdfParams::MIN_ITERATIONS);
        assert_eq!(params.memory_bytes, KdfParams::DEFAULT_MEMORY_BYTES);
    }
}
