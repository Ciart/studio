use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

const APP_NAME: &str = "Ciart Studio";
const BUNDLE_ID: &str = "dev.local.ciartstudio";
const ICON_NAME: &str = "AppIcon";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/{ICON_NAME}.icon");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("macos") {
        return;
    }

    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let profile_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("OUT_DIR should live under target/<profile>/build/<pkg>/out")
        .to_path_buf();

    let contents = profile_dir.join(format!("{APP_NAME}.app/Contents"));
    let macos = contents.join("MacOS");
    let resources = contents.join("Resources");
    if let Err(err) = fs::create_dir_all(&macos).and_then(|_| fs::create_dir_all(&resources)) {
        println!("cargo:warning=failed to create {APP_NAME}.app: {err}");
        return;
    }

    let exe = macos.join(APP_NAME);
    if fs::symlink_metadata(&exe).is_err() {
        let bin = env::var("CARGO_PKG_NAME").unwrap();
        if let Err(err) = std::os::unix::fs::symlink(format!("../../../{bin}"), &exe) {
            println!("cargo:warning=failed to link {APP_NAME} executable: {err}");
        }
    }

    if let Err(err) = fs::write(contents.join("Info.plist"), info_plist()) {
        println!("cargo:warning=failed to write Info.plist: {err}");
    }

    let icon_src = manifest.join(format!("assets/{ICON_NAME}.icon"));
    if icon_src.is_dir() {
        compile_icon(&icon_src, &resources, &out_dir);
    }
}

fn compile_icon(icon_src: &Path, resources: &Path, out_dir: &Path) {
    let output = Command::new("xcrun")
        .arg("actool")
        .arg(icon_src)
        .arg("--compile")
        .arg(resources)
        .args(["--platform", "macosx"])
        .args(["--minimum-deployment-target", "26.0"])
        .args(["--app-icon", ICON_NAME])
        .arg("--output-partial-info-plist")
        .arg(out_dir.join("actool-partial.plist"))
        .output();

    match output {
        Ok(out) if out.status.success() => {}
        Ok(out) => println!(
            "cargo:warning=actool failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ),
        Err(err) => println!("cargo:warning=actool unavailable: {err}"),
    }
}

fn info_plist() -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key><string>{APP_NAME}</string>
  <key>CFBundleDisplayName</key><string>{APP_NAME}</string>
  <key>CFBundleExecutable</key><string>{APP_NAME}</string>
  <key>CFBundleIdentifier</key><string>{BUNDLE_ID}</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>0.1</string>
  <key>CFBundleIconFile</key><string>{ICON_NAME}</string>
  <key>CFBundleIconName</key><string>{ICON_NAME}</string>
  <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
"#
    )
}
