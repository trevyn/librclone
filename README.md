[<img alt="github" src="https://img.shields.io/badge/github-trevyn/librclone-663399?style=for-the-badge&labelColor=555555&logo=github" height="27">](https://github.com/trevyn/librclone)
[<img alt="crates.io" src="https://img.shields.io/crates/v/librclone.svg?style=for-the-badge&color=ffc833&logo=rust" height="27">](https://crates.io/crates/librclone)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-librclone-353535?style=for-the-badge&labelColor=555555&logo=docs.rs" height="27">](https://docs.rs/librclone)

Rust bindings for [`librclone`](https://github.com/rclone/rclone/tree/master/librclone).

Automatically compiles `rclone` as a library and links it into your Rust application.

Supports macOS and Linux. PR for Windows support welcome!

| crate version       | `rclone` version | MSRV | Minimum `go` version |
| ------------------- | ---------------- | ---- | -------------------- |
| `librclone = "0.10"` | v1.72.1          | 1.82 | 1.24.4               |
| `librclone = "0.9"` | v1.69.0          | 1.82 | 1.21                 |
| `librclone = "0.8"` | v1.66.0          | 1.70 | 1.21                 |
| `librclone = "0.7"` | v1.65.0          | 1.65 | 1.19                 |
| `librclone = "0.6"` | v1.64.2          | 1.65 | 1.19                 |
| `librclone = "0.5"` | v1.63.1          | 1.60 | 1.18                 |
| `librclone = "0.4"` | v1.62.2          | 1.54 | 1.18                 |
| `librclone = "0.3"` | v1.61.0          | 1.54 | 1.17                 |
| `librclone = "0.2"` | v1.60.1          | 1.54 | 1.17                 |
| `librclone = "0.1"` | v1.56.2          | 1.54 | 1.17                 |

## Updating the bundled rclone version

1. Update the Go module in `librclone-sys` using the Go version from the target rclone `go.mod`:

   ```sh
   cd librclone-sys
   go mod edit -go=<go-version>
   go mod edit -require=github.com/rclone/rclone@<rclone-version>
   go mod tidy -go=<go-version>
   ```

2. Bump crate versions in `Cargo.toml` and `librclone-sys/Cargo.toml`.
3. Update the version table in `README.md` and add a changelog entry in `CHANGELOG.md`.
