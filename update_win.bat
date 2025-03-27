@echo off

echo Download GXShell...
set CARGO_TARGET_DIR=E:\gxshell_release\win
 

cargo build --release
cargo install --path . --root %CARGO_TARGET_DIR% 