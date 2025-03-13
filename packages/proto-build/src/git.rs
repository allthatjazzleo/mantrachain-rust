use log::{info, warn};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process;
use std::fs;
use std::path::Path;
use regex::Regex;

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<(), String> {
    let stdout = process::Stdio::inherit();

    let exit_status = process::Command::new("git")
        .args(args)
        .stdout(stdout)
        .status()
        .expect("git exit status missing");

    if !exit_status.success() {
        return Err(format!(
            "git exited with error code: {:?}",
            exit_status.code()
        ));
    }

    Ok(())
}

pub fn update_submodule(dir: &str, rev: &str) {
    let full_path = |p: &str| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(p);

    info!("Updating {} submodule...", dir);

    // switch to the given revision
    run_git(["-C", full_path(dir).to_str().unwrap(), "checkout", rev]).expect("failed to checkout");

    // pull the latest changes
    match run_git(["-C", full_path(dir).to_str().unwrap(), "pull"]) {
        Ok(_) => info!("Updated {} submodule to revision {}", dir, rev),
        Err(_) => warn!(
            "Failed to update {} with revision {}. This might be caused by revision is a tag",
            dir, rev
        ),
    };

    // run_git(["submodule", "update", "--init"]).expect("failed to update submodules");
}

pub fn get_cosmos_sdk_version_from_go_mod(project_dir: &str) -> Result<String, String> {
    let go_mod_path = Path::new(project_dir).join("go.mod");
    let content = fs::read_to_string(go_mod_path)
        .expect("Failed to read go.mod file");

    // Regular expression to match the cosmos-sdk version in replace directive
    let re = Regex::new(r#"github.com/cosmos/cosmos-sdk\s+=>\s+github.com/MANTRA-Chain/cosmos-sdk\s+([^\s]+)"#)
        .map_err(|_| "Failed to create regex for cosmos-sdk version".to_string())?;
    
    if let Some(captures) = re.captures(&content) {
        captures.get(1)
            .map(|m| m.as_str().to_string())
            .ok_or("Failed to extract cosmos-sdk version from go.mod".to_string())
    } else {
        Err("Failed to extract cosmos-sdk version from go.mod".to_string())
    }
}
