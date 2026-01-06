TARGET_DIR := dylib

build:
	cargo build --lib --release
	cp target/release/libc_accumulative_hash_set.* $(TARGET_DIR)/
	cbindgen --config cbindgen.toml --crate c-accumulative-hash-set --output $(TARGET_DIR)/libc_accumulative_hash_set.h
