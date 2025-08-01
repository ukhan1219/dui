use std::fs;

fn main() {
    // Tell Cargo to rerun this script if Cargo.toml changes
    println!("cargo:rerun-if-changed=Cargo.toml");
    
    // Read the version from Cargo.toml
    let cargo_toml = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
    let version = extract_version(&cargo_toml);
    
    // Make the version available to the main code
    println!("cargo:rustc-env=CARGO_PKG_VERSION={}", version);
}

fn extract_version(cargo_toml: &str) -> String {
    for line in cargo_toml.lines() {
        if line.trim().starts_with("version = ") {
            // Extract version from "version = "1.0.0""
            let version = line.split('=').nth(1)
                .and_then(|s| s.trim().strip_prefix('"'))
                .and_then(|s| s.strip_suffix('"'))
                .unwrap_or("0.0.0");
            return version.to_string();
        }
    }
    "0.0.0".to_string()
} 