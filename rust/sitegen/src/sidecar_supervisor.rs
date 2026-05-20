//! Spawn and manage the Node content sidecar as a child process.

use std::path::Path;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use tokio::process::{Child, Command};
use tracing::warn;

const READY_TIMEOUT: Duration = Duration::from_secs(15);
const POLL_INTERVAL: Duration = Duration::from_millis(100);

pub async fn spawn(repo_root: &Path, sidecar_script: &Path) -> Result<(Child, String)> {
    if !sidecar_script.exists() {
        anyhow::bail!("sidecar script missing: {sidecar_script:?}");
    }
    let port_file = repo_root.join(".content-sidecar-port");
    // Remove any stale port file so the readiness check doesn't latch a dead
    // sidecar from a previous run.
    let _ = std::fs::remove_file(&port_file);

    let child = Command::new("node")
        .arg(sidecar_script)
        .env("CONTENT_SIDECAR_PORT", "0")
        .env("CONTENT_SIDECAR_PORT_FILE", &port_file)
        .current_dir(repo_root)
        .kill_on_drop(true)
        .spawn()
        .with_context(|| "spawn node content-sidecar — is `node` on PATH?")?;

    let started = Instant::now();
    loop {
        if started.elapsed() > READY_TIMEOUT {
            anyhow::bail!(
                "content sidecar did not become ready in {:?}",
                READY_TIMEOUT
            );
        }
        if port_file.exists() {
            let port_text = std::fs::read_to_string(&port_file)?;
            let port: u16 = port_text
                .trim()
                .parse()
                .with_context(|| format!("parse sidecar port {port_text:?}"))?;
            if port == 0 {
                // file written but port not finalized yet
                tokio::time::sleep(POLL_INTERVAL).await;
                continue;
            }
            return Ok((child, format!("http://127.0.0.1:{port}")));
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

pub async fn shutdown(mut child: Child) {
    if let Some(pid) = child.id() {
        // SIGTERM first; the sidecar handles it gracefully.
        let _ = unsafe { libc_kill(pid as i32, SIGTERM) };
    }
    let kill = tokio::time::timeout(Duration::from_secs(3), child.wait()).await;
    if kill.is_err() {
        warn!("sidecar did not exit on SIGTERM; killing");
        let _ = child.kill().await;
    }
}

const SIGTERM: i32 = 15;
extern "C" {
    fn kill(pid: i32, sig: i32) -> i32;
}
unsafe fn libc_kill(pid: i32, sig: i32) -> i32 {
    kill(pid, sig)
}
