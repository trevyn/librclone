Rust bindings for [`librclone`](https://github.com/rclone/rclone/tree/master/librclone).

Automatically compiles `rclone` as a library and links it into your Rust application.

Supports macOS and Linux. PR for Windows support welcome!

| crate version       | `rclone` version | MSRV | Minimum `go` version |
| ------------------- | ---------------- | ---- | -------------------- |
| `librclone = "0.2"` | v1.60.1          | 1.54 | 1.17                 |
| `librclone = "0.1"` | v1.56.2          | 1.54 | 1.17                 |

To generate updated `go.mod` and `go.sum` files on new rclone version:

```ignore
cd librclone-sys
rm go.mod && rm go.sum
go mod init github.com/trevyn/librclone
go get -d github.com/rclone/rclone/librclone
```
