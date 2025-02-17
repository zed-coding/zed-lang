# The Zed Programming Language

## Table of Contents
- [Introduction](#introduction)
- [Installation](#installation)
- [Language Features](#language-features)
  - [Basic Syntax](#basic-syntax)
  - [Functions](#functions)
  - [Function Predeclarations](#function-predeclarations)
  - [Includes and Modules](#includes-and-modules)
  - [Control Flow](#control-flow)
  - [Type System](#type-system)
  - [Standard Library](#standard-library)
- [Build System](#build-system)
- [VS Code Extension](#vs-code-extension)
- [Example Programs](#example-programs)
- [Language Execution Model](#language-execution-model)
- [Implementation Details](#implementation-details)

## Introduction

Zed is a simple programming language that compiles to x86_64 assembly. It features functions, control flow, strings, basic arithmetic operations, a module system with includes, and a standard library providing common functionality. With predeclarations support and proper symbol resolution, it enables advanced programming patterns like mutual recursion, forward references, and modular code organization.

## Installation

### Prerequisites
- Rust compiler and Cargo
- GNU Assembler (as)
- GNU Linker (ld)

### Building from Source
```bash
# Clone the repository
git clone https://github.com/zed-coding/zed-lang
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

#### Operators
- Arithmetic: `+`, `-`, `*`, `/`
- Comparison: `<`, `>`, `<=`, `>=`, `==`
- Assignment: `=`

### Functions

Functions in Zed can be predeclared and defined separately, enabling more flexible code organization and mutual recursion.

#### Basic Function Definition
```rust
fn add(a, b) {
    return a + b;
}

// Usage
@include "./std/io.zed";
result = add(5, 3);
println(result);
```

### Function Predeclarations

Predeclarations enable:
- Using functions before they're defined
- Mutual recursion between functions
- Better code organization
- Breaking circular dependencies

Rules for predeclarations:
- Function signature in predeclaration must match definition
- All predeclared functions must be defined exactly once
- Functions can be called after predeclaration but before definition
- Parameter names in predeclarations are optional
- Predeclarations end with a semicolon (;)

#### Example:
```rust
fn factorial(n);  // Predeclare factorial

@include "./std/io.zed";
println("Calculating factorial:");
result = factorial(5);
println(result);

fn factorial(n) {
    if (n < 2) {
        return 1;
    }
    return n * factorial(n - 1);
}
```

### Includes and Modules

Zed supports a module system through includes, allowing code to be split across multiple files.

#### Include Syntax
```rust
@include "module.zed";  // Include relative to current file
@include "utils.zed" from "./lib";  // Include from specific directory
```

#### Symbol Resolution
- Each file can define functions and variables
- Functions defined in one file can be used in files that include it
- Circular includes are detected and prevented
- src/main.zed is treated specially as the entry point
- Other files are treated as libraries

#### Example Module Structure
```
project/
├── src/
│   ├── main.zed         # Entry point (gets _start symbol)
│   ├── math.zed         # Math utilities
│   └── string_utils.zed # String utilities
└── lib/
    └── core.zed         # Core functionality
```

#### Include Rules
1. Files can only be included once (duplicates are ignored)
2. Circular includes are detected and cause compilation error
3. Each file maintains its own scope for variables
4. Functions are available after include statement
5. Function conflicts are detected at link time

#### Example Module Usage
```rust
// math.zed
fn add(a, b) {
    return a + b;
}

// main.zed
@include "./std/io.zed";
@include "math.zed";

result = add(5, 3);
println(result);
```

### Control Flow

#### If Statement
```rust
if (condition) {
    println("True");
} else {
    println("False");
}
```

#### While Loop
```rust
while (condition) {
    // Loop body
}
```

### Type System
Zed currently supports:
- Integers (64-bit)
- Strings
- Functions

### Standard Library

Zed includes a standard library providing essential functionality. The standard library is organized into modules:

#### Standard Library Structure
```
std/
├── io.zed    # Input/output functions (println, etc.)
├── math.zed  # Mathematical operations
└── fs.zed    # Filesystem operations
```

#### Available Modules

##### std/io.zed
```rust
// Include with: @include "./std/io.zed";
fn println(format, ...);  // Print with newline
fn puts(str);            // Print string with newline
fn putchar(c);           // Print single character
```

##### std/math.zed
```rust
// Include with: @include "./std/math.zed";
fn abs(x);       // Absolute value
fn min(a, b);    // Minimum of two numbers
fn max(a, b);    // Maximum of two numbers
```

##### std/fs.zed
```rust
// Include with: @include "./std/fs.zed";
fn open(path, flags);   // Open file
fn close(fd);          // Close file
fn read(fd, buf, len); // Read from file
```

#### Using the Standard Library
```rust
// Include io.zed to use println
@include "./std/io.zed";
println("Hello %s\n", "World");

// Include math.zed for math functions
@include "./std/math.zed";
x = abs(-42);
println("Absolute value of %d is %d\n", x, abs_x);
```

## Build System

### Project Structure
```
project-name/
├── src/
│   ├── main.zed    # Entry point (gets _start)
│   └── lib.zed     # Library code
├── std/           # Standard library
│   ├── io.zed     # I/O functions
│   ├── math.zed   # Math functions
│   └── fs.zed     # Filesystem functions
├── examples/
├── target/
│   ├── debug/
│   └── release/
├── zed.json
└── .gitignore
```

### Build Process
1. Compiles all .zed files in src/
2. src/main.zed gets _start symbol and program entry point
3. Other files are compiled as libraries
4. Linker combines all object files
5. Produces final executable

### Commands
```bash
zed build           # Debug build
zed build --release # Release build
zed run            # Build and run
zed clean          # Remove build artifacts
```

## Implementation Details

### Compiler Pipeline
1. Lexical Analysis
   - Converts source code into tokens
   - Handles includes and module resolution
   - Processes comments and whitespace

2. Parsing
   - Processes function predeclarations
   - Handles includes recursively
   - Builds Abstract Syntax Tree (AST)
   - Tracks declared and defined functions
   - Validates predeclarations and definitions

3. Code Generation
   - Generates x86_64 assembly
   - Main file gets _start symbol
   - Library files only generate function code
   - Handles forward references
   - Manages symbol visibility

4. Assembly and Linking
   - Assembles each file separately
   - Links all object files
   - Resolves function references
   - Produces executable binary

### Symbol Resolution
- Each source file is compiled to separate object file
- src/main.zed gets _start symbol for program entry
- Other files only generate function definitions
- Functions are globally visible after definition
- Symbol conflicts are detected at link time
- Duplicate function definitions cause link error
- All predeclared functions must be defined once

### Error Handling

#### Include Errors
```
error: circular include detected: 'math.zed'
  --> main.zed:1:1
1 | @include "math.zed";
  | ^^^^^^^^^^^^^^^^^^

error: couldn't read 'utils.zed': No such file
  --> main.zed:2:1
2 | @include "utils.zed";
  | ^^^^^^^^^^^^^^^^^^
```

#### Symbol Errors
```
error: multiple definition of 'add'
  --> lib.zed:5:1
5 | fn add(a, b) {
  | ^^^^^^^^^^^

error: undefined reference to 'calculate'
  --> main.zed:3:1
3 | result = calculate(5);
  |          ^^^^^^^^
```

### Memory Model
- Stack-based memory management
- No heap allocation
- Function parameters passed on stack
- Local variables stored in stack frame
- No global variables (yet)

## Example Programs

### Hello World
```rust
@include "./std/io.zed";
println("Hello, World!");
```

### Factorial with Predeclaration
```rust
@include "./std/io.zed";

fn factorial(n);  // Predeclare factorial

println("Calculating factorial of 5:");
result = factorial(5);
println(result);

fn factorial(n) {
    if (n < 2) {
        return 1;
    }
    return n * factorial(n - 1);
}
```

### Using Multiple Stdlib Modules
```rust
@include "./std/io.zed";
@include "./std/math.zed";

x = -42;
abs_x = abs(x);
println("Absolute value of %d is %d", x, abs_x);
```
