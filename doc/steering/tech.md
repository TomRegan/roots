# Technology Stack

## Framework

**Rust** - Systems programming language providing memory safety, performance, and reliability. Built as a native command-line application using Rust's standard library and ecosystem.

## Core Libraries

* **`clap`** - Command-line argument parsing with subcommands and help generation
* **`config`** - Hierarchical configuration management with YAML support
* **`epub`** - EPUB format parsing and metadata extraction
* **`mobi`** - MOBI format parsing via mobi-rs library
* **`reqwest`** - HTTP client for external metadata API calls
* **`serde`** - Serialization/deserialization for JSON and YAML data

## Data Management

* **Configuration**: YAML-based configuration with environment variable overrides
* **Metadata Storage**: Embedded database for book metadata and query operations
* **File Processing**: Pattern-based file type detection and format-specific parsers

## Testing

* **`assert_cmd`** - Integration testing of CLI commands and outputs
* **`rstest`** - Parameterized testing framework for Rust
* **Test Strategy**: Integration tests verify CLI behavior, sample files from Project Gutenberg

## Build System

* **Cargo** - Rust's built-in package manager and build tool
* **Makefile** - Convenience wrapper for common development tasks
* **Commands**: `make compile`, `make test`, `make install`, `make clean`

## Architecture

* **Layered Architecture**: Clear separation between interface, application, and infrastructure layers
* **Command Pattern**: Enumerated commands with centralized dispatch logic
* **Modular Design**: Feature-based module organization (book, database, filesystem, internet)
* **Configuration-First**: Externalized configuration with sensible defaults
* **Error Handling**: Graceful degradation and informative error messages
* **CLI-Native**: Designed specifically for command-line usage patterns

## External Integration

* **Web APIs**: Optional metadata enrichment from online book databases
* **File System**: Directory traversal and file import capabilities
* **Cross-Platform**: Support for macOS, Linux, and Windows environments