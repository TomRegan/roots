# Import Command Design

## Overview

The import command will be redesigned to provide a complete e-book library management solution. It will recursively scan directories for EPUB and MOBI files, extract metadata, store it in a SQLite database, and optionally organize files in a managed library structure.

## Architecture

### High-Level Flow

```
Directory Path Input
        ↓
File Discovery (Recursive)
        ↓
File Validation & Type Detection
        ↓
Metadata Extraction
        ↓
Duplicate Detection
        ↓
Database Storage
        ↓
File Organization (Optional)
        ↓
Progress Reporting & Summary
```

### Component Hierarchy

```
ImportCommand
├── DirectoryScanner
│   ├── FileWalker
│   └── FileFilter
├── MetadataExtractor
│   ├── EpubExtractor (existing)
│   └── MobiExtractor (existing)
├── DatabaseManager
│   ├── BookRepository
│   ├── AuthorRepository
│   └── SubjectRepository
├── LibraryOrganizer
│   ├── FileNamer
│   └── DirectoryStructure
└── ProgressReporter
```

## Data Models

### Database Schema

```sql
-- Books table
CREATE TABLE books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    publisher TEXT,
    publication_date TEXT,
    imprint TEXT,
    description TEXT,
    asin TEXT,
    isbn TEXT,
    file_path TEXT NOT NULL,
    file_size INTEGER,
    file_hash TEXT,
    imported_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Authors table
CREATE TABLE authors (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

-- Book-Author relationship
CREATE TABLE book_authors (
    book_id INTEGER,
    author_id INTEGER,
    PRIMARY KEY (book_id, author_id),
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES authors(id) ON DELETE CASCADE
);

-- Subjects table
CREATE TABLE subjects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

-- Book-Subject relationship
CREATE TABLE book_subjects (
    book_id INTEGER,
    subject_id INTEGER,
    PRIMARY KEY (book_id, subject_id),
    FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
    FOREIGN KEY (subject_id) REFERENCES subjects(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX idx_books_title ON books(title);
CREATE INDEX idx_books_isbn ON books(isbn);
CREATE INDEX idx_books_asin ON books(asin);
CREATE INDEX idx_books_imported_at ON books(imported_at);
CREATE INDEX idx_authors_name ON authors(name);
CREATE INDEX idx_subjects_name ON subjects(name);
```

### Rust Data Structures

```rust
// Enhanced Book struct
#[derive(Debug, Clone)]
pub struct Book {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub authors: Vec<String>,
    pub publisher: Option<String>,
    pub publication_date: Option<DateTime<Utc>>,
    pub imprint: Option<String>,
    pub description: Option<String>,
    pub subjects: Vec<String>,
    pub asin: Option<String>,
    pub isbn: Option<String>,
    pub file_path: PathBuf,
    pub file_size: Option<u64>,
    pub file_hash: Option<String>,
    pub imported_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Import configuration
#[derive(Debug, Clone)]
pub struct ImportConfig {
    pub source_directory: PathBuf,
    pub library_directory: PathBuf,
    pub file_action: FileAction,
    pub duplicate_handling: DuplicateHandling,
    pub directory_structure: DirectoryStructure,
    pub progress_reporting: bool,
}

#[derive(Debug, Clone)]
pub enum FileAction {
    Copy,
    Move,
    Link,
    None, // Import metadata only
}

#[derive(Debug, Clone)]
pub enum DuplicateHandling {
    Skip,
    Replace,
    KeepBoth,
    Ask,
}

#[derive(Debug, Clone)]
pub enum DirectoryStructure {
    AuthorTitle,
    GenreAuthor,
    YearAuthor,
    Flat,
}

// Import result tracking
#[derive(Debug, Default)]
pub struct ImportResult {
    pub files_processed: usize,
    pub files_imported: usize,
    pub files_skipped: usize,
    pub files_failed: usize,
    pub duplicates_found: usize,
    pub errors: Vec<ImportError>,
}

#[derive(Debug)]
pub struct ImportError {
    pub file_path: PathBuf,
    pub error_type: ErrorType,
    pub message: String,
}

#[derive(Debug)]
pub enum ErrorType {
    FileNotFound,
    PermissionDenied,
    CorruptedFile,
    UnsupportedFormat,
    MetadataExtractionFailed,
    DatabaseError,
    FileSystemError,
}
```

## Components and Interfaces

### ImportCommand

```rust
pub struct ImportCommand {
    config: ImportConfig,
    db_manager: DatabaseManager,
    scanner: DirectoryScanner,
    extractor: MetadataExtractor,
    organizer: LibraryOrganizer,
    progress_reporter: ProgressReporter,
}

impl ImportCommand {
    pub fn new(config: ImportConfig) -> Result<Self, ImportError>;
    pub fn execute(&mut self) -> Result<ImportResult, ImportError>;
    fn process_file(&mut self, file_path: &Path) -> Result<(), ImportError>;
    fn handle_duplicate(&mut self, book: &Book) -> Result<DuplicateAction, ImportError>;
}
```

### DirectoryScanner

```rust
pub struct DirectoryScanner {
    supported_extensions: HashSet<String>,
}

impl DirectoryScanner {
    pub fn new() -> Self;
    pub fn scan_directory(&self, path: &Path) -> Result<Vec<PathBuf>, ImportError>;
    fn is_supported_file(&self, path: &Path) -> bool;
}
```

### MetadataExtractor

```rust
pub struct MetadataExtractor;

impl MetadataExtractor {
    pub fn extract_metadata(&self, file_path: &Path) -> Result<Book, ImportError>;
    fn calculate_file_hash(&self, file_path: &Path) -> Result<String, ImportError>;
}
```

### DatabaseManager

```rust
pub struct DatabaseManager {
    connection: Connection,
}

impl DatabaseManager {
    pub fn new(db_path: &Path) -> Result<Self, ImportError>;
    pub fn initialize_schema(&self) -> Result<(), ImportError>;
    pub fn insert_book(&self, book: &Book) -> Result<i64, ImportError>;
    pub fn find_duplicates(&self, book: &Book) -> Result<Vec<Book>, ImportError>;
    pub fn transaction<F, R>(&self, f: F) -> Result<R, ImportError>
    where
        F: FnOnce(&Transaction) -> Result<R, ImportError>;
}
```

### LibraryOrganizer

```rust
pub struct LibraryOrganizer {
    config: ImportConfig,
}

impl LibraryOrganizer {
    pub fn new(config: ImportConfig) -> Self;
    pub fn organize_file(&self, book: &Book) -> Result<PathBuf, ImportError>;
    fn generate_file_path(&self, book: &Book) -> Result<PathBuf, ImportError>;
    fn sanitize_filename(&self, name: &str) -> String;
}
```

## Error Handling

### Error Strategy

1. **Graceful Degradation**: Individual file failures don't stop the import process
2. **Detailed Logging**: All errors are logged with context for debugging
3. **User-Friendly Messages**: Clear error messages guide user action
4. **Rollback Support**: Database transactions ensure consistency
5. **Recovery**: System can resume interrupted imports

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum ImportError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),
    
    #[error("Metadata extraction failed: {0}")]
    MetadataExtraction(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Duplicate handling error: {0}")]
    DuplicateHandling(String),
}
```

## Testing Strategy

### Unit Tests

- **MetadataExtractor**: Test extraction from various file formats
- **DatabaseManager**: Test CRUD operations and schema creation
- **DirectoryScanner**: Test recursive directory traversal
- **LibraryOrganizer**: Test file naming and directory structure generation
- **ImportCommand**: Test end-to-end import flow with mocked dependencies

### Integration Tests

- **Database Integration**: Test with actual SQLite database
- **File System Integration**: Test with real directory structures
- **Configuration Integration**: Test with various configuration options
- **Error Scenarios**: Test handling of corrupted files, permission errors

### Test Data

- Sample EPUB and MOBI files with known metadata
- Directory structures with various nesting levels
- Corrupted files for error testing
- Files with special characters in names
- Files with duplicate metadata

### Performance Tests

- **Large Collections**: Test with thousands of books
- **Memory Usage**: Monitor memory consumption during imports
- **Database Performance**: Test query performance with large datasets
- **File System Performance**: Test with various file system types

## Dependencies

### New Dependencies

```toml
[dependencies]
rusqlite = { version = "0.29", features = ["bundled"] }
thiserror = "1.0"
walkdir = "2.3"
sha2 = "0.10"
indicatif = "0.17"
```

### Existing Dependencies Used

- `epub`: For EPUB metadata extraction
- `mobi`: For MOBI metadata extraction
- `config`: For configuration management
- `chrono`: For date/time handling
- `serde`: For configuration serialization

## Configuration Integration

### Enhanced Configuration

```yaml
# etc/default.yml additions
database:
  path: "library.db"
  auto_create: true
  
import:
  file_action: "copy"  # copy, move, link, none
  duplicate_handling: "skip"  # skip, replace, keep_both, ask
  directory_structure: "author_title"  # author_title, genre_author, year_author, flat
  progress_reporting: true
  calculate_hash: true
  
library:
  directory: "~/Books"
  auto_organize: true
  
scanning:
  recursive: true
  follow_symlinks: false
  supported_extensions: [".epub", ".mobi"]
```

## Performance Considerations

1. **Batch Database Operations**: Use transactions for bulk inserts
2. **Lazy Loading**: Only extract metadata when needed
3. **Memory Management**: Process files sequentially to limit memory usage
4. **Database Indexing**: Optimize queries with appropriate indexes
5. **File System Optimization**: Use efficient directory traversal
6. **Progress Reporting**: Minimize overhead of progress updates

## Security Considerations

1. **Path Validation**: Prevent directory traversal attacks
2. **File Size Limits**: Prevent DoS through large file processing
3. **Input Sanitization**: Sanitize metadata before database insertion
4. **Permission Checks**: Verify file and directory permissions
5. **Database Security**: Use parameterized queries to prevent SQL injection