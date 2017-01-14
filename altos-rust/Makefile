binary = altos_rust
static_lib = lib$(binary).a
linker_script = rust.ld
target = thumbv6m-none-eabi
core_lib = altos_core

build_path = build/

debug_static_lib = target/$(target)/debug/$(static_lib)
debug_build_path = $(build_path)debug/
debug_build = $(debug_build_path)$(binary)

release_static_lib = target/$(target)/release/$(static_lib)
release_build_path = $(build_path)release/
release_build = $(release_build_path)$(binary)

### CARGO ###
cargo = xargo
cargo_args = --target $(target)

### TEST ###
test_dependencies = altos_core \
										arm \
										volatile \

# --lib flag only runs the unit test suite, doc tests are currently and issue for cross-compiled 
#  platforms. See: https://github.com/rust-lang/cargo/issues/1789
test_args = $(foreach dep, $(test_dependencies),-p $(dep)) --lib

### LINKER ###
linker = arm-none-eabi-ld
linker_args = -n --gc-sections -T $(linker_script) 

### SIZE ###
size = arm-none-eabi-size
size_flags = -t

### GDB ###
gdb = arm-none-eabi-gdb
gdb_flags = 
st_port = 4242
ocd_port = 3333
st_gdb_flags = $(gdb_flags) -eval-command="target remote :$(st_port)"
ocd_gdb_flags = $(gdb_flags) -eval-command="target remote :$(ocd_port)"

### Make targets ###

.PHONY: debug release clean

all: debug

clean:
	@$(cargo) clean
	@rm -rf $(build_path)

debug: $(linker_script)
	@mkdir -p $(debug_build_path)
	@$(cargo) build $(cargo_args)
	@$(linker) $(linker_args) -o $(debug_build) $(debug_static_lib)
	@$(size) $(size_flags) $(debug_build)

release: $(linker_script)
	@mkdir -p $(release_build_path)
	@$(cargo) build $(cargo_args) --release
	@$(linker) $(linker_args) -o $(release_build) $(release_static_lib)
	@$(size) $(size_flags) $(release_build)

gdb: debug
	@$(gdb) $(gdb_flags) $(debug_build)
	
gdb-st: debug
	@$(gdb) $(st_gdb_flags) $(debug_build)

gdb-ocd: debug
	@$(gdb) $(ocd_gdb_flags) $(debug_build)

test:
	@$(cargo) test $(test_args)

size: debug

