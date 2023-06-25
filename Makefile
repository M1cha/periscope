BUILD_TYPE ?= release

all: lib sys ovl vwr

lib: nx/libscope/lib/libscope.a

nx/libscope/lib/libscope.a: $(wildcard nx/libscope/src/*.c)
	$(MAKE) -C nx/libscope/

sys: nx/sysmodule/sys-scope.nsp

nx/sysmodule/sys-scope.nsp: $(wildcard nx/sysmodule/src/*.c)
	$(MAKE) -C nx/sysmodule/

ovl: nx/overlay/periscope-overlay.ovl

nx/overlay/periscope-overlay.ovl: $(wildcard nx/overlay/src/*.c)
	$(MAKE) -C nx/overlay/

vwr: desktop/target/$(BUILD_TYPE)/periscope

ifeq ($(BUILD_TYPE),release)
FLAG := --release
else
FLAG :=
endif

desktop/target/$(BUILD_TYPE)/periscope: $(wildcard desktop/src/*.rs)
	cargo b --manifest-path desktop/Cargo.toml $(FLAG)

clean:
	cargo clean --manifest-path desktop/Cargo.toml
	$(MAKE) -C nx/sysmodule/ clean
	$(MAKE) -C nx/overlay/ clean
	$(MAKE) -C nx/libscope/ clean
