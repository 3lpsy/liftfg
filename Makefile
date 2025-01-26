# Define the shell to use
SHELL := /bin/bash

# Handle all args after the first one
.SECONDEXPANSION:
ARGS = $(shell echo "$(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))")
.PHONY: shell shell-dev shell-prod emulator css tailwind dx dx-dev dx-prod

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

# Cli commands
migrate:
	@echo "Running: cd ui && dx serve --port 1420"
	@cd ui && dx serve --port 1420
# DX commands
dx: dx-dev

dx-dev:
	@echo "Running: cd ui && dx serve --port 1420"
	@cd ui && dx serve --port 1420

dx-prod:
	@echo "Running: cd ui && dx build"
	@cd ui && dx build

# Shell commands

shell: shell-dev

shell-dev:
	@echo "Running: cargo  run -p liftfg-shell --no-default-features --features tauri/native-tls --color always -- -d test.db $(ARGS)"
	@cargo run -p liftfg-shell --no-default-features --features tauri/native-tls --color always -- -d test.db $(ARGS)

shell-prod:
	@echo "Running: cargo build"
	@cargo build

shell-test:
	@echo "Running: cargo test -p fgshell $(ARGS)"
	@cargo test -p fgshell $(ARGS)

emulator:
	@QT_QPA_PLATFORM=xcb DISPLAY=:0 $(ANDROID_HOME)/emulator/emulator -avd Pixel_7a_API_34_Default -netdelay none -netspeed full -no-snapshot-load -no-snapshot-save

android: android-dev

android-dev:
	@echo "Running adb uninstall org.liftfg.app"
	@adb uninstall org.liftfg.app || true
	@echo "Running: cargo tauri android dev"
	@QT_QPA_PLATFORM=xcb DISPLAY=:0 cargo tauri android dev

css: tailwind

tailwind:
	@echo "Running: npx @tailwindcss/cli -i styles/tailwind.css -o ./assets/main.css --watch"
	@cd ui && npx @tailwindcss/cli -i styles/tailwind.css -o ./assets/main.css --watch

# Handle arguments
%:
	@:
