#[cfg(unix)]
#[path = "build/macos.rs"]
mod macos;

#[cfg(windows)]
#[path = "build/windows.rs"]
mod windows;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(unix)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        macos::build();
    }

    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        windows::build();
    }
}
