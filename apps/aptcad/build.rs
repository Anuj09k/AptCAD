//! Windows only: embed the app icon and version info into `aptcad.exe`. Elsewhere a no-op.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/aptcad.ico");
    println!("cargo:rerun-if-env-changed=APTCAD_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=APTCAD_BUILD_SHA");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    if std::path::Path::new("../../assets/app-icon/aptcad.ico").exists() {
        res.set_icon("../../assets/app-icon/aptcad.ico");
    }
    res.set("ProductName", "AptCAD")
        .set("FileDescription", "AptCAD computer-aided design")
        .set("CompanyName", "Aptimizer")
        .set("LegalCopyright", "Copyright (c) 2026 Anuj N (Aptimizer). MIT OR Apache-2.0.")
        .set("OriginalFilename", "aptcad.exe")
        .set("InternalName", "aptcad");
    if let Err(e) = res.compile() {
        if std::env::var_os("APTCAD_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=aptcad.exe built without icon/version resources: {e}");
    }
}
