#!/usr/bin/env bash

WORKDIR=$(pwd)

cd $WORKDIR/rust
cargo build --release --target x86_64-pc-windows-gnu  
cargo build --release --target x86_64-unknown-linux-gnu

cd $WORKDIR

mv $WORKDIR/rust/target/x86_64-pc-windows-gnu/release/godot_keylogger.dll $WORKDIR/godot/addons/keylogger/lib/libgodot_keylogger.release.dll
mv $WORKDIR/rust/target/x86_64-unknown-linux-gnu/release/libgodot_keylogger.so $WORKDIR/godot/addons/keylogger/lib/libgodot_keylogger.release.so
