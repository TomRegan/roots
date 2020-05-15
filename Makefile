all: help

.PHONY: compile
compile: ## compile sources
	@cargo build

.PHONY: release
release: ## compile and optimize
	@cargo build --release

.PHONY: test
test: ## run the tests
	@RUST_BACKTRACE=full cargo test

.PHONY: remove
remove: ## remove the binary
	@cargo uninstall

.PHONY: install
install: ## install the binary
	@cargo install --path .

.PHONY: clean
clean: ## remove the target directory
	@cargo clean

.PHONY: help
help:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z0-9_-]+:.*?## / {gsub("\\\\n",sprintf("\n%22c",""), $$2);printf "\033[92m%-10s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
