build:
	cargo build --target thumbv7m-none-eabi
inspect:
	cargo readobj --bin embedded-pg -- --file-headers
size:
	cargo size --bin embedded-pg --release -- -A
disassemble:
	cargo objdump --bin embedded-pg --release -- --disassemble --no-show-raw-insn --print-imm-hex
run-qemu:
	qemu-system-arm \
	-cpu cortex-m3 \
	-machine lm3s6965evb \
	-nographic \
	-semihosting-config enable=on,target=native \
	-kernel target/thumbv7m-none-eabi/debug/$(dist)
debug:
	qemu-system-arm \
	-cpu cortex-m3 \
	-machine lm3s6965evb \
	-nographic \
	-semihosting-config enable=on,target=native \
	-gdb tcp::3333 \
	-S \
	-kernel target/thumbv7m-none-eabi/debug/$(dist)
gdb:
	gdb -q target/thumbv7m-none-eabi/debug/$(dist)