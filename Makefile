.PHONY: build run clean help debug release

all: run

build:
	@echo "Building bootimage (debug)..."
	cargo bootloader

release:
	@echo "Building bootimage (release)..."
	cargo bootloader -r

run: release
	@echo "Starting QEMU..."
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os-none/release/bootimage-rust-os.bin

debug: build
	@echo "Starting QEMU (debug)..."
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust_os-none/debug/bootimage-rust-os.bin

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
