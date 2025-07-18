# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`roots` is a Rust-based e-book library manager that handles both EPUB and MOBI formats. It provides a command-line interface for importing, organizing, and querying e-book collections with metadata support.

## Development Commands

### Build and Test
- **Build**: `cargo build` or `make compile`
- **Release build**: `cargo build --release` or `make release`
- **Run tests**: `RUST_BACKTRACE=full cargo test` or `make test`
- **Clean**: `cargo clean` or `make clean`

### Installation
- **Install locally**: `cargo install --path .` or `make install`
- **Uninstall**: `cargo uninstall` or `make remove`

### Test Dependencies
The test suite requires sample e-book files which are automatically downloaded:
- `make test` will download test files to `var/cache/` if they don't exist
- Test files include `pg98.mobi` and `pg98.epub` from Project Gutenberg

## Architecture

### Core Structure
- **Entry Point**: `src/main.rs` - Simple entry point that initializes configuration and CLI
- **CLI Interface**: `src/interface/cli.rs` - Command-line argument parsing and command dispatch using clap
- **Command Processing**: `src/application/command.rs` - Enum defining all available commands
- **Configuration**: `src/configuration/mod.rs` - YAML-based configuration system with environment variable support

### Key Modules
- **application/book/**: Core book handling with support for EPUB and MOBI formats
- **database/**: Database operations for book metadata storage and queries
- **filesystem/**: File system operations for book import/export
- **internet/**: Web-based metadata fetching from external APIs

### Command Structure
The CLI supports these main commands:
- `config` - Display configuration settings and paths
- `fields` - List queryable metadata fields
- `find` - Search for book metadata online
- `import` - Import e-books from directories
- `info` - Display book information with optional web metadata fetching
- `list` - Query the library with various filters
- `update` - Synchronize filesystem with database

### Configuration System
- Default config location: `~/.config/roots/default.yml`
- Environment variables: Prefix with `ROOTS_` (e.g., `ROOTS_DEBUG`)
- Key settings include library directory, import behavior, and external API keys
- Uses `config` crate for hierarchical configuration merging

### Book Processing
- Supports EPUB and MOBI formats via respective crates
- Extracts metadata including title, author, publisher, ISBN, ASIN
- Handles both local file analysis and web-based metadata enhancement
- Uses pattern matching on file extensions to determine processing strategy

### Database Integration
- Uses embedded database for metadata storage
- Provides query capabilities for listing titles and available fields
- Handles cases where database is not initialized gracefully

## Testing Strategy

Tests use `assert_cmd` for integration testing of the CLI interface. Key test patterns:
- Command success/failure validation
- Output content verification
- Error handling for missing files/arguments
- Configuration conflict detection