use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const LIBRCLONE_PACKAGE: &str = "github.com/rclone/rclone/librclone";
const RCLONE_EXPORTED_FUNCTIONS: [&str; 4] = [
    "RcloneRPC",
    "RcloneInitialize",
    "RcloneFinalize",
    "RcloneFreeString",
];

struct BuildConfig<'a> {
    mode: &'a str,
    output_name: &'a str,
    extra_go_args: &'a [&'a str],
    link_kind: &'a str,
    link_name: &'a str,
    error_context: &'a str,
}

struct WindowsGoConfig {
    extra_env: Vec<(&'static str, String)>,
    extra_go_args: &'static [&'static str],
}

fn main() {
    let target_triple = env::var("TARGET").unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // docs.rs builder blocks network, would have to vendor everything. This allows `librclone` itself to doc build.
    if env::var("DOCS_RS").is_ok() {
        std::fs::write(out_path.join("bindings.rs"), "").unwrap();
        return;
    }

    println!("cargo:rerun-if-changed=go.mod");
    println!("cargo:rerun-if-changed=go.sum");
    println!("cargo:rerun-if-changed=librclone.go");

    if target_triple.contains("windows") {
        println!("cargo:rerun-if-env-changed=LIBRCLONE_WINFSP_INCLUDE");
        println!("cargo:rerun-if-env-changed=LIBRCLONE_GO_CC");
        let windows_go = windows_go_config();
        build_librclone(
            &out_path,
            BuildConfig {
                mode: "c-shared",
                output_name: "librclone.dll",
                extra_go_args: windows_go.extra_go_args,
                link_kind: "dylib",
                link_name: "librclone",
                error_context: "unable to build librclone.dll for Windows",
            },
            &windows_go.extra_env,
        );
        if target_triple.contains("msvc") {
            create_msvc_import_library(&out_path, &target_triple);
        }
    } else {
        build_librclone(
            &out_path,
            BuildConfig {
                mode: "c-archive",
                output_name: "librclone.a",
                extra_go_args: &[],
                link_kind: "static",
                link_name: "rclone",
                error_context: "unable to build librclone static archive",
            },
            &[],
        );

        if target_triple.ends_with("darwin") {
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=Security");
            println!("cargo:rustc-link-lib=resolv");
        }
    }

    generate_bindings(&out_path);
}

fn build_librclone(out_path: &Path, config: BuildConfig<'_>, extra_env: &[(&str, String)]) {
    let output_path = out_path.join(config.output_name);
    run_go_build(
        config.mode,
        &output_path,
        config.extra_go_args,
        extra_env,
        config.error_context,
    );

    println!("cargo:rustc-link-search=native={}", out_path.display());
    println!(
        "cargo:rustc-link-lib={}={}",
        config.link_kind, config.link_name
    );
}

fn windows_go_config() -> WindowsGoConfig {
    let mut extra_env = vec![("CGO_ENABLED", "1".to_string())];
    let mut extra_go_args: &'static [&'static str] = &[];

    if let Some(winfsp_include) = non_empty_env("LIBRCLONE_WINFSP_INCLUDE") {
        extra_env.push(("CPATH", winfsp_include));
        extra_go_args = &["-tags", "cmount"];
    } else {
        println!(
            "cargo:warning=LIBRCLONE_WINFSP_INCLUDE is not set. Building without `cmount` tag on Windows."
        );
    }

    if let Some(go_cc) = non_empty_env("LIBRCLONE_GO_CC") {
        extra_env.push(("CC", go_cc));
    }

    WindowsGoConfig {
        extra_env,
        extra_go_args,
    }
}

fn generate_bindings(out_path: &Path) {
    let header_path = out_path.join("librclone.h");
    let mut builder = bindgen::Builder::default()
        .header(header_path.to_string_lossy().to_string())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    for function in RCLONE_EXPORTED_FUNCTIONS {
        builder = builder.allowlist_function(function);
    }

    let bindings = builder.generate().expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn run_go_build(
    build_mode: &str,
    output_path: &Path,
    extra_args: &[&str],
    extra_env: &[(&str, String)],
    context: &str,
) {
    let mut args = vec!["build".to_string(), format!("--buildmode={build_mode}")];
    args.push("-mod=mod".to_string());
    args.extend(extra_args.iter().map(|arg| (*arg).to_string()));
    args.push("-o".to_string());
    args.push(output_path.display().to_string());
    args.push(LIBRCLONE_PACKAGE.to_string());

    run_command("go", &args, extra_env, context);
}

fn create_msvc_import_library(out_path: &Path, target_triple: &str) {
    let machine_arg = msvc_machine_arg(target_triple).unwrap_or_else(|| {
        panic!(
            "unsupported MSVC target architecture in TARGET={target_triple}; expected x86_64, i686, or aarch64"
        )
    });

    let def_path = out_path.join("librclone.def");
    let def_body = format!(
        "LIBRARY librclone\nEXPORTS\n{}\n",
        RCLONE_EXPORTED_FUNCTIONS.join("\n")
    );
    std::fs::write(&def_path, def_body)
        .unwrap_or_else(|err| panic!("unable to write {}: {err}", def_path.display()));

    let out_lib = out_path.join("librclone.lib");
    let args = vec![
        format!("/def:{}", def_path.display()),
        machine_arg.to_string(),
        format!("/out:{}", out_lib.display()),
    ];
    run_command(
        "lib.exe",
        &args,
        &[],
        "unable to generate MSVC import library librclone.lib; ensure lib.exe is available in PATH",
    );
}

fn non_empty_env(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn msvc_machine_arg(target_triple: &str) -> Option<&'static str> {
    if target_triple.starts_with("x86_64-") {
        Some("/machine:x64")
    } else if target_triple.starts_with("i686-") {
        Some("/machine:x86")
    } else if target_triple.starts_with("aarch64-") {
        Some("/machine:arm64")
    } else {
        None
    }
}

fn run_command(program: &str, args: &[String], extra_env: &[(&str, String)], context: &str) {
    let invocation = format!("{program} {}", args.join(" "));

    let mut command = Command::new(program);
    command.args(args);
    for (key, value) in extra_env {
        command.env(key, value);
    }

    let output = command
        .output()
        .unwrap_or_else(|err| panic!("`{invocation}` failed to start: {err}"));
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!(
            "{context}: `{invocation}` exited with status {}. stderr='{}'. stdout='{}'",
            output.status,
            stderr.trim(),
            stdout.trim()
        );
    }
}
