# Changelog

## 0.6.2 - 2023-10-18

- Update rclone to v1.64.1
- Change `librclone-sys` SPDX to include licenses for all `rclone` Go dependencies
- MSRV is now 1.65

## 0.6.1 - 2023-10-09

- Vendor Go dependencies into `librclone-sys`; should now support offline builds (#12, thanks @brettalcox!)

## 0.6.0 - 2023-10-02

- Update rclone to v1.64.0
- MSRV is now 1.63

## 0.5.0 - 2023-07-19

- Update rclone to v1.63.1
- MSRV is now 1.60

## 0.4.0 - 2023-04-06

- Update rclone to v1.62.2
- Change `librclone-sys` SPDX to MIT-only to reflect `rclone` license
- Fix `aarch64-unknown-linux-gnu` build (Thanks @dotlambda and @winterqt!)
- Fix `libresolv` linkage on macOS with Rust 1.66+ and Go 1.20+.

## 0.3.0 - 2022-12-22

- Update rclone to v1.61.0

## 0.2.0 - 2022-12-02

- Update rclone to v1.60.1

## 0.1.1 - 2021-10-06

- Fix docs.rs build

## 0.1.0 - 2021-10-05

- Initial release! 🎉
- MSRV is Rust 1.54, rclone is v1.56.2
