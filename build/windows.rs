use std::path::PathBuf;
use std::{env, fs};

const APP_NAME: &str = "Ciart Studio";
const ICON_NAME: &str = "AppIcon";

pub fn build() {
    println!("cargo:rerun-if-changed=assets/{ICON_NAME}.ico");
    println!("cargo:rerun-if-changed=Cargo.toml");

    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let icon = manifest.join(format!("assets/{ICON_NAME}.ico"));
    if !icon.is_file() {
        println!("cargo:warning=missing assets/{ICON_NAME}.ico");
        return;
    }

    let script = out_dir.join(format!("{ICON_NAME}.rc"));
    if let Err(err) = fs::write(&script, resource_script(&icon)) {
        println!("cargo:warning=failed to write {ICON_NAME}.rc: {err}");
        return;
    }

    if let Err(err) = embed_resource::compile(&script, embed_resource::NONE).manifest_optional() {
        println!("cargo:warning=failed to embed resources: {err:?}");
    }
}

fn resource_script(icon: &std::path::Path) -> String {
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let bin = env::var("CARGO_PKG_NAME").unwrap();
    let major = env::var("CARGO_PKG_VERSION_MAJOR").unwrap();
    let minor = env::var("CARGO_PKG_VERSION_MINOR").unwrap();
    let patch = env::var("CARGO_PKG_VERSION_PATCH").unwrap();

    let description = env::var("CARGO_PKG_DESCRIPTION")
        .ok()
        .filter(|it| !it.is_empty())
        .unwrap_or_else(|| APP_NAME.to_string());
    let company = env::var("CARGO_PKG_AUTHORS")
        .ok()
        .and_then(|it| it.split(':').next().map(str::to_string))
        .filter(|it| !it.is_empty())
        .unwrap_or_else(|| APP_NAME.to_string());

    let icon = escape(&icon.display().to_string());
    format!(
        r#"1 ICON "{icon}"

1 VERSIONINFO
FILEVERSION {major},{minor},{patch},0
PRODUCTVERSION {major},{minor},{patch},0
FILEOS 0x40004L
FILETYPE 0x1L
BEGIN
  BLOCK "StringFileInfo"
  BEGIN
    BLOCK "040904b0"
    BEGIN
      VALUE "CompanyName", "{company}"
      VALUE "FileDescription", "{description}"
      VALUE "FileVersion", "{version}"
      VALUE "InternalName", "{bin}"
      VALUE "OriginalFilename", "{bin}.exe"
      VALUE "ProductName", "{app}"
      VALUE "ProductVersion", "{version}"
    END
  END
  BLOCK "VarFileInfo"
  BEGIN
    VALUE "Translation", 0x409, 1200
  END
END
"#,
        app = escape(APP_NAME),
        company = escape(&company),
        description = escape(&description),
    )
}

fn escape(value: &str) -> String {
    value.replace('\\', r"\\").replace('"', r#"\""#)
}
