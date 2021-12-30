ARCH ?= x86_64
MODE ?= debug
CARGO_ADDITIONAL ?=

ifneq ($(MODE),debug)
CARGO_ADDITIONAL += --profile $(MODE)
endif

build:
	@cargo build --target ../.targets/$(ARCH)/kernel.json $(CARGO_ADDITIONAL)

check:
	@cargo check --target ../.targets/$(ARCH)/kernel.json

clippy:
	@cargo check --target ../.targets/$(ARCH)/kernel.json