all: help

var/cache:
	mkdir -p var/cache

var/cache/pg98.mobi: var/cache
	curl --output var/cache/pg98.mobi https://www.gutenberg.org/cache/epub/98/pg98.mobi

var/cache/pg98.epub: var/cache
	curl --output var/cache/pg98.epub https://www.gutenberg.org/cache/epub/98/pg98.epub

.PHONY: compile
compile: ## compile sources
	cargo build

.PHONY: release
release: ## compile and optimize
	cargo build --release

.PHONY: test
test: compile var/cache/pg98.mobi var/cache/pg98.epub ## run the tests
	RUST_BACKTRACE=full cargo test

.PHONY: remove
remove: ## remove the binary
	cargo uninstall

.PHONY: install
install: ## install the binary
	cargo install --path .

.PHONY: clean
clean: ## remove the target directory
	cargo clean
	$(RM) -rf var/

.PHONY: help
help:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z0-9_-]+:.*?## / {gsub("\\\\n",sprintf("\n%22c",""), $$2);printf "\033[92m%-10s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
