use std::env::consts;

use serde_json::json;

fn call(method: &str, input: serde_json::Value) {
    println!("\n{method} {input}");
    match librclone::rpc(method, input.to_string()) {
        Ok(output) => {
            println!("{output}");
        }
        Err(error) => {
            eprintln!("Error: {error}");
        }
    }
}

fn main() {
    println!("librclone running on: {}", consts::OS);

    librclone::initialize();

    call("core/version", json!({}));
    call(
        "rc/noop",
        json!({
            "lib":"rclone"
        }),
    );

    librclone::finalize();
}
