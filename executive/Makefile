ARCH ?= x86_64
MODE ?= debug
CARGOADD ?=
ifeq ($(MODE),release)
CARGOADD += --release
endif

THISDIR := .
DIRS = $(wildcard */)

all: dirs

dirs: */
	@for dir in $(DIRS); do \
		$(MAKE) -C $${dir} ARCH=$(ARCH) CARGOADD=$(CARGOADD) MODE=$(MODE); \
	done