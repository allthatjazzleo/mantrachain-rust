//! Build Mantrachain proto files. This build script clones the CosmosSDK and Mantrachain version
//! specified in the COSMOS_SDK_REV and MANTRACHAIN_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{env, path::PathBuf};

use proto_build::{
    code_generator::{CodeGenerator, CosmosProject},
    git,
};

/// The mantrachain commit or tag to be cloned and used to build the proto files
const MANTRACHAIN_REV: &str = "v3.0.0";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const OUT_DIR: &str = "../mantrachain-std/src/types/";
/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../../dependencies/cosmos-sdk/";
/// Directory where the mantrachain submodule is located
const MANTRACHAIN_DIR: &str = "../../dependencies/mantrachain/";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

pub fn generate() {
    let args: Vec<String> = env::args().collect();
    
    // Extract Cosmos SDK version from go.mod
    let cosmos_sdk_rev = git::get_cosmos_sdk_version_from_go_mod(MANTRACHAIN_DIR).unwrap();
    
    if args.iter().any(|arg| arg == "--update-deps") {
        git::update_submodule(COSMOS_SDK_DIR, &cosmos_sdk_rev);
        git::update_submodule(MANTRACHAIN_DIR, MANTRACHAIN_REV);
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let mantrachain_project = CosmosProject {
        name: "mantrachain".to_string(),
        version: MANTRACHAIN_REV.to_string(),
        project_dir: MANTRACHAIN_DIR.to_string(),
        exclude_mods: vec![],
    };

    let cosmos_project = CosmosProject {
        name: "cosmos".to_string(),
        version: cosmos_sdk_rev,
        project_dir: COSMOS_SDK_DIR.to_string(),
        exclude_mods: vec!["reflection".to_string(), "autocli".to_string()],
    };

    let mantrachain_code_generator = CodeGenerator::new(
        out_dir,
        tmp_build_dir,
        mantrachain_project,
        vec![
            cosmos_project,
        ],
    );

    mantrachain_code_generator.generate();
}

fn main() {
    pretty_env_logger::init();
    generate();
}
