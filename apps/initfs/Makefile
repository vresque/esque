
APPNAME = initfs
APPVERSION = 0.1


ARCH ?= x86_64
MODE ?= debug
CARGOADD ?=
ifeq ($(MODE),release)
CARGOADD += --release
endif

all: build move

build:
	cargo build --target ../../.target/$(ARCH)/apps.json

move:
	ls ../../build/apps &> /dev/null || mkdir --parents ../../build/apps/
	cp ../../target/$(APPNAME)/$(MODE)/$(APPNAME) ../../build/apps/$(APPNAME)

