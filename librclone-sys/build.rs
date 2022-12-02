use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target_triple = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir);

    // docs.rs builder blocks network, would have to vendor everything. This allows `librclone` itself to doc build.
    if std::env::var("DOCS_RS").is_ok() {
        std::fs::write(out_path.join("bindings.rs"), "").unwrap();
        return;
    }

    println!("cargo:rerun-if-changed=go.mod");
    println!("cargo:rerun-if-changed=go.sum");

    Command::new("go")
        .args(["build", "--buildmode=c-archive", "-o"])
        .arg(&format!("{}/librclone.a", out_dir))
        .arg("github.com/rclone/rclone/librclone")
        .status()
        .expect("`go build` failed. Is `go` installed and latest version?");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=rclone");

    if target_triple.ends_with("darwin") {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
    }

    let bindings = bindgen::Builder::default()
        .header(format!("{}/librclone.h", out_dir))
        .allowlist_function("RcloneRPC")
        .allowlist_function("RcloneInitialize")
        .allowlist_function("RcloneFinalize")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
