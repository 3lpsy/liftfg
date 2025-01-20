# Define the shell to use
SHELL := /bin/bash

# Handle all args after the first one
.SECONDEXPANSION:
ARGS = $(shell echo "$(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))")

# Default target shows available commands
.PHONY: all
all:
	@echo "Available commands:"
	@echo "  make dx          - Same as make dx-dev"
	@echo "  make dx-dev      - Change to ui directory and run dx serve"
	@echo "  make dx-prod     - Change to ui directory and run dx build"
	@echo "  make shell       - Same as make shell-dev"
	@echo "  make shell-dev   - Run cargo tauri dev with optional args"
	@echo "  make shell-prod  - Run cargo build"

# DX commands
.PHONY: dx dx-dev dx-prod
dx: dx-dev

dx-dev:
	@echo "Running: cd ui && dx serve"
	@cd ui && dx serve --port 1420

dx-prod:
	@echo "Running: cd ui && dx build"
	@cd ui && dx build

# Shell commands
.PHONY: shell shell-dev shell-prod
shell: shell-dev

shell-dev:
	@echo "Running: cargo  run -p liftfg-shell --no-default-features --features tauri/native-tls --color always -- -d test.db $(ARGS)"
	@cargo  run -p liftfg-shell --no-default-features --features tauri/native-tls --color always -- -d test.db $(ARGS)

shell-prod:
	@echo "Running: cargo build"
	@cargo build

android: android-dev

android-dev:
	@echo "Running adb uninstall org.liftfg.app"
	@adb uninstall org.liftfg.app || true
	@echo "Running: cargo tauri android dev"
	@cargo tauri android dev

# Handle arguments
%:
	@:
