# Zed Programming Language

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

## Overview

Zed is a systems programming language that compiles directly to x86-64 assembly. It focuses on simplicity, low-level control, and minimal runtime overhead while providing a comfortable syntax for systems programming.

## Key Features

- Direct compilation to x86-64 assembly
- Zero runtime overhead
- First-class inline assembly support
- C-like syntax with modern conveniences
- Minimal but powerful standard library
- Integrated build system
- VSCode integration with syntax highlighting
- Built-in documentation generator
- Code formatting tool

## Language Fundamentals

### Basic Syntax

```zed
// Single-line comments
/* Multi-line
   comments */

// Include standard library
@include <std/io.zed>;

// Include local file
@include "mylib.zed";

// Function declaration
fn add(a, b) {
    return a + b;
}

// Variables
x = 42;
str = "Hello, Zed!";
```

### Standard Library

The standard library is organized into modules:

#### I/O Operations (`std/io.zed`)
- `puts(str)`: Write raw string to stdout
- `putchar(c)`: Write single character
- `print_number(n)`: Print numeric value
- `println(x)`: Print string with newline
- `vprintln(x)`: Print numeric value with newline

#### Math Operations (`std/math.zed`)
- `abs(x)`: Absolute value
- `min(a, b)`: Minimum of two numbers
- `max(a, b)`: Maximum of two numbers

#### String Operations (`std/string.zed`)
- `strlen(str)`: Get string length
- `strcpy(dest, src)`: Copy string with null termination
- `strcat(dest, src)`: Concatenate strings
- `strcmp(s1, s2)`: Compare strings

#### System Operations (`std/sys.zed`)
- `exit(code)`: Exit program with status code
- `sleep(seconds)`: Sleep for specified seconds
- `getpid()`: Get process ID
- `time()`: Get system time

#### Memory Operations (`std/memory.zed`)
- `memcpy(dest, src, n)`: Copy n bytes of memory
- `memset(ptr, value, n)`: Set n bytes to value
- `malloc(size)`: Allocate memory
- `free(ptr, size)`: Free allocated memory

### Control Flow

```zed
// If statement
if (condition) {
    // code
} else {
    // code
}

// While loop
while (condition) {
    // code
}
```

### Functions

```zed
// Function declaration with implementation
fn add(a, b) {
    return a + b;
}

// Function predeclaration
fn complex_function();

// Later implementation
fn complex_function() {
    // Implementation
}
```

### Inline Assembly

Zed provides comprehensive inline assembly support with full constraint specifications:

```zed
fn example() {
    asm "movq %rdi, %rax    # Move input to rax
         addq $1, %rax      # Add 1
         ret"               # Return value in rax
    : "=r"[result]         # Output constraints
    : "r"[input]           # Input constraints
    : "rax";               # Clobber list
}
```

Supported constraint types:
- `r`: Register constraint
- `=r`: Output register constraint
- Memory clobbers: `"memory"`
- Condition codes: `"cc"`

### Arrays and Memory Management

```zed
// Basic array operations
buffer[0] = 65;  // Store byte
value = buffer[0];  // Load byte

// Dynamic memory allocation
ptr = malloc(1024);  // Allocate 1024 bytes
memset(ptr, 0, 1024);  // Zero memory
free(ptr, 1024);  // Free memory
```

## Development Tools

### Build System

The Zed build system (`zed`) provides the following commands:

```bash
# Create new project
zed new project-name

# Build project
zed build
zed build --release  # With optimizations

# Run project
zed run
zed run --release    # Run optimized build

# Clean build artifacts
zed clean

# Install/update standard library
zed install-std
```

### Documentation Generator (zed-docgen)

The `zed-docgen` tool generates beautiful HTML documentation from Zed source files:

```bash
# Generate docs for a single file
zed-docgen input.zed -o docs/

# Generate docs for an entire project
zed-docgen src/ -o docs/ --title "My Project"

# Include private functions
zed-docgen src/ -o docs/ --private
```

Features:
- Markdown support in documentation comments
- Syntax highlighted code blocks
- Search functionality
- Public/private function visibility
- Function grouping and navigation
- Responsive design
- Print-friendly styling

### Code Formatter (zed-fmt)

The `zed-fmt` tool formats Zed code according to consistent style rules:

```bash
# Format a single file
zed-fmt file.zed

# Format and write changes
zed-fmt --write file.zed

# Check formatting only (useful for CI)
zed-fmt --check src/

# Format with custom settings
zed-fmt --indent 2 --max-width 80 src/
```

Formatting rules:
- Consistent indentation
- Operator spacing
- Line length limits
- Comment preservation
- Special handling for inline assembly
- Empty line management

### Package Manager (zed-pkg)

Zed provides a robust package management system through `zed-pkg`, allowing easy installation, publishing, and management of packages.

### Package Registry

The Zed package registry hosts community-created packages, enabling simple sharing and reuse of code.

### Installing Packages

```bash
# Install a package
zed-pkg install package_name

# Install a specific version
zed-pkg install package_name --version 1.0.0
```

Packages are automatically installed into `src/pkg/package_name.zed`.

### Managing Packages

```bash
# List installed packages
zed-pkg list

# Remove a package
zed-pkg remove package_name
```

### Publishing Packages

To publish a package:
1. Ensure your project has a `zed.json` with package metadata
2. Run `zed-pkg publish`

### Package Structure

- Packages are stored in `src/pkg/`
- Each package has a `.zed` file for code
- Metadata is stored in a companion `.json` file

### Best Practices
- Use semantic versioning
- Include clear documentation
- Keep packages focused and modular

## Project Structure

A typical Zed project has the following structure:

```
project/
├── src/
│   └── main.zed    # Entry point
├── examples/       # Example code
├── docs/          # Generated documentation
├── target/        # Build outputs
│   ├── debug/
│   └── release/
├── zed.json       # Project configuration
└── .gitignore
```

### Project Configuration

The `zed.json` file contains project metadata:

```json
{
  "name": "project-name",
  "version": "0.1.0",
  "target": "main"
}
```

## Memory Alignment

Zed provides fine-grained control over memory alignment through the `@align` directive. This is crucial for:
- SIMD operations requiring aligned memory access
- Cache-line optimization
- Hardware requirements
- DMA and device interactions

#### Basic Usage

```zed
// Align variable allocation
@align(16)
buffer = malloc(1024);

// Align function stack frame
@align(16)
fn process_vectors(data) {
    // Function body...
}
```

#### Alignment Rules

- Alignment must be a positive power of 2 (2, 4, 8, 16, 32, etc.)
- Multiple align directives are not allowed on the same declaration
- Alignment applies to the immediately following declaration or definition

#### Use Cases

1. **SIMD Operations**
```zed
@align(16)
fn vector_add(a, b) {
    asm "vmovdqu %rdi, %xmm0
         vmovdqu %rsi, %xmm1
         vpaddd  %xmm0, %xmm1, %xmm0";
}
```

2. **Cache Line Optimization**
```zed
@align(64)  // Common cache line size
buffer = malloc(256);
```

3. **Hardware Requirements**
```zed
@align(4096)  // Page size alignment
page = malloc(4096);
```

The assembler ensures proper alignment by padding as needed. The stack is automatically realigned when entering aligned functions to maintain the specified alignment requirements.

## Installation

### Prerequisites
- Rust toolchain (2021 edition or later)
- GNU Assembler (as)
- GNU Linker (ld)

### Building from Source

```bash
git clone https://github.com/zed-coding/zed-lang.git
cd zed-lang

# Build everything
make
```

### Standard Library Installation

The standard library is automatically installed to `~/.zed-lang/std/version/1.0.0/` when creating a new project. Manual installation:

```bash
zed install-std
```

## VS Code Extension

The Zed VS Code extension provides:

### Syntax Highlighting
- Keywords and control flow
- Functions and variables
- Strings and numbers
- Comments (single-line and block)
- Inline assembly with register highlighting
- Include directives

### Editor Features
- Bracket matching and auto-closing
- Comment toggling (Ctrl+/)
- Scope awareness
- Custom theme optimized for Zed

### Configuration
The extension includes:
- Language configuration for proper editing behavior
- Dark theme optimized for Zed syntax
- Full TextMate grammar for accurate highlighting

## Language Internals

### Compilation Process
1. Lexical analysis (lexer.rs)
2. Parsing and AST generation (parser.rs)
3. Code generation to x86-64 (codegen.rs)
4. Assembly and linking via GNU tools

### Error Handling
- Detailed error messages with source location
- Syntax and semantic error detection
- Color-coded error output

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

### Development Setup
1. Install Rust and required tools
2. Clone repository
3. Build compiler and tools
4. Install VS Code extension (optional)

## License

This project is licensed under the Apache License 2.0.

## Author

Voltaged (VoltagedDebunked)  
Email: rusindanilo@gmail.com
