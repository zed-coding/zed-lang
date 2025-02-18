# Zed Programming Language

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

## Overview

Zed is a systems programming language that compiles directly to x86-64 assembly. It focuses on simplicity, low-level control, and minimal runtime overhead while providing a comfortable syntax for systems programming.

## Key Features

- Direct compilation to x86-64 assembly
- Zero runtime overhead
- First-class inline assembly support
- C-like syntax with modern conveniences
- Static typing
- Comprehensive standard library
- Built-in build system

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
- `puts(str)`: Write string to stdout
- `putchar(c)`: Write character to stdout
- `println(str)`: Write string with newline

#### Math Operations (`std/math.zed`)
- `abs(x)`: Absolute value
- `min(a, b)`: Minimum of two numbers
- `max(a, b)`: Maximum of two numbers

#### String Operations (`std/string.zed`)
- `strlen(str)`: Get string length
- `strcpy(dest, src)`: Copy string
- `strcat(dest, src)`: Concatenate strings
- `strcmp(s1, s2)`: Compare strings

#### System Operations (`std/sys.zed`)
- `exit(code)`: Exit program
- `sleep(seconds)`: Sleep for seconds
- `getpid()`: Get process ID
- `time()`: Get system time

#### Memory Operations (`std/memory.zed`)
- `memcpy(dest, src, n)`: Copy memory
- `memset(ptr, value, n)`: Set memory
- `malloc(size)`: Allocate memory
- `free(ptr, size)`: Free memory

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
// Function declaration
fn my_function(param1, param2) {
    return param1 + param2;
}

// Function predeclaration
fn complex_function();

// Implementation
fn complex_function() {
    // code
}
```

### Inline Assembly

Zed provides powerful inline assembly support with operand constraints:

```zed
fn syscall(number) {
    asm "movq %rdi, %rax    # syscall number
         syscall"
    :                       # outputs
    : "r"[number]          # inputs
    : "rax";               # clobbers
}
```

### Arrays and Memory

```zed
// Array operations
buffer[0] = 65;  // ASCII 'A'
value = buffer[0];

// Memory allocation
ptr = malloc(1024);
free(ptr, 1024);
```

## Build System

Zed includes a robust build system (`zed`) with the following commands:

```bash
# Create new project
zed new project-name

# Build project
zed build
zed build --release

# Run project
zed run
zed run --release

# Clean build artifacts
zed clean

# Install/update standard library
zed install-std
```

## Project Structure

A typical Zed project looks like:

```
my-project/
├── src/
│   └── main.zed
├── examples/
├── target/
├── zed.json
└── .gitignore
```

## Installation

### Prerequisites
- Rust toolchain (1.70.0+)
- GNU Assembler (as)
- GNU Linker (ld)

### Build from Source

```bash
git clone https://github.com/zed-coding/zed-lang.git
cd zed-lang
cargo build --release
cargo install --path .
```

### Standard Library Installation

The standard library is automatically installed in `~/.zed-lang/std/version/1.0.0/` when creating a new project. You can manually install or update it:

```bash
zed install-std
```

## VS Code Extension

Zed includes syntax highlighting support for Visual Studio Code. Features include:

- Syntax highlighting for:
  - Keywords
  - Functions
  - Strings
  - Numbers
  - Comments
  - Inline assembly
- Bracket matching
- Auto-closing pairs
- Comment toggling

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

## Author

Voltaged (VoltagedDebunked)  
Email: rusindanilo@gmail.com

## License

Apache License 2.0
