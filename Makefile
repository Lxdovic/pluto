EXE   := Pluto_2.0
LXE   := Pluto_2.0
_THIS := $(realpath $(dir $(abspath $(lastword $(MAKEFILE_LIST)))))
TMPDIR := $(_THIS)/tmp

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

NAME := $(EXE)$(EXT)

rule:
	cargo rustc -- -C target-cpu=native --emit link=$(NAME)

tmp-dir:
	mkdir -p $(TMPDIR)
