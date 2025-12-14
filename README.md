# Codebase - Semantic Code Search Tool

A fast, offline semantic code search tool powered by tree-sitter that lets you search for code patterns using natural language queries.

## Features

- **Natural Language Queries**: Search using plain English like "find functions", "show structs", "find enums"
- **Semantic Search**: Find code by what it does, not just by pattern matching (use `-i` flag)
- **Multi-Language Support**: Rust, Python, JavaScript, TypeScript, C, and C++
- **Ranked Results**: Relevance scoring for semantic searches
- **Context Lines**: View surrounding code with the `--context` option
- **Syntax-Aware**: Uses tree-sitter for accurate parsing, not just text matching
- **Fast**: Recursively scans directories and filters by file extension
- **Colored Output**: Beautiful terminal output with syntax highlighting

## Installation

### Option 1: Install from crates.io (Recommended)

```bash
cargo install codebase
```

This installs the `codebase` command globally on your system.

### Option 2: Build from Source

```bash
git clone https://github.com/yourusername/codebase.git
cd codebase
cargo build --release
```

The binary will be at `target/release/codebase.exe` (Windows) or `target/release/codebase` (Linux/Mac).

To install globally:
```bash
cargo install --path .
```

### Option 3: Download Pre-built Binary

Download the latest release for your platform from [GitHub Releases](https://github.com/yourusername/codebase/releases):
- Windows: `codebase-x86_64-pc-windows-msvc.zip`
- macOS: `codebase-x86_64-apple-darwin.tar.gz`
- Linux: `codebase-x86_64-unknown-linux-gnu.tar.gz`

Extract and add to your PATH.

## Usage

### Basic Search (Pattern Mode)
```bash
# Find all functions in current directory
codebase "find functions"

# Find structs in a specific directory
codebase "find structs" --path src

# Search only Rust files
codebase "find functions" --ext rs

# Search multiple file types
codebase "find classes" --ext "py,js,ts"
```

### Semantic Search (Implementation Mode)
```bash
# Find functions that parse files (ranked by relevance)
codebase "parse file tree-sitter" --path src --ext rs -i

# Show why each function matched
codebase "authentication validate user" --ext py -i --verbose

# Search for specific functionality across languages
codebase "handle http requests" --ext "js,ts,py" -i
```

Semantic mode (`-i` flag) searches by **what the code does**, not just patterns:
- Extracts function names and bodies
- Scores matches based on keyword relevance
- Ranks results by score (highest first)
- Shows match reasons in verbose mode

### Context Lines
```bash
# Show 2 lines of context before and after each match
codebase "find structs" --context 2

# Short form
codebase "find impl" -C 3
```

### Supported Query Patterns

#### Rust
- **Functions**: "find functions", "find fn", "find methods"
- **Structs**: "find structs"
- **Enums**: "find enums"
- **Traits**: "find traits", "find interface"
- **Implementations**: "find impl"
- **Constants**: "find const"
- **Imports**: "find import", "find use"

#### Python
- **Functions**: "find functions", "find def"
- **Classes**: "find classes"
- **Decorators**: "find decorators"
- **Imports**: "find imports"

#### JavaScript
- **Functions**: "find functions"
- **Arrow Functions**: "find arrow"
- **Classes**: "find classes"
- **Variables**: "find var", "find let", "find const"
- **Imports/Exports**: "find import", "find export"

#### TypeScript
- **Functions**: "find functions"
- **Classes**: "find classes"
- **Interfaces**: "find interface"
- **Type Aliases**: "find type"
- **Enums**: "find enum"
- **Imports/Exports**: "find import", "find export"

#### C/C++
- **Functions**: "find functions"
- **Structs**: "find structs"
- **Classes** (C++): "find classes"
- **Enums**: "find enums"
- **Typedefs** (C): "find typedef"
- **Namespaces** (C++): "find namespace"
- **Templates** (C++): "find template"

## Examples

```bash
# Find all Rust implementations with context
$ codebase "find impl" --ext rs -C 2

src\parser.rs

  20:1
  19
  20 → impl SupportedLanguage {
  21       /// Detect language from file extension
  22       pub fn from_extension(ext: &str) -> Option<Self> {
```

```bash
# Find all TypeScript interfaces
$ codebase "find interface" --ext ts

src/types.ts
  12:1 → interface UserConfig {
  45:1 → interface DatabaseConnection {
```

## Command Line Options

- `<QUERY>`: Natural language query (required)
- `-p, --path <PATH>`: Path to search (default: current directory)
- `-e, --ext <EXT>`: Filter by file extensions (e.g., "rs,py,js")
- `-C, --context <N>`: Show N lines of context around matches
- `-v, --verbose`: Show verbose output including parse errors
- `-h, --help`: Show help information

## How It Works

1. **File Walking**: Recursively scans the specified directory
2. **Language Detection**: Identifies language from file extension
3. **Tree-sitter Parsing**: Parses files into Abstract Syntax Trees (AST)
4. **Pattern Matching**: Maps your natural language query to tree-sitter queries
5. **Results**: Displays matches with file paths, line numbers, and optional context

## Supported Languages & Extensions

| Language   | Extensions                      |
|------------|---------------------------------|
| Rust       | `.rs`                           |
| Python     | `.py`                           |
| JavaScript | `.js`, `.jsx`                   |
| TypeScript | `.ts`, `.tsx`                   |
| C          | `.c`, `.h`                      |
| C++        | `.cpp`, `.hpp`, `.cc`, `.cxx`, `.hh` |

## License

This project uses tree-sitter and various tree-sitter language parsers.
