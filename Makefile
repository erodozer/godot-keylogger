PLATFORM=$(shell uname -o)
ifeq ($(PLATFORM),Darwin)
	EXT=dylib
else
	EXT=so
endif

ARCH=$(shell uname -m)

debug:
	cargo build
	cp target/debug/libgd_keylogger.$(EXT) addons/keylogger/lib/libgd_keylogger.debug.$(ARCH).$(EXT)

release:
	cargo build --release
	cp target/release/libgd_keylogger.$(EXT) addons/keylogger/lib/libgd_keylogger.release.$(ARCH).$(EXT)

package: debug | release
