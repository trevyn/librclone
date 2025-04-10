#[cfg(test)]
mod tests {
    use librclone::{finalize, initialize, rpc};
    use serde_json::{json, Value};

    #[test]
    fn get_rpc_config() {
        initialize();
        let res = rpc("options/get", json!({}).to_string()).unwrap();
        let value = serde_json::from_str::<Value>(&res).unwrap();
        assert!(value.get("main").is_some());
        finalize();
    }

    #[test]
    fn set_rpc_config() {
        initialize();
        let res = rpc(
            "options/set",
            json!({
                "main": {
                    "AskPassword": false,
                },
            })
            .to_string(),
        );
        assert!(res.is_ok());
        finalize();
    }
}
