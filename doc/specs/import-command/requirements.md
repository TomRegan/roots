# Import Command Requirements

## Purpose

The import command enables users to build and maintain a personal e-book library by recursively scanning directories for EPUB and MOBI files, extracting metadata, and organizing them in a managed library structure with database-backed indexing.

## User Stories

### 1. Directory Import
**As a user, I want to import e-books from a directory and its subdirectories, so that I can build my library from existing collections.**

**Acceptance Criteria:**
- The command accepts a directory path as input
- The system recursively scans all subdirectories for supported file types
- Only EPUB (.epub) and MOBI (.mobi) files are processed
- The import continues even if some files are corrupted or inaccessible
- Progress is displayed during long-running imports
- A summary is displayed upon completion showing files processed, imported, and skipped

### 2. Metadata Extraction and Storage
**As a user, I want book metadata to be extracted and stored in a database, so that I can query and organize my library.**

**Acceptance Criteria:**
- Metadata is extracted from each supported file format
- Book information includes: title, author(s), publisher, publication date, ISBN, ASIN, description, subjects
- Metadata is stored in a SQLite database for fast querying
- Database schema supports multiple authors and subjects per book
- File path and import timestamp are recorded for each book
- Database is created automatically if it doesn't exist

### 3. Library Organization
**As a user, I want imported books to be organized in a consistent directory structure, so that my library is well-maintained.**

**Acceptance Criteria:**
- Books are copied (not moved) to a managed library directory
- Directory structure follows a configurable pattern (e.g., Author/Title or Genre/Author)
- File names are sanitized to remove invalid characters
- Original file structure is preserved in the database for reference
- Configuration determines whether to move, copy, or link files

### 4. Duplicate Detection
**As a user, I want duplicate books to be detected and handled appropriately, so that my library remains clean.**

**Acceptance Criteria:**
- Duplicates are detected based on ISBN, ASIN, or title/author combination
- When duplicates are found, the user is informed
- Configurable behavior for handling duplicates (skip, replace, keep both)
- Duplicate detection occurs before file operations
- Database maintains information about all file locations for the same book

### 5. Error Handling and Validation
**As a user, I want clear error messages for problematic files, so that I can address issues in my collection.**

**Acceptance Criteria:**
- Corrupted or unreadable files are skipped with informative error messages
- Permission errors are handled gracefully
- Invalid file formats are identified and skipped
- Network issues (if fetching metadata) don't halt the import process
- Partial failures don't prevent other files from being processed
- Error summary is provided at the end of import

### 6. Configuration Support
**As a user, I want to configure import behavior, so that the system works according to my preferences.**

**Acceptance Criteria:**
- Library directory location is configurable
- Import behavior (move/copy/link) is configurable
- Directory structure pattern is configurable
- Duplicate handling behavior is configurable
- File naming conventions are configurable
- Settings are read from the existing YAML configuration system

### 7. Import Status and Progress
**As a user, I want to see import progress and status, so that I know what's happening during long operations.**

**Acceptance Criteria:**
- Progress indicator shows current file being processed
- File count and percentage completion are displayed
- Time estimates are provided for long-running imports
- Final summary includes statistics (files processed, imported, skipped, errors)
- Verbose mode provides detailed information about each file

### 8. Database Integration
**As a user, I want the import system to integrate with existing query functionality, so that imported books are immediately available for search and listing.**

**Acceptance Criteria:**
- Imported books appear in `list` command results
- Metadata fields are available for the `fields` command
- Database schema is compatible with existing query infrastructure
- Database operations are transactional to ensure consistency
- Database can be queried while imports are in progress (read-only)

## Non-Functional Requirements

### Performance
- Import should handle libraries with thousands of books efficiently
- Database operations should be optimized for bulk inserts
- Memory usage should remain reasonable for large imports

### Reliability
- Database transactions ensure data consistency
- Partial failures don't corrupt the database
- System can recover from interrupted imports

### Usability
- Clear progress indication for long-running operations
- Informative error messages guide user action
- Sensible defaults minimize configuration required

### Compatibility
- Works with existing configuration system
- Maintains backward compatibility with current database queries
- Supports both EPUB and MOBI formats as currently implemented