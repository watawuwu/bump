# Setting
#===============================================================
SHELL := /bin/bash
OS    := $(shell uname | tr A-Z a-z)

# Const
#===============================================================
name := bump

# Option
#===============================================================
SHELL                   := /bin/bash
LOG_LEVEL               := debug
PREFIX                  := $(HOME)/.cargo
LOG                     := $(shell echo '$(name)' | tr - _)=$(LOG_LEVEL)
CARGO_TOOLCHAIN         := stable
CARGO_OPTIONS           :=
CARGO_SUB_OPTIONS       :=
CARGO_COMMAND           := cargo +$(CARGO_TOOLCHAIN) $(CARGO_OPTIONS)
APP_ARGS                := patch 1.0.0

# Environment
#===============================================================
export RUST_LOG=$(LOG)
export RUST_BACKTRACE=1

# Task
#===============================================================
deps: ## Install depend tools
	rustup component add rust-src
	rustup component add rustfmt
	rustup component add clippy
	$(CARGO_COMMAND) install --force cargo-outdated

run: lint ## Execute a main.rs
	$(CARGO_COMMAND) run $(CARGO_SUB_OPTIONS) $(APP_ARGS)

test: lint ## Run the tests
	$(CARGO_COMMAND) test $(CARGO_SUB_OPTIONS) -- --nocapture

check: ## Check syntax, but don't build object files
	$(CARGO_COMMAND) check $(CARGO_SUB_OPTIONS)

build: ## Build all project
	$(CARGO_COMMAND) build $(CARGO_SUB_OPTIONS)

update: ## Update modules
	$(CARGO_COMMAND) update

check-dep: ## Check dep version
	$(CARGO_COMMAND) outdated

clean: ## Remove the target directory
	$(CARGO_COMMAND) clean

install: ## Install to $(PREFIX) directory
	$(CARGO_COMMAND) install --force --root $(PREFIX) --path .

fmt: ## Run fmt
	$(CARGO_COMMAND) fmt

clippy: ## Run clippy
	$(CARGO_COMMAND) clippy

lint: fmt clippy ## Run fmt and clippy

release-build: ## Build all project
	$(MAKE) build CARGO_SUB_OPTIONS="--release"

help: ## Print help
	echo -e "Usage: make [task]\n\nTasks:"
	perl -nle 'printf("    \033[33m%s%-20s\033[0m %s\n",$$1,$$2,$$3) if /^([a-zA-Z]){1}([a-zA-Z_-]*?):(?:.+?## )?(.*?)$$/' $(MAKEFILE_LIST)

# Config
#===============================================================
.SILENT: help
# If you want `Target` instead of `Task`, you can avoid it by using dot(.) and slash(/)
# ex) node_modules: => ./node_modules:
.PHONY: $(shell egrep -o '^(_)?[a-zA-Z-]+:' $(MAKEFILE_LIST) | sed 's/://')
.DEFAULT_GOAL := build
