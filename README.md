Rust bindings for [`librclone`](https://github.com/rclone/rclone/tree/master/librclone).

Supports macOS and Linux. PR for Windows support welcome!

To generate updated `go.mod` and `go.sum` files on new rclone version:

```ignore
cd librclone-sys
rm go.mod && rm go.sum
go mod init github.com/trevyn/librclone
go get -d github.com/rclone/rclone/librclone
```
