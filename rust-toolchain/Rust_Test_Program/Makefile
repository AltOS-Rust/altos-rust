binary = altos_rust
kernel = target/$(target)/debug/$(binary)
linker_script = rust.ld
target = cortex-m0

all: cargo

cargo: $(linker_script)
	@xargo build --target $(target)

gdb: cargo
	@arm-none-eabi-gdb $(kernel)
	
