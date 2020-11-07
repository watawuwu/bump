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
TARGET                  := x86_64-apple-darwin
TOOLCHAIN               := stable
CARGO_BIN               := cross
ifneq (,$(findstring mingw64, $(OS)))
    CARGO_BIN := cargo
endif
ifneq (,$(findstring darwin, $(OS)))
    CARGO_BIN := cargo
endif
ifeq (,$(shell command -v cross 2> /dev/null))
    CARGO_BIN := cargo
endif
# cross command coudn't recognize environment variable
CARGO_BUILD_TARGET_DIR  := $(CURDIR)/target
CARGO_BUILD_TARGET      := x86_64-apple-darwin
CARGO_OPTIONS           :=
CARGO_SUB_OPTIONS       := --target $(CARGO_BUILD_TARGET) --target-dir $(CARGO_BUILD_TARGET_DIR)
CARGO_COMMAND           := $(CARGO_BIN) $(CARGO_OPTIONS)
APP_ARGS                := patch 1.0.0

# Environment
#===============================================================
export RUST_LOG=$(LOG)
export RUST_BACKTRACE=1
export DOCKER_BUILDKIT=1
export COMPOSE_DOCKER_CLI_BUILD=1

# Task
#===============================================================
deps: ## Install depend tools
ifneq ($(CARGO_BUILD_TARGET),)
	rustup target add $(CARGO_BUILD_TARGET)
endif
	rustup component add rustfmt
	rustup component add clippy
	rustup show # for container

dev-deps: ## Install dev depend tools
	rustup component add rust-src
	$(CARGO_COMMAND) install --force cargo-outdated

run: fix fmt clippy ## Execute a main.rs
	$(CARGO_COMMAND) run -- $(APP_ARGS) $(CARGO_SUB_OPTIONS)

test: fix fmt clippy ## Run the tests
	$(CARGO_COMMAND) test $(CARGO_SUB_OPTIONS) -- --nocapture

check: fix fmt ## Check syntax, but don't build object files
	$(CARGO_COMMAND) check $(CARGO_SUB_OPTIONS)

build: ## Build all project
	$(CARGO_COMMAND) build $(CARGO_SUB_OPTIONS)

release-build: ## Build all project
	$(CARGO_COMMAND) build --release $(CARGO_SUB_OPTIONS)

check-lib: ## Check module version
	$(CARGO_COMMAND) outdated -R

update: ## Update modules
	$(CARGO_COMMAND) update

clean: ## Remove the target directory
	$(CARGO_COMMAND) clean

install: ## Install to $(PREFIX) directory
	$(CARGO_COMMAND) install --force --root $(PREFIX) --path .

fix: ## Run fmt
	$(CARGO_COMMAND) fix --allow-staged --allow-dirty $(CARGO_SUB_OPTIONS)

fmt: ## Run fmt
	$(CARGO_COMMAND) fmt

fmt-check: ## Run fmt
	$(CARGO_COMMAND) fmt --all -- --check

clippy: ## Run clippy
	$(CARGO_COMMAND) clippy --all-features $(CARGO_SUB_OPTIONS) -- -D warnings

publish:
ifeq ($(LEVEL),)
	$(error LEVEL not set correctly.)
endif
	cargo release $(LEVEL) --no-dev-version --tag-name "{{version}}"

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
