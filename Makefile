# Makefile for gmk

PROJECT_NAME := gmk
VERSION := $(shell grep '^version =' Cargo.toml | cut -d '"' -f 2)

.PHONY: all
all: build

# Development
.PHONY: dev
dev: ## Run the tool in development mode. usage: make dev ARGS="list"
	cargo run -- $(ARGS)

.PHONY: test
test: ## Run tests
	cargo test

.PHONY: lint
lint: ## Run format check and clippy
	cargo fmt -- --check
	cargo clippy -- -D warnings

.PHONY: clean
clean: ## Clean build artifacts
	cargo clean

# Build
.PHONY: build
build: ## Build release binary
	cargo build --release

# Installation
.PHONY: install
install: ## Install gmk locally
	cargo install --path .

# Demo
.PHONY: demo
demo: build ## Generate demo GIF into assets/demo.gif
	mkdir -p assets
	vhs demo.tape

# Help
.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%%-20s\033[0m %%s\n", $$1, $$2}'
