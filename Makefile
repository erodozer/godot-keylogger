debug:
	cargo build
	cp target/debug/libgd_keylogger.so addons/keylogger/lib/libgd_keylogger.debug.so

release:
	cargo build --release
	cp target/release/libgd_keylogger.so addons/keylogger/lib/libgd_keylogger.release.so

package: debug | release
