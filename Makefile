#!/usr/bin/env gmake

# Oasis Core binary base path.
OASIS_CORE_ROOT_PATH := .oasis-core

# Runtime binary base path.
RUNTIME_ROOT_PATH := .runtime

# Runtime cargo target directory.
RUNTIME_CARGO_TARGET_DIR := $(if $(CARGO_TARGET_DIR),$(CARGO_TARGET_DIR),target)/default

# Extra build args.
EXTRA_BUILD_ARGS := $(if $(RELEASE),--release,)

# Check if we're running in an interactive terminal.
ISATTY := $(shell [ -t 0 ] && echo 1)

ifdef ISATTY
# Running in interactive terminal, OK to use colors!
MAGENTA = \e[35;1m
CYAN = \e[36;1m
RED = \e[31;1m
OFF = \e[0m

# Built-in echo doesn't support '-e'.
ECHO = /bin/echo -e
else
# Don't use colors if not running interactively.
MAGENTA = ""
CYAN = ""
RED = ""
OFF = ""

# OK to use built-in echo.
ECHO = echo
endif


.PHONY: \
	all \
	check check-oasis-core \
	download-artifacts \
	runtime test-client gateway genesis \
	genesis-update \
	clean clean-test-e2e \
	fmt \
	test test-unit test-e2e

all: runtime gateway genesis test-client
	@$(ECHO) "$(CYAN)*** Everything built successfully!$(OFF)"

check: check-oasis-core

check-oasis-core:
	@test -x $(OASIS_CORE_ROOT_PATH)/go/oasis-node/oasis-node || ( \
		$(ECHO) "$(RED)error:$(OFF) oasis-node not found in $(OASIS_CORE_ROOT_PATH) (check OASIS_CORE_ROOT_PATH)" && \
		$(ECHO) "       Maybe you need to run \"make download-artifacts\"?" && \
		exit 1 \
	)

download-artifacts:
	@$(ECHO) "$(CYAN)*** Downloading Oasis Core release artifacts...$(OFF)"
	@scripts/download_artifacts.sh "$(OASIS_CORE_ROOT_PATH)"
	@$(ECHO) "$(CYAN)*** Downloading done!$(OFF)"

symlink-artifacts:
	@$(ECHO) "$(CYAN)*** Symlinking runtime build artifacts...$(OFF)"
	@scripts/symlink_artifact.sh "$(RUNTIME_ROOT_PATH)" oasis-ssvm-runtime "$(RUNTIME_CARGO_TARGET_DIR)/debug"
	@$(ECHO) "$(CYAN)*** Symlinking done!$(OFF)"

runtime:
	@$(ECHO) "$(CYAN)*** Building oasis-ssvm-runtime...$(OFF)"
	@CARGO_TARGET_DIR=$(RUNTIME_CARGO_TARGET_DIR) cargo build -p oasis-ssvm-runtime $(EXTRA_BUILD_ARGS)

gateway:
	@$(ECHO) "$(CYAN)*** Building web3-gateway...$(OFF)"
	@CARGO_TARGET_DIR=$(RUNTIME_CARGO_TARGET_DIR) cargo build -p web3-gateway $(EXTRA_BUILD_ARGS) $(CARGO_BUILD_ARGS)

genesis:
	@$(ECHO) "$(CYAN)*** Building genesis utilities...$(OFF)"
	@CARGO_TARGET_DIR=$(RUNTIME_CARGO_TARGET_DIR) cargo build -p genesis $(EXTRA_BUILD_ARGS) $(CARGO_BUILD_ARGS)

test-client:
	@$(ECHO) "$(CYAN)*** Building test-client...$(OFF)"
	@CARGO_TARGET_DIR=$(RUNTIME_CARGO_TARGET_DIR) cargo build -p test-client $(EXTRA_BUILD_ARGS)

test: test-unit test-e2e

test-unit: check-oasis-core
	@$(ECHO) "$(CYAN)*** Running unit tests...$(OFF)"
	@cargo test

test-e2e: check-oasis-core symlink-artifacts
	@$(ECHO) "$(CYAN)*** Running E2E tests...$(OFF)"
	@export OASIS_CORE_ROOT_PATH=$(OASIS_CORE_ROOT_PATH) RUNTIME_CARGO_TARGET_DIR=$(RUNTIME_CARGO_TARGET_DIR) && \
		scripts/test_e2e.sh

fmt:
	@cargo fmt

clean:
	@$(ECHO) "$(CYAN)*** Cleaning up...$(OFF)"
	@cargo clean
