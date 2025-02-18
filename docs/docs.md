# Zed Programming Language

[![GitHub contributors](https://img.shields.io/github/contributors/zed-coding/zed-lang.svg)](https://github.com/zed-coding/zed-lang/graphs/contributors)
[![GitHub stars](https://img.shields.io/github/stars/zed-coding/zed-lang.svg)](https://github.com/zed-coding/zed-lang/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/zed-coding/zed-lang.svg)](https://github.com/zed-coding/zed-lang/issues)
[![GitHub license](https://img.shields.io/github/license/zed-coding/zed-lang.svg)](https://github.com/zed-coding/zed-lang/blob/main/LICENSE)
[![Latest Release](https://img.shields.io/github/v/release/zed-coding/zed-lang)](https://github.com/zed-coding/zed-lang/releases)
![Build Status](https://img.shields.io/github/actions/workflow/status/zed-coding/zed-lang/build.yml)

## Overview

Zed is a systems programming language focusing on simplicity, performance, and low-level control. Designed to compile directly to x86-64 assembly, Zed provides developers with a straightforward yet powerful programming experience.

## Key Features

- Direct x86-64 assembly compilation
- Minimal runtime overhead
- Inline assembly support
- Simple, C-like syntax
- Statically typed
- Lightweight standard library

## Language Fundamentals

### Syntax

```zed
// Single-line comment
/* Multi-line
   comment */

// Include standard library
@include <std/io.zed>;
@include <std/math.zed>;
```

## Standard Library

### Current Implementations

#### I/O Library (`std/io.zed`)
- `puts(str)`: Write a string to standard output
- `putchar(c)`: Write a single character to standard output
- `println(str)`: Write a string with a newline

#### Math Library (`std/math.zed`)
- `abs(x)`: Calculate absolute value
- `min(a, b)`: Return minimum of two numbers
- `max(a, b)`: Return maximum of two numbers

## Data Types

### Primitive Types
- 64-bit signed integers
- Strings (basic support)

## Control Structures

```zed
// Conditional statement
if (x > 10) {
    println("Greater than 10");
} else {
    println("Less than or equal to 10");
}

// While loop
i = 0;
while (i < 10) {
    println(i);
    i = i + 1;
}
```

## Function Declaration

```zed
// Function with return
fn add(a, b) {
    return a + b;
}

// Predeclaration
fn complex_function();
fn complex_function() {
    // Implementation
}
```

## Inline Assembly

```zed
fn custom_syscall(call_number, arg1, arg2, arg3) {
    asm "syscall" 
    : "=r"[result]
    : "r"[call_number], "r"[arg1], "r"[arg2], "r"[arg3]
    : "rcx", "r11";
    return result;
}
```

## Compilation Process

1. Lexical Analysis
2. Syntactic Parsing
3. Abstract Syntax Tree Generation
4. x86-64 Assembly Code Generation
5. Assembly and Linking

## Roadmap

### Planned Improvements
- Expanded type system
- More standard library functions
- Improved error handling
- Optimization techniques

## Installation

### Prerequisites
- Rust (1.70.0+)
- GNU Assembler (as)
- GNU Linker (ld)

### Build from Source
```bash
git clone https://github.com/zed-coding/zed-lang.git
cd zed-lang
cargo build --release
cargo install --path .
```

## Contributing

Contributions are welcome. Please:
1. Fork the repository
2. Create a feature branch
3. Commit changes
4. Push to the branch
5. Create a pull request

## License

Apache 2.0

## Author

Voltaged (VoltagedDebunked)
Email: rusindanilo@gmail.com

## Acknowledgments

Thanks to the open-source community and early contributors.
