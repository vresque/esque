ARCH ?= x86_64
MODE ?= debug
CARGO_ADDITIONAL ?=

ifneq ($(MODE),debug)
CARGO_ADDITIONAL += --profile $(MODE)
endif


build:
	echo $(CARGO_ADDITIONAL)
	@cargo build --target ../.targets/$(ARCH)/boot.json $(CARGO_ADDITIONAL)