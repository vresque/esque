printf "Creating new app '$1'"

mkdir $1
cd $1
cargo init
echo "#![no_std]
#![no_main]
#[esque::main] // Adds custom Crt0
pub fn main() {}


" > src/main.rs
echo "esque = { path = \"../../lib/esque\" }" >> Cargo.toml
mkdir .cargo
echo '[unstable]
build-std-features = ["compiler-builtins-mem"]
build-std = ["core", "compiler_builtins", "alloc"]' >> .cargo/config.toml
echo "
APPNAME = $1
APPVERSION = 0.1


ARCH ?= x86_64
MODE ?= debug
CARGOADD ?=
ifeq (\$(MODE),release)
CARGOADD += --release
endif

all: build move

build:
	cargo build --target ../../.target/\$(ARCH)/apps.json

move:
	ls ../../build/apps &> /dev/null || mkdir --parents ../../build/apps/
	cp ../../target/\$(APPNAME)/\$(MODE)/\$(APPNAME) ../../build/apps/\$(APPNAME)
" > Makefile
