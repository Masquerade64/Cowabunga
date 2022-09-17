@echo off
cargo clean
set "RUSTFLAGS=--remap-path-prefix=C:\\Users\\%username%=home"
rustup run stable-x86_64-pc-windows-msvc cargo build --release
copy target\release\cowabunga.exe cowabunga64.exe
cargo clean
set "RUSTFLAGS=--remap-path-prefix=C:\\Users\\%username%=home"
rustup run stable-i686-pc-windows-msvc cargo build --release
copy target\release\cowabunga.exe cowabunga.exe
cargo clean
pause