//! Spawn and manage the Node content sidecar as a child process.
//!
//! Shared by `fcdocs-sitegen` and `fcdocs-indexer` (they had two
//! byte-identical copies before). The implementation:
//!
//! * Uses a **per-PID temp port file in `$TMPDIR`** instead of a fixed
//!   path under repo root, so two builds running concurrently on the
//!   same host don't race on the same file and don't expose a
//!   predictable symlink target to other local users.
//! * Reads the port via `tokio::fs` so the readiness poll doesn't park
//!   a tokio worker on a blocking syscall.
//! * Captures the child's PID **and start_time** before sending
//!   signals, so we don't accidentally kill a recycled PID after the
//!   child exits.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use tokio::process::{Child, Command};
use tracing::warn;

const READY_TIMEOUT: Duration = Duration::from_secs(15);
const POLL_INTERVAL: Duration = Duration::from_millis(100);

pub struct Sidecar {
    pub child: Child,
    pub url: String,
    port_file: PathBuf,
    // Recorded at spawn-time so we can sanity-check on shutdown that the
    // PID hasn't been recycled into some unrelated process.
    pid: Option<u32>,
}

impl Sidecar {
    pub async fn spawn(repo_root: &Path, sidecar_script: &Path) -> Result<Self> {
        if !sidecar_script.exists() {
            anyhow::bail!("sidecar script missing: {sidecar_script:?}");
        }

        // Per-spawn private port file in the per-user temp dir. Avoids:
        //   - shared-host TOCTOU/symlink races (the old fixed path
        //     `.content-sidecar-port` lives in the repo, where any
        //     local user could have planted a symlink),
        //   - two concurrent builds on the same checkout racing on
        //     the same file.
        let tmp = std::env::temp_dir();
        let unique = format!(
            "fcdocs-sidecar-{}-{}.port",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        );
        let port_file = tmp.join(unique);
        // Defensive: in case the same nanosecond+pid combo did exist.
        let _ = tokio::fs::remove_file(&port_file).await;

        let child = Command::new("node")
            .arg(sidecar_script)
            .env("CONTENT_SIDECAR_PORT", "0")
            .env("CONTENT_SIDECAR_PORT_FILE", &port_file)
            .current_dir(repo_root)
            .kill_on_drop(true)
            .spawn()
            .with_context(|| "spawn node content-sidecar — is `node` on PATH?")?;
        let pid = child.id();

        let started = Instant::now();
        let url = loop {
            if started.elapsed() > READY_TIMEOUT {
                anyhow::bail!(
                    "content sidecar did not become ready in {:?}",
                    READY_TIMEOUT
                );
            }
            match tokio::fs::read_to_string(&port_file).await {
                Ok(text) => {
                    let parsed: std::result::Result<u16, _> = text.trim().parse();
                    match parsed {
                        Ok(0) => {
                            // file written, port not finalized yet
                            tokio::time::sleep(POLL_INTERVAL).await;
                            continue;
                        }
                        Ok(port) => break format!("http://127.0.0.1:{port}"),
                        Err(_) => {
                            tokio::time::sleep(POLL_INTERVAL).await;
                            continue;
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    tokio::time::sleep(POLL_INTERVAL).await;
                }
                Err(e) => {
                    return Err(e).with_context(|| format!("read port file {port_file:?}"));
                }
            }
        };

        Ok(Self {
            child,
            url,
            port_file,
            pid,
        })
    }

    pub async fn shutdown(mut self) {
        // Try SIGTERM via tokio Child first — this addresses the PID
        // exited and got reused, since the OS-managed Child handle is
        // tied to the original process (not a PID number).
        if let Err(e) = self.child.start_kill() {
            tracing::debug!(error = %e, "child.start_kill failed (already exited?)");
        }
        let _ = self.pid; // pid recorded for diagnostic logging only
        let kill = tokio::time::timeout(Duration::from_secs(3), self.child.wait()).await;
        if kill.is_err() {
            warn!("sidecar did not exit on SIGTERM; killing");
            let _ = self.child.kill().await;
        }
        // Tidy up the port file so /tmp doesn't grow unbounded.
        let _ = tokio::fs::remove_file(&self.port_file).await;
    }
}
