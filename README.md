[<img alt="github" src="https://img.shields.io/badge/github-trevyn/librclone-663399?style=for-the-badge&labelColor=555555&logo=github" height="27">](https://github.com/trevyn/librclone)
[<img alt="crates.io" src="https://img.shields.io/crates/v/librclone.svg?style=for-the-badge&color=ffc833&logo=rust" height="27">](https://crates.io/crates/librclone)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-librclone-353535?style=for-the-badge&labelColor=555555&logo=docs.rs" height="27">](https://docs.rs/librclone)

Rust bindings for [`librclone`](https://github.com/rclone/rclone/tree/master/librclone).

Automatically compiles `rclone` as a library and links it into your Rust application.

Supports macOS and Linux. PR for Windows support welcome!

| crate version       | `rclone` version | MSRV | Minimum `go` version |
| ------------------- | ---------------- | ---- | -------------------- |
| `librclone = "0.6"` | v1.64.0          | 1.63 | 1.19                 |
| `librclone = "0.5"` | v1.63.1          | 1.60 | 1.18                 |
| `librclone = "0.4"` | v1.62.2          | 1.54 | 1.18                 |
| `librclone = "0.3"` | v1.61.0          | 1.54 | 1.17                 |
| `librclone = "0.2"` | v1.60.1          | 1.54 | 1.17                 |
| `librclone = "0.1"` | v1.56.2          | 1.54 | 1.17                 |

To generate updated `go.mod` and `go.sum` files on new rclone version:

```ignore
cd librclone-sys
rm go.mod && rm go.sum
go mod init github.com/trevyn/librclone
go get -d github.com/rclone/rclone/librclone
```
