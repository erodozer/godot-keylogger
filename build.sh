#!/usr/bin/env bash

WORKDIR=$(pwd)

cd $WORKDIR/rust
cargo build
cargo build --release

cd $WORKDIR

mv $WORKDIR/rust/target/debug/libgodot_keylogger.so $WORKDIR/godot/addons/keylogger/libgodot_keylogger.debug.so
mv $WORKDIR/rust/target/release/libgodot_keylogger.so $WORKDIR/godot/addons/keylogger/libgodot_keylogger.release.so
