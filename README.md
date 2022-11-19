# amdgpu-fw-extract

CLI tool for extracting actual firmware binaries from Linux AMDGPU firmware binaries, written in Rust.

The syntax for this small program is `<EXEC> <FW TYPE> <INPUT FILE> <OUTPUT FILE>`, i.e. `cargo run dtm ~/Downloads/raven_ta.bin ~/Downloads/raven_dtm.bin`

Valid firmware types:

- xgmi
- ras
- hdcp
- dtm
- secure_display
- asd
- gfx

This project is licensed with the Creative Commons Attribution-NoCommercial-NoDerivatives license. You should've
received this license with this project, if not, see [here](https://creativecommons.org/licenses/by-nc-nd/4.0/).
