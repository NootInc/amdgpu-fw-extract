# amdgpu-fw-extract

Rust CLI tool for extracting actual firmware binaries from Linux AMDGPU firmware descriptor binaries

The syntax for this small program is `executable <FW TYPE> <INPUT FILE> <OUTPUT FILE>`, i.e. `cargo run psp_dtm ~/Downloads/raven_ta.bin ~/Downloads/raven_dtm.bin`

Valid firmware types:

- psp_xgmi
- psp_ras
- psp_hdcp
- psp_dtm
- psp_secure_display
- psp_asd
- gfx

This project is licensed with the Creative Commons Attribution-NoCommercial-NoDerivatives license. You should've
received this license with this project, if not, see [here](https://creativecommons.org/licenses/by-nc-nd/4.0/).
