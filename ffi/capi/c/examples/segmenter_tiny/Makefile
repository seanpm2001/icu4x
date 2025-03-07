# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

# PLEASE KEEP THIS FILE IN SYNC WITH ../fixeddecimal_tiny/Makefile

.DEFAULT_GOAL := test
.PHONY: build test
FORCE:

ALL_HEADERS := $(wildcard ../../include/*.h)
ICU4X_NIGHTLY_TOOLCHAIN ?= "nightly-2022-12-26"

$(ALL_HEADERS):

GCC := gcc
CLANG := clang-15
LLD := lld-15

../../../../../target/debug/libicu_capi_staticlib.a: FORCE
	cargo build -p icu_capi_staticlib --no-default-features --features compiled_data,default_components

icu_capi_staticlib_tiny/target/x86_64-unknown-linux-gnu/debug/libicu_capi_staticlib_tiny.a: FORCE
	rustup toolchain install ${ICU4X_NIGHTLY_TOOLCHAIN}
	rustup component add rust-src --toolchain ${ICU4X_NIGHTLY_TOOLCHAIN}
	cd icu_capi_staticlib_tiny && \
	RUSTFLAGS="-Clinker-plugin-lto -Clinker=$(CLANG) -Ccodegen-units=1 -Clink-arg=-flto -Cpanic=abort" cargo +${ICU4X_NIGHTLY_TOOLCHAIN} panic-abort-build --target x86_64-unknown-linux-gnu

icu_capi_staticlib_tiny/target/x86_64-unknown-linux-gnu/release-opt-size/libicu_capi_staticlib_tiny.a: FORCE
	rustup toolchain install ${ICU4X_NIGHTLY_TOOLCHAIN}
	rustup component add rust-src --toolchain ${ICU4X_NIGHTLY_TOOLCHAIN}
	cd icu_capi_staticlib_tiny && \
	RUSTFLAGS="-Clinker-plugin-lto -Clinker=$(CLANG) -Ccodegen-units=1 -Clink-arg=-flto -Cpanic=abort" cargo +${ICU4X_NIGHTLY_TOOLCHAIN} panic-abort-build --target x86_64-unknown-linux-gnu --profile=release-opt-size

# Naive target: no optimizations, full std
optim0.elf: ../../../../../target/debug/libicu_capi_staticlib.a $(ALL_HEADERS) test.c
	$(GCC) test.c ../../../../../target/debug/libicu_capi_staticlib.a -ldl -lpthread -lm -g -o optim0.elf

# optim.elf: gcc with maximum link-time code stripping (gc-sections and strip-all)
optim1.elf: ../../../../../target/debug/libicu_capi_staticlib.a $(ALL_HEADERS) test.c
	$(GCC) -fdata-sections -ffunction-sections test.c ../../../../../target/debug/libicu_capi_staticlib.a -ldl -lpthread -lm -g -o optim1.elf -Wl,--gc-sections -Wl,--strip-all

# optim2.elf: clang single-step with gc-sections
optim2.elf: icu_capi_staticlib_tiny/target/x86_64-unknown-linux-gnu/debug/libicu_capi_staticlib_tiny.a $(ALL_HEADERS) test.c
	$(CLANG) -flto -fdata-sections -ffunction-sections test.c icu_capi_staticlib_tiny/target/x86_64-unknown-linux-gnu/debug/libicu_capi_staticlib_tiny.a -g -o optim2.elf -Wl,--gc-sections

optim3.o: $(ALL_HEADERS) test.c
	$(CLANG) -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -g -o optim3.o

# optim3.elf: clang two-step with lld, debug mode
optim3.elf: optim3.o icu_capi_staticlib_tiny/target/x86_64-unknown-linux-gnu/debug/libicu_capi_staticlib_tiny.a
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -L . -o optim3.elf optim3.o icu_capi_staticlib_tiny/target/x86_64-unknown-linux-gnu/debug/libicu_capi_staticlib_tiny.a -Wl,--gc-sections

optim4.o: $(ALL_HEADERS) test.c
	$(CLANG) -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -g -o optim4.o

# optim4.elf: clang two-step with lld, release mode with debug symbols
optim4.elf: optim4.o icu_capi_staticlib_tiny/target/x86_64-unknown-linux-gnu/release-opt-size/libicu_capi_staticlib_tiny.a
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -L . -o optim4.elf optim4.o icu_capi_staticlib_tiny/target/x86_64-unknown-linux-gnu/release-opt-size/libicu_capi_staticlib_tiny.a -Wl,--gc-sections

optim5.o: $(ALL_HEADERS) test.c
	$(CLANG) -c -flto=thin -fdata-sections -ffunction-sections --target=x86_64-unknown-linux-gnu test.c -o optim5.o

# optim5.elf: clang two-step with lld, release mode stripped of debug symbols
optim5.elf: optim5.o icu_capi_staticlib_tiny/target/x86_64-unknown-linux-gnu/release-opt-size/libicu_capi_staticlib_tiny.a
	$(CLANG) -flto=thin -fuse-ld=$(LLD) -L . -o optim5.elf optim5.o icu_capi_staticlib_tiny/target/x86_64-unknown-linux-gnu/release-opt-size/libicu_capi_staticlib_tiny.a -Wl,--gc-sections -Wl,--strip-all

build: optim0.elf optim1.elf optim2.elf optim3.elf optim4.elf optim5.elf

test: build
	./optim0.elf
	./optim1.elf
	./optim2.elf
	./optim2.elf
	./optim4.elf
	./optim5.elf

clean:
	git clean -xf *
	rm -f ../../../../../target/debug/libicu_capi_staticlib.a
