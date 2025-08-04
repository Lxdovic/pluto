EXE   := Pluto_1.0.1
LXE   := Pluto_1.0.1
_THIS := $(realpath $(dir $(abspath $(lastword $(MAKEFILE_LIST)))))
TMPDIR := $(_THIS)/tmp
HCE   := true

ifeq ($(OS),Windows_NT)
	EXT := .exe
	VER := win
	# Different native flag for macOS
else ifeq ($(shell uname -s), Darwin)
	EXT :=
	VER := darwin
else
	EXT :=
	VER := linux
endif

ifeq ($(HCE),true)
	FEATURES := tuning,log,classical
else
	FEATURES := tuning,log
endif

NAME := $(EXE)$(EXT)

rule:
	cargo rustc -r -p engine --bins --features $(FEATURES) -- -C target-cpu=native --emit link=$(NAME)

tmp-dir:
	mkdir -p $(TMPDIR)
