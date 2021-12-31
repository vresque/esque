
APPNAME = objector
APPVERSION = 0.1


ARCH ?= x86_64
MODE ?= debug
CARGOADD ?=

all: build move

build:
	cargo build --target ../../.targets/$(ARCH)/apps.json

move:
	cp ../../target/apps/$(MODE)/$(APPNAME) ../../build/$(APPNAME)

