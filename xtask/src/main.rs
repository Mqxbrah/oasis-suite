use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() || args[0] == "help" {
        println!("Usage: cargo xtask <command> [options]");
        println!();
        println!("Commands:");
        println!("  bundle <plugin_name> [--release]    Build and bundle a plugin as .vst3");
        return Ok(());
    }

    match args[0].as_str() {
        "bundle" => {
            if args.len() < 2 {
                bail!("Usage: cargo xtask bundle <plugin_name> [--release]");
            }
            let plugin_name = &args[1];
            let release = args.iter().any(|a| a == "--release");
            bundle_plugin(plugin_name, release)?;
        }
        other => bail!("Unknown command: {other}"),
    }

    Ok(())
}

fn bundle_plugin(plugin_name: &str, release: bool) -> Result<()> {
    let workspace_root = workspace_root()?;

    println!("Building {plugin_name}...");
    let mut build_cmd = Command::new("cargo");
    build_cmd.arg("build").arg("-p").arg(plugin_name);
    if release {
        build_cmd.arg("--release");
    }
    build_cmd.current_dir(&workspace_root);

    let status = build_cmd.status().context("Failed to run cargo build")?;
    if !status.success() {
        bail!("cargo build failed for {plugin_name}");
    }

    let profile = if release { "release" } else { "debug" };
    let lib_name = format!("lib{}.dylib", plugin_name);
    let dylib_path = workspace_root.join("target").join(profile).join(&lib_name);

    if !dylib_path.exists() {
        bail!("Built library not found at {}", dylib_path.display());
    }

    let bundle_name = plugin_display_name(plugin_name);
    let bundle_dir = workspace_root
        .join("target")
        .join("bundled")
        .join(format!("{bundle_name}.vst3"))
        .join("Contents")
        .join("MacOS");

    std::fs::create_dir_all(&bundle_dir)
        .context("Failed to create VST3 bundle directory")?;

    let dest = bundle_dir.join(&bundle_name);
    std::fs::copy(&dylib_path, &dest)
        .with_context(|| format!("Failed to copy dylib to {}", dest.display()))?;

    // Create Info.plist
    let plist_dir = bundle_dir.parent().unwrap();
    let plist_path = plist_dir.join("Info.plist");
    let plist_content = generate_plist(&bundle_name, plugin_name);
    std::fs::write(&plist_path, plist_content)
        .context("Failed to write Info.plist")?;

    let vst3_path = workspace_root
        .join("target")
        .join("bundled")
        .join(format!("{bundle_name}.vst3"));

    println!("Bundled: {}", vst3_path.display());
    Ok(())
}

fn plugin_display_name(crate_name: &str) -> String {
    match crate_name {
        "oasis_wide" => "Oasis Wide".to_string(),
        "oasis_eq" => "Oasis EQ".to_string(),
        "oasis_comp" => "Oasis Comp".to_string(),
        "oasis_verb" => "Oasis Verb".to_string(),
        "oasis_delay" => "Oasis Delay".to_string(),
        "oasis_drive" => "Oasis Drive".to_string(),
        "oasis_limit" => "Oasis Limit".to_string(),
        "oasis_punch" => "Oasis Punch".to_string(),
        "oasis_deess" => "Oasis DeEss".to_string(),
        "oasis_shift" => "Oasis Shift".to_string(),
        "oasis_pump" => "Oasis Pump".to_string(),
        "oasis_synth" => "Oasis Synth".to_string(),
        other => other.replace('_', " "),
    }
}

fn workspace_root() -> Result<PathBuf> {
    let output = Command::new("cargo")
        .args(["locate-project", "--workspace", "--message-format", "plain"])
        .output()
        .context("Failed to locate workspace root")?;

    let path_str = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in cargo locate-project output")?;
    let cargo_toml = Path::new(path_str.trim());

    Ok(cargo_toml
        .parent()
        .expect("Cargo.toml should have a parent directory")
        .to_path_buf())
}

fn generate_plist(bundle_name: &str, crate_name: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>
    <key>CFBundleExecutable</key>
    <string>{bundle_name}</string>
    <key>CFBundleGetInfoString</key>
    <string>{bundle_name} 1.0.0</string>
    <key>CFBundleIdentifier</key>
    <string>com.oasis-suite.{crate_name}</string>
    <key>CFBundleName</key>
    <string>{bundle_name}</string>
    <key>CFBundlePackageType</key>
    <string>BNDL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
</dict>
</plist>"#
    )
}
