//! The RPC smoke test.
//!
//! It replaces `ops/scripts/smoke_rpc.sh`. It starts a short-lived node and
//! probes the JSON-RPC method `bud_chainId` on the operator listener. That
//! listener is used on purpose: the smoke test then does not depend on the
//! public-RPC auth policy or the allow-list. It validates node boot, not
//! public exposure.
//!
//! Environment, all optional: `SMOKE_NETWORK` (default `devnet`),
//! `SMOKE_RPC_PORT` (default 18546, the operator listener),
//! `SMOKE_PUBLIC_RPC_PORT` (default 18545), `SMOKE_DB_PATH`, `SMOKE_BIN`,
//! `RUST_LOG` (default `warn`) and `BUDLUM_RPC_AUTH_REQUIRED` (default `0`).
//! An explicit `SMOKE_DB_PATH` must not exist. Existing data is never removed.

use std::net::{Ipv4Addr, TcpListener};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use crate::support::{make_temp_dir, tail_lines};
use crate::{has_program, json, rpc};

const TRIES: u32 = 60;
const POLL_PAUSE: Duration = Duration::from_millis(500);
const REQUEST_TIME: Duration = Duration::from_secs(5);

/// The settings of one run.
#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub network: String,
    pub rpc_port: u16,
    pub public_port: u16,
    pub db_path: Option<PathBuf>,
    pub bin: Option<PathBuf>,
}

impl Config {
    /// Read the settings from the environment.
    ///
    /// # Errors
    ///
    /// If a port is not a number.
    pub fn from_env() -> Result<Self, String> {
        let get = |name: &str| std::env::var(name).ok().filter(|v| !v.is_empty());
        let port = |name: &str, default: u16| -> Result<u16, String> {
            get(name).map_or(Ok(default), |v| {
                v.parse()
                    .map_err(|_| format!("{name} is not a port: {v:?}"))
            })
        };
        let rpc_port = port("SMOKE_RPC_PORT", 18546)?;
        let public_port = port("SMOKE_PUBLIC_RPC_PORT", 18545)?;
        Ok(Self {
            network: get("SMOKE_NETWORK").unwrap_or_else(|| "devnet".to_string()),
            rpc_port,
            public_port,
            db_path: get("SMOKE_DB_PATH").map(PathBuf::from),
            bin: get("SMOKE_BIN").map(PathBuf::from),
        })
    }
}

/// The two listeners must never share a port, or the second bind fails with
/// `EADDRINUSE` and the probe loop times out. Returns the operator port to
/// use, and a note when it had to move.
///
/// # Errors
///
/// If the shifted port would not fit in a `u16`.
pub fn operator_port(rpc_port: u16, public_port: u16) -> Result<(u16, Option<String>), String> {
    if rpc_port != public_port {
        return Ok((rpc_port, None));
    }
    let shifted = public_port
        .checked_add(1)
        .ok_or_else(|| format!("no port above the public port {public_port}"))?;
    Ok((
        shifted,
        Some(format!(
            "operator port {rpc_port} equals public port {public_port}; shifting operator to {shifted}"
        )),
    ))
}

/// The node command line.
#[must_use]
pub fn node_args(cfg: &Config, operator: u16, db: &Path) -> Vec<String> {
    vec![
        "--network".to_string(),
        cfg.network.clone(),
        "--port".to_string(),
        "0".to_string(),
        "--rpc-public-listener".to_string(),
        format!("127.0.0.1:{}", cfg.public_port),
        "--rpc-operator-listener".to_string(),
        format!("127.0.0.1:{operator}"),
        "--db-path".to_string(),
        db.join("chain").display().to_string(),
        "--snapshot-dir".to_string(),
        db.join("snapshots").display().to_string(),
        "--p2p-identity-file".to_string(),
        db.join("secrets/node-id.key").display().to_string(),
    ]
}

/// Read the chain id from a successful response to this probe, not from a
/// nested member or an error message that happens to mention `result`.
fn reply_chain_id(body: &str) -> Option<u64> {
    let value = json::parse(body).ok()?;
    if value.get("jsonrpc").and_then(json::Value::as_str) != Some("2.0")
        || value.get("id") != Some(&json::Value::Number("1".to_string()))
        || value.get("error").is_some()
    {
        return None;
    }
    rpc::parse_hex_quantity(value.get("result")?.as_str()?)
}

fn expected_chain_id(network: &str) -> Result<u64, String> {
    // Keep in sync with Network::chain_id in src/core/chain_config.rs.
    match network {
        "mainnet" => Ok(45260),
        "testnet" => Ok(45261),
        "devnet" => Ok(45262),
        _ => Err(format!("unknown smoke network: {network}")),
    }
}

fn require_free_port(port: u16) -> Result<(), String> {
    if port == 0 {
        return Err("the smoke RPC ports must be nonzero".to_string());
    }
    TcpListener::bind((Ipv4Addr::LOCALHOST, port))
        .map(drop)
        .map_err(|e| format!("smoke RPC port {port} is not free: {e}"))
}

/// Kills and reaps the node when the run ends, however it ends.
struct NodeGuard(Child);

impl Drop for NodeGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// Run the smoke test.
///
/// # Errors
///
/// If no binary can be found or built, the node cannot start or exits early,
/// the RPC never answers, or the reply does not identify the selected network.
pub fn run(root: &Path, cfg: &Config) -> Result<String, String> {
    let (operator, note) = operator_port(cfg.rpc_port, cfg.public_port)?;
    if let Some(note) = note {
        eprintln!("[smoke] {note}");
    }
    let expected = expected_chain_id(&cfg.network)?;
    // A pre-existing listener must not answer on behalf of our child.
    require_free_port(operator)?;
    require_free_port(cfg.public_port)?;
    let bin = match &cfg.bin {
        Some(b) => b.clone(),
        None => find_binary(root)?,
    };

    let db = prepare_db(cfg.db_path.as_deref())?;
    std::fs::create_dir_all(db.join("secrets"))
        .map_err(|e| format!("{} could not be created: {e}", db.display()))?;
    let log_path = db.join("node.log");
    let log = std::fs::File::create(&log_path)
        .map_err(|e| format!("{} could not be created: {e}", log_path.display()))?;
    let log_err = log
        .try_clone()
        .map_err(|e| format!("could not share the log file: {e}"))?;

    let args = node_args(cfg, operator, &db);
    println!("[smoke] starting {} {}", bin.display(), args.join(" "));
    let env_or = |name: &str, default: &str| {
        std::env::var(name)
            .ok()
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| default.to_string())
    };
    let rust_log = env_or("RUST_LOG", "warn");
    let auth = env_or("BUDLUM_RPC_AUTH_REQUIRED", "0");
    let child = Command::new(&bin)
        .args(&args)
        .env("RUST_LOG", rust_log)
        .env("BUDLUM_RPC_AUTH_REQUIRED", auth)
        .stdout(Stdio::from(log))
        .stderr(Stdio::from(log_err))
        .spawn()
        .map_err(|e| format!("{} could not be started: {e}", bin.display()))?;
    let mut guard = NodeGuard(child);

    let mut body = None;
    for attempt in 1..=TRIES {
        let reply = rpc::call(operator, "bud_chainId", "[]", REQUEST_TIME);
        if guard.0.try_wait().map_err(|e| e.to_string())?.is_some() {
            return Err(format!("node exited early; log:\n{}", log_tail(&log_path)));
        }
        if let Ok(reply) = reply {
            body = Some(reply);
            break;
        }
        std::thread::sleep(POLL_PAUSE);
        if attempt == TRIES {
            return Err(format!(
                "timeout waiting for RPC; log:\n{}",
                log_tail(&log_path)
            ));
        }
    }
    let body = body.ok_or_else(|| "no RPC reply".to_string())?;

    println!("[smoke] RPC response: {body}");
    if reply_chain_id(&body) != Some(expected) {
        return Err(format!(
            "expected a successful chain id reply for {expected}"
        ));
    }
    Ok(format!(
        "[smoke] OK - bud_chainId responded on {}",
        cfg.network
    ))
}

fn log_tail(path: &Path) -> String {
    std::fs::read_to_string(path).map_or_else(|_| String::new(), |t| tail_lines(&t, 80))
}

/// Never reset or adopt an existing path. An explicit path must be new;
/// the default is already created exclusively by `make_temp_dir`.
fn prepare_db(requested: Option<&Path>) -> Result<PathBuf, String> {
    let Some(path) = requested else {
        return make_temp_dir("budlum-smoke-db");
    };
    std::fs::create_dir(path)
        .map_err(|e| format!("refusing to reuse smoke directory {}: {e}", path.display()))?;
    Ok(path.to_path_buf())
}

fn is_executable(path: &Path) -> bool {
    std::fs::metadata(path).is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
}

fn docker_image_exists(image: &str) -> bool {
    Command::new("docker")
        .args(["image", "inspect", image])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

/// Pick the node binary: the debug build, the release build, the one inside
/// the `budlum-core:devnet` image, or a fresh debug build, in that order.
fn find_binary(root: &Path) -> Result<PathBuf, String> {
    let debug = root.join("target/debug/budlum-core");
    let release = root.join("target/release/budlum-core");
    if is_executable(&debug) {
        return Ok(debug);
    }
    if is_executable(&release) {
        return Ok(release);
    }
    if docker_image_exists("budlum-core:devnet") {
        // The docker-smoke workflow builds budlum-core:devnet with compose build.
        println!("[smoke] extracting binary from Docker image...");
        return extract_from_image(root);
    }
    if has_program("cargo") {
        println!("[smoke] building budlum-core (debug)...");
        crate::run_checked("cargo", &["build", "-q", "--bin", "budlum-core"], root)?;
        return Ok(debug);
    }
    Err("no budlum-core binary and no cargo".to_string())
}

fn extract_from_image(root: &Path) -> Result<PathBuf, String> {
    let out = Command::new("docker")
        .args(["create", "budlum-core:devnet"])
        .current_dir(root)
        .output()
        .map_err(|e| format!("`docker create` could not be run: {e}"))?;
    if !out.status.success() {
        return Err("`docker create budlum-core:devnet` failed".to_string());
    }
    let id = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let dir = make_temp_dir("budlum-smoke-bin")?;
    let bin = dir.join("budlum-core");
    let source = format!("{id}:/usr/local/bin/budlum-core");
    let copied = crate::run_checked("docker", &["cp", &source, &bin.display().to_string()], root);
    let _ = Command::new("docker")
        .args(["rm", &id])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    copied?;
    std::fs::set_permissions(&bin, std::fs::Permissions::from_mode(0o755))
        .map_err(|e| format!("{} could not be made executable: {e}", bin.display()))?;
    Ok(bin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_clash_of_ports_shifts_the_operator() {
        assert_eq!(operator_port(18546, 18545).expect("ok"), (18546, None));
        let (port, note) = operator_port(18545, 18545).expect("ok");
        assert_eq!(port, 18546);
        assert!(note.expect("a note").contains("shifting operator to 18546"));
        assert!(operator_port(65535, 65535).is_err());
    }

    #[test]
    fn the_node_command_line_has_every_flag() {
        let cfg = Config {
            network: "devnet".to_string(),
            rpc_port: 18546,
            public_port: 18545,
            db_path: None,
            bin: None,
        };
        let args = node_args(&cfg, 18546, Path::new("/tmp/db"));
        let line = args.join(" ");
        assert!(line.starts_with("--network devnet --port 0 "));
        assert!(line.contains("--rpc-public-listener 127.0.0.1:18545"));
        assert!(line.contains("--rpc-operator-listener 127.0.0.1:18546"));
        assert!(line.contains("--db-path /tmp/db/chain"));
        assert!(line.contains("--snapshot-dir /tmp/db/snapshots"));
        assert!(line.contains("--p2p-identity-file /tmp/db/secrets/node-id.key"));
    }

    #[test]
    fn a_reply_must_match_the_probe_and_have_no_error() {
        assert_eq!(
            reply_chain_id(r#"{"jsonrpc":"2.0","result":"0xb0ce","id":1}"#),
            Some(45262)
        );
        for body in [
            r#"{"jsonrpc":"2.0","error":{"code":-32601,"result":"0xb0ce"},"id":1}"#,
            r#"{"jsonrpc":"2.0","result":"0xb0ce","error":null,"id":1}"#,
            r#"{"jsonrpc":"2.0","result":null,"id":1}"#,
            r#"{"jsonrpc":"2.0","result":"0xb0ce","id":2}"#,
            r#"{"result":"0xb0ce","id":1}"#,
            "not JSON with \"result\" in it",
        ] {
            assert_eq!(reply_chain_id(body), None, "{body}");
        }
        assert_ne!(
            reply_chain_id(r#"{"jsonrpc":"2.0","result":"0x1","id":1}"#),
            expected_chain_id("devnet").ok()
        );
        assert!(expected_chain_id("unknown").is_err());
    }

    #[test]
    fn existing_listener_is_refused() {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).expect("listener");
        let port = listener.local_addr().expect("address").port();
        assert!(require_free_port(port).is_err());
        assert!(require_free_port(0).is_err());
    }

    #[test]
    fn existing_directory_is_not_deleted_or_adopted() {
        let dir = make_temp_dir("budlum-smoke-test").expect("dir");
        let sentinel = dir.join("keep");
        std::fs::write(&sentinel, b"preserve").expect("file");
        for path in [Path::new(""), Path::new("/"), Path::new("."), &dir] {
            assert!(prepare_db(Some(path)).is_err());
        }
        assert_eq!(std::fs::read(&sentinel).expect("sentinel"), b"preserve");
        let link = dir.join("link");
        std::os::unix::fs::symlink(&dir, &link).expect("symlink");
        assert!(prepare_db(Some(&link)).is_err());
        assert_eq!(std::fs::read(&sentinel).expect("sentinel"), b"preserve");
        std::fs::remove_dir_all(dir).expect("cleanup");
    }

    #[test]
    fn new_directory_is_created_exclusively() {
        let parent = make_temp_dir("budlum-smoke-test").expect("parent");
        let requested = parent.join("new-db");
        assert_eq!(prepare_db(Some(&requested)).expect("new db"), requested);
        assert!(prepare_db(Some(&requested)).is_err());
        let automatic = prepare_db(None).expect("automatic db");
        assert!(automatic.is_dir());
        std::fs::remove_dir_all(automatic).expect("cleanup");
        std::fs::remove_dir_all(parent).expect("cleanup");
    }
}
