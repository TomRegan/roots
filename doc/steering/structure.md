# Project Structure

## Root Directory

* **`Cargo.toml`** - Rust package manifest with dependencies and metadata
* **`Makefile`** - Build automation with convenience commands
* **`README.md`** - Project overview and usage instructions
* **`LICENSE.md`** - Software license terms
* **`CLAUDE.md`** - Development guidance for AI assistants

## Source Code (`src/`)

* **`main.rs`** - Application entry point and initialization
* **`interface/`** - User interface layer
  * **`cli.rs`** - Command-line argument parsing using clap
  * **`mod.rs`** - Interface module definitions
* **`application/`** - Core business logic
  * **`command.rs`** - Command enumeration and dispatch
  * **`files.rs`** - File system operations
  * **`book/`** - Book-specific functionality
    * **`file.rs`** - File format detection and handling
    * **`loader.rs`** - Book loading and parsing
    * **`mod.rs`** - Book module definitions
* **`configuration/`** - Configuration management
  * **`mod.rs`** - YAML-based config with environment variable support
* **`database/`** - Data persistence layer
  * **`mod.rs`** - Database connection and operations
  * **`query.rs`** - Query building and execution
* **`filesystem/`** - File system utilities
  * **`mod.rs`** - Directory traversal and file operations
* **`internet/`** - External API integration
  * **`mod.rs`** - Web-based metadata fetching

## Configuration (`etc/`)

* **`default.yml`** - Default configuration template

## Documentation (`doc/`)

* **`generate-specs.md`** - Documentation generation instructions
* **`generate-steering-rules.md`** - Project steering rules template
* **`product.md`** - Product vision and principles
* **`structure.md`** - This file - project structure documentation
* **`tech.md`** - Technology stack and architecture

## Runtime Data (`var/`)

* **`cache/`** - Downloaded test files and temporary data
  * **`pg98.epub`** - Project Gutenberg test file
  * **`pg98.mobi`** - Project Gutenberg test file

## Build Artifacts (`target/`)

* Rust compilation output (not tracked in version control)

## Development Files

* **`Brewfile`** - macOS development dependencies
* **`roots.iml`** - IntelliJ IDEA project file
* **`share/`** - Sample data files for testing and development