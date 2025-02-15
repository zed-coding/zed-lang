# The Zed Programming Language

## Table of Contents
- [Introduction](#introduction)
- [Installation](#installation)
- [Language Features](#language-features)
- [Build System](#build-system)
- [VS Code Extension](#vs-code-extension)
- [Example Programs](#example-programs)

## Introduction

Zed is a simple programming language that compiles to x86_64 assembly. It features functions, control flow, strings, and basic arithmetic operations.

## Installation

### Prerequisites
- Rust compiler and Cargo
- GNU Assembler (as)
- GNU Linker (ld)

### Building from Source
```bash
# Clone the repository
git clone https://github.com/zed-lang/zed-lang
cd zed-lang

# Build everything

make
```

## Language Features

### Basic Syntax

#### Comments
```rust
// Single line comment

/* Multi-line
   comment */
```

#### Variables and Assignment
```rust
x = 10;
name = "John";
```

#### Functions
```rust
fn add(a, b) {
    return a + b;
}

fn factorial(n) {
    if (n < 2) {
        return 1;
    }
    return n * factorial(n - 1);
}
```

#### Control Flow
```rust
// If statement
if (condition) {
    printLn("True");
} else {
    printLn("False");
}

// While loop
while (condition) {
    // Loop body
}
```

#### Built-in Functions
```rust
printLn("Hello, World!");  // Print with newline
```

#### Operators
- Arithmetic: `+`, `-`, `*`, `/`
- Comparison: `<`, `>`, `<=`, `>=`, `==`
- Assignment: `=`

### Type System
Zed currently supports:
- Integers (64-bit)
- Strings
- Functions

## Build System

The Zed build system is written in Rust and provides project management and build automation.

### Commands

#### Creating a New Project
```bash
zed new project-name
```

Creates a new project with the following structure:
```
project-name/
├── src/
│   └── main.zed
├── examples/
├── target/
│   ├── debug/
│   └── release/
├── zed.json
└── .gitignore
```

#### Building
```bash
zed build           # Debug build
zed build --release # Release build
```

#### Running
```bash
zed run           # Build and run in debug mode
zed run --release # Build and run in release mode
```

#### Cleaning
```bash
zed clean  # Remove build artifacts
```

### Project Configuration
The `zed.json` file contains project settings:
```json
{
    "name": "project-name", // Project Name
    "version": "0.1.0", // Version
    "target": "main" // Name of the executable
}
```

## VS Code Extension

### Features
- Syntax highlighting
- Bracket matching
- Auto-closing pairs
- Comment toggling
- Multi-line comment support

### Installation
1. Open VS Code
2. Press Ctrl+Shift+P (Cmd+Shift+P on Mac)
3. Type "Install from VSIX"
4. Select the zed-language.vsix file

### Language Configuration
The extension provides:
- Syntax highlighting for:
  - Keywords (if, else, while, fn, return)
  - Built-in functions (printLn)
  - Strings with escape sequences
  - Numbers
  - Comments (both // and /* */)
  - Function names
  - Operators

## Example Programs

### Hello World
```rust
// No main function needed - code executes from top to bottom
printLn("Hello, World!");
```

### Factorial Calculator
```rust
fn factorial(n) {
    if (n < 2) {
        return 1;
    }
    return n * factorial(n - 1);
}

// Program execution starts here
printLn("Calculating factorial of 5:");
result = factorial(5);
printLn(result);
```

### Complex Example
```rust
/* Fibonacci sequence with loop
   and prime number checking */

fn fib_loop(n) {
    if (n < 2) {
        return n;
    }
    
    a = 0;
    b = 1;
    i = 2;
    
    while (i <= n) {
        temp = b;
        b = a + b;
        a = temp;
        i = i + 1;
    }
    
    return b;
}

fn is_prime(n) {
    if (n < 2) {
        return 0;
    }
    if (n == 2) {
        return 1;
    }
    
    i = 2;
    while (i * i <= n) {
        if ((n / i) * i == n) {
            return 0;
        }
        i = i + 1;
    }
    return 1;
}

// Program execution starts at the top level
printLn("Finding prime Fibonacci numbers:");
count = 0;
i = 2;

while (count < 5) {
    fib = fib_loop(i);
    if (is_prime(fib)) {
        printLn(fib);
        count = count + 1;
    }
    i = i + 1;
}
```

## Language Execution Model

### Program Structure
- Zed programs execute statements sequentially from top to bottom
- No explicit main() function is required
- Function declarations can appear anywhere in the file
- All top-level statements are executed in order
- Functions are only executed when called

### Entry Point
Unlike languages like C, Java, or Rust, Zed does not use a main() function as an entry point. Instead:
- The program starts executing from the first statement in the file
- Function declarations are registered but not executed
- Top-level statements form the program's main execution path

## Implementation Details

### Compiler Pipeline
1. Lexical Analysis: Converts source code into tokens
2. Parsing: Builds Abstract Syntax Tree (AST)
3. Code Generation: Generates x86_64 assembly
4. Assembly: Uses GNU assembler (as)
5. Linking: Uses GNU linker (ld)

### Assembly Generation
The compiler generates position-independent x86_64 assembly code with:
- Proper function calling conventions
- Stack frame management
- String handling
- Arithmetic operations
- Control flow structures

### Error Handling
The compiler provides detailed error messages with:
- Line and column information
- Error context
- Helpful suggestions