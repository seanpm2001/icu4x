# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.DEFAULT_GOAL := build
FORCE:

ALL_HEADERS := $(wildcard ../../include/*)
ICU4X_NIGHTLY_TOOLCHAIN ?= "nightly-2022-12-26"

$(ALL_HEADERS):

# 100 KiB, working around a bug in older rustc
# https://github.com/unicode-org/icu4x/issues/2753
# keep in sync with .cargo/config.toml
WASM_STACK_SIZE := 100000

BASEDIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

lib: $(ALL_HEADERS)
	rm -rf lib;
	mkdir lib;
	cp ../../include/* lib

target/wasm32-unknown-unknown/release/icu_capi_cdylib.wasm: FORCE
	rustup toolchain install ${ICU4X_NIGHTLY_TOOLCHAIN}
	rustup component add rust-src --toolchain ${ICU4X_NIGHTLY_TOOLCHAIN}
	# Build the WASM library
	RUSTFLAGS="-Cpanic=abort -Copt-level=s -C link-arg=-zstack-size=${WASM_STACK_SIZE} -Clinker-plugin-lto -Ccodegen-units=1 -C linker=${BASEDIR}/ld.py -C linker-flavor=wasm-ld" \
	CARGO_TARGET_DIR="target" `# Special linker flags shouldn't overwrite the top-level target directory` \
	cargo +${ICU4X_NIGHTLY_TOOLCHAIN} build \
		-Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort \
		--target wasm32-unknown-unknown \
		--release \
		--package icu_capi_cdylib \
		--no-default-features
		--features buffer_provider,logging \

icu_capi.wasm: target/wasm32-unknown-unknown/release/icu_capi_cdylib.wasm
	cp target/wasm32-unknown-unknown/release/icu_capi_cdylib.wasm icu_capi.wasm

data.postcard: icu_capi.wasm
	cargo run -p icu_datagen -- \
		--keys-for-bin icu_capi.wasm \
		--locales full \
		--format blob \
		--out data.postcard

build: icu_capi.wasm data.postcard lib

clean:
	git clean -xf *
