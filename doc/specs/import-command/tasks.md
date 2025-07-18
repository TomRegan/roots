# Import Command Implementation Tasks

## Phase 1: Database Foundation

### Task 1.1: Add Database Dependencies
- [ ] Update `Cargo.toml` to add `rusqlite`, `thiserror`, `walkdir`, `sha2`, and `indicatif` dependencies
- [ ] Update dependency versions to ensure compatibility
- [ ] Run `cargo check` to verify dependencies resolve correctly

_Requirements: 2.2, 8.4_

### Task 1.2: Create Database Schema
- [ ] Create `src/database/schema.rs` with SQL schema definitions
- [ ] Implement schema initialization functions
- [ ] Create migration support for future schema changes
- [ ] Add schema version tracking

_Requirements: 2.2, 8.4_

### Task 1.3: Implement Database Manager
- [ ] Enhance `src/database/mod.rs` with SQLite connection management
- [ ] Implement `DatabaseManager` struct with connection pooling
- [ ] Add transaction support for bulk operations
- [ ] Implement error handling for database operations

_Requirements: 2.2, 8.4_

### Task 1.4: Create Repository Pattern
- [ ] Create `src/database/repositories/mod.rs` for repository pattern
- [ ] Implement `BookRepository` with CRUD operations
- [ ] Implement `AuthorRepository` with relationship management
- [ ] Implement `SubjectRepository` with relationship management
- [ ] Add duplicate detection queries

_Requirements: 2.2, 4.1, 4.2_

## Phase 2: File System Operations

### Task 2.1: Implement Directory Scanner
- [ ] Create `src/filesystem/scanner.rs` for directory traversal
- [ ] Implement recursive directory walking with `walkdir`
- [ ] Add file type filtering for supported formats
- [ ] Implement progress reporting for large directories
- [ ] Add error handling for permission and access issues

_Requirements: 1.2, 1.3, 5.2, 7.1_

### Task 2.2: Enhance File Metadata Extraction
- [ ] Modify `src/application/book/mod.rs` to include file system metadata
- [ ] Add file size and hash calculation
- [ ] Implement file validation and corruption detection
- [ ] Add support for additional metadata fields

_Requirements: 2.1, 2.2, 5.3_

### Task 2.3: Create Library Organization System
- [ ] Create `src/filesystem/organizer.rs` for library management
- [ ] Implement configurable directory structure patterns
- [ ] Add file naming sanitization using existing regex patterns
- [ ] Implement file operations (copy, move, link)
- [ ] Add conflict resolution for duplicate file names

_Requirements: 3.1, 3.2, 3.3, 3.4, 6.4_

## Phase 3: Import Command Implementation

### Task 3.1: Create Import Configuration
- [ ] Extend `src/configuration/mod.rs` with import-specific settings
- [ ] Add configuration parsing for new import options
- [ ] Implement configuration validation
- [ ] Add default configuration values

_Requirements: 6.1, 6.2, 6.3, 6.4, 6.5_

### Task 3.2: Implement Import Command Structure
- [ ] Create `src/application/import/mod.rs` for import functionality
- [ ] Implement `ImportCommand` struct with configuration handling
- [ ] Add import workflow orchestration
- [ ] Implement progress tracking and reporting

_Requirements: 1.1, 7.1, 7.2, 7.3, 7.4_

### Task 3.3: Integrate Metadata Extraction
- [ ] Create `src/application/import/extractor.rs` for metadata handling
- [ ] Integrate existing `EpubLoader` and `MobiLoader`
- [ ] Add error handling for corrupted or invalid files
- [ ] Implement metadata validation and sanitization

_Requirements: 2.1, 2.2, 5.3, 5.4_

### Task 3.4: Implement Duplicate Detection
- [ ] Create `src/application/import/duplicates.rs` for duplicate handling
- [ ] Implement duplicate detection algorithms (ISBN, ASIN, title/author)
- [ ] Add configurable duplicate handling strategies
- [ ] Implement user interaction for duplicate resolution

_Requirements: 4.1, 4.2, 4.3, 4.4_

## Phase 4: Command Line Integration

### Task 4.1: Update CLI Handler
- [ ] Modify `handle_import_command` in `src/interface/cli.rs`
- [ ] Remove placeholder implementation
- [ ] Add import configuration parsing from CLI arguments
- [ ] Implement progress display and user feedback

_Requirements: 1.1, 1.6, 7.1, 7.2_

### Task 4.2: Add Import Command Options
- [ ] Update CLI argument parsing to include import options
- [ ] Add flags for duplicate handling, file actions, and progress reporting
- [ ] Implement help text and usage examples
- [ ] Add validation for command line arguments

_Requirements: 1.1, 6.1, 6.2, 6.3_

### Task 4.3: Implement Progress Reporting
- [ ] Create `src/application/import/progress.rs` for progress tracking
- [ ] Integrate `indicatif` for progress bars and spinners
- [ ] Add time estimation and completion statistics
- [ ] Implement verbose mode for detailed output

_Requirements: 7.1, 7.2, 7.3, 7.4_

## Phase 5: Database Query Integration

### Task 5.1: Update Query Module
- [ ] Modify `src/database/query.rs` to use actual database
- [ ] Implement `list_fields()` with real field enumeration
- [ ] Implement `list_titles()` with database queries
- [ ] Add support for filtering and sorting

_Requirements: 8.1, 8.2, 8.3_

### Task 5.2: Create Query Builder
- [ ] Create `src/database/query_builder.rs` for complex queries
- [ ] Implement filtering by author, title, ISBN, publication date
- [ ] Add sorting and pagination support
- [ ] Implement full-text search capabilities

_Requirements: 8.1, 8.2, 8.3_

### Task 5.3: Update List Command
- [ ] Modify `handle_list_command` in `src/interface/cli.rs`
- [ ] Integrate with new database query functionality
- [ ] Add support for new metadata fields
- [ ] Implement table formatting improvements

_Requirements: 8.1, 8.2_

## Phase 6: Error Handling and Logging

### Task 6.1: Implement Error Types
- [ ] Create `src/application/import/errors.rs` with comprehensive error definitions
- [ ] Implement `thiserror` for error handling
- [ ] Add error context and user-friendly messages
- [ ] Implement error recovery strategies

_Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

### Task 6.2: Add Logging Infrastructure
- [ ] Add logging dependency (`log` and `env_logger`)
- [ ] Implement structured logging throughout import process
- [ ] Add configurable log levels
- [ ] Implement log rotation for long-running imports

_Requirements: 5.1, 5.2, 5.5_

### Task 6.3: Implement Graceful Error Handling
- [ ] Add transaction rollback on critical errors
- [ ] Implement partial failure recovery
- [ ] Add retry logic for transient errors
- [ ] Create error summary reporting

_Requirements: 5.1, 5.2, 5.4, 5.5_

## Phase 7: Testing and Validation

### Task 7.1: Create Unit Tests
- [ ] Write tests for `DatabaseManager` operations
- [ ] Test `DirectoryScanner` with various directory structures
- [ ] Test `MetadataExtractor` with sample files
- [ ] Test `LibraryOrganizer` file naming and organization
- [ ] Test error handling scenarios

_Requirements: All_

### Task 7.2: Create Integration Tests
- [ ] Create test database with sample data
- [ ] Test complete import workflow with sample files
- [ ] Test configuration loading and validation
- [ ] Test CLI argument parsing and validation
- [ ] Test progress reporting functionality

_Requirements: All_

### Task 7.3: Create Performance Tests
- [ ] Test import performance with large file collections
- [ ] Test database query performance
- [ ] Test memory usage during imports
- [ ] Create benchmarks for critical operations

_Requirements: All_

### Task 7.4: Update Test Infrastructure
- [ ] Modify `Makefile` to include new test dependencies
- [ ] Update test data download to include more sample files
- [ ] Add test database cleanup procedures
- [ ] Implement test isolation for parallel execution

_Requirements: All_

## Phase 8: Documentation and Finalization

### Task 8.1: Update Configuration Documentation
- [ ] Document new configuration options in `etc/default.yml`
- [ ] Update `CLAUDE.md` with new import functionality
- [ ] Create configuration examples for different use cases
- [ ] Document database schema and migration procedures

_Requirements: 6.1, 6.2, 6.3, 6.4, 6.5_

### Task 8.2: Update CLI Help and Usage
- [ ] Update command help text with new options
- [ ] Create comprehensive usage examples
- [ ] Update `README.md` with import command documentation
- [ ] Add troubleshooting guide for common issues

_Requirements: 1.1, 1.6, 7.1_

### Task 8.3: Performance Optimization
- [ ] Optimize database queries with proper indexing
- [ ] Implement connection pooling for better performance
- [ ] Add caching for frequently accessed data
- [ ] Optimize file system operations

_Requirements: Performance, Reliability_

### Task 8.4: Security Review
- [ ] Review input validation and sanitization
- [ ] Audit file system operations for security issues
- [ ] Review database operations for SQL injection prevention
- [ ] Test with malicious file inputs

_Requirements: Security_

## Validation Checklist

- [ ] `roots import ~/books/non-fiction/` successfully recurses into subdirectories
- [ ] Books are properly stored in SQLite database with all metadata
- [ ] `roots list` shows imported books immediately
- [ ] `roots fields` shows all available metadata fields
- [ ] Duplicate detection works correctly
- [ ] Library organization follows configured patterns
- [ ] Progress reporting works for large imports
- [ ] Error handling gracefully manages problematic files
- [ ] Configuration options work as documented
- [ ] All tests pass including integration tests