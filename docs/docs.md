# The Zed Programming Language

## Table of Contents
- [Introduction](#introduction)
- [Installation](#installation)
- [Language Features](#language-features)
  - [Basic Syntax](#basic-syntax)
  - [Functions](#functions)
  - [Function Predeclarations](#function-predeclarations)
  - [Control Flow](#control-flow)
  - [Type System](#type-system)
- [Build System](#build-system)
- [VS Code Extension](#vs-code-extension)
- [Example Programs](#example-programs)
- [Language Execution Model](#language-execution-model)
- [Implementation Details](#implementation-details)

## Introduction

Zed is a simple programming language that compiles to x86_64 assembly. It features functions, control flow, strings, and basic arithmetic operations. With predeclarations support, it enables advanced programming patterns like mutual recursion and forward references.

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
result = add(5, 3);
println(result);
```

#### Function Predeclarations
```rust
// Function predeclaration
fn calculate_sum(a, b);

// Can use the function before its definition
result = calculate_sum(5, 3);

// Function definition
fn calculate_sum(a, b) {
    return a + b;
}
```

#### Recursive Functions with Predeclarations
```rust
fn factorial(n);  // Predeclare factorial

fn factorial(n) {
    if (n < 2) {
        return 1;
    }
    return n * factorial(n - 1);
}
```

#### Mutual Recursion
```rust
// Predeclare both functions
fn is_even(n);
fn is_odd(n);

fn is_even(n) {
    if (n == 0) {
        return 1;
    }
    return is_odd(n - 1);
}

fn is_odd(n) {
    if (n == 0) {
        return 0;
    }
    return is_even(n - 1);
}
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

## Build System

The Zed build system provides project management and build automation.

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
    "name": "project-name",
    "version": "0.1.0",
    "target": "main"
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
The extension provides syntax highlighting for:
- Keywords (if, else, while, fn, return)
- Built-in functions (println)
- Strings with escape sequences
- Numbers
- Comments (both // and /* */)
- Function names
- Operators

## Example Programs

### Hello World
```rust
println("Hello, World!");
```

### Factorial with Predeclaration
```rust
fn factorial(n);  // Predeclare factorial

// Can use factorial before its definition
println("Calculating factorial of 5:");
result = factorial(5);
println(result);

// Define factorial
fn factorial(n) {
    if (n < 2) {
        return 1;
    }
    return n * factorial(n - 1);
}
```

### Mutual Recursion Example
```rust
// Predeclare functions for mutual recursion
fn is_even(n);
fn is_odd(n);

// Calculate if numbers 1-5 are even
i = 1;
while (i <= 5) {
    if (is_even(i)) {
        println("Even: ");
    } else {
        println("Odd: ");
    }
    println(i);
    i = i + 1;
}

// Define the mutually recursive functions
fn is_even(n) {
    if (n == 0) return 1;
    return is_odd(n - 1);
}

fn is_odd(n) {
    if (n == 0) return 0;
    return is_even(n - 1);
}
```

## Language Execution Model

### Program Structure
- Zed programs execute statements sequentially from top to bottom
- No explicit main() function is required
- Function declarations and predeclarations can appear anywhere
- Functions are registered (both declarations and predeclarations) during parsing
- All top-level statements are executed in order
- Functions are only executed when called

### Entry Point
Unlike languages like C or Java, Zed:
- Starts executing from the first statement in the file
- Registers function declarations and predeclarations
- Executes top-level statements in order
- Validates all predeclarations are defined
- Reports errors for undefined or multiply defined functions

## Implementation Details

### Compiler Pipeline
1. Lexical Analysis
   - Converts source code into tokens
   - Handles comments and whitespace
   - Recognizes function predeclaration syntax

2. Parsing
   - Processes function predeclarations
   - Builds Abstract Syntax Tree (AST)
   - Tracks declared and defined functions
   - Validates predeclarations and definitions match
   - Ensures all predeclared functions are defined

3. Code Generation
   - Generates x86_64 assembly
   - Predeclarations don't generate code
   - Only function definitions generate assembly
   - Handles forward references in function calls

4. Assembly and Linking
   - Uses GNU assembler (as)
   - Uses GNU linker (ld)
   - Produces executable binary

### Error Handling
The compiler provides detailed error messages for:

#### Predeclaration Errors
```
error: function 'calculate_sum' declared but not defined
  --> main.zed:1:1
1 | fn calculate_sum(a, b);
  | ^^^^^^^^^^^^^^^^^^^^^

error: function 'factorial' is already defined
  --> main.zed:10:1
10 | fn factorial(n) {
   | ^^^^^^^^^^^^^
```

#### General Syntax Errors
```
error: unexpected token in expression: '}'
  --> main.zed:5:1
5 | }
  | ^

error: undefined variable 'x'
  --> main.zed:3:5
3 | if (x > 0) {
  |     ^
```

### Memory Model
- Stack-based memory management
- No heap allocation
- Function parameters passed on stack
- Local variables stored in stack frame
- No global variables (yet)
