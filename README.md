# Zed Programming Language

[![GitHub contributors](https://img.shields.io/github/contributors/zed-coding/zed-lang.svg)](https://github.com/zed-coding/zed-lang/graphs/contributors)
[![GitHub stars](https://img.shields.io/github/stars/zed-coding/zed-lang.svg)](https://github.com/zed-coding/zed-lang/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/zed-coding/zed-lang.svg)](https://github.com/zed-coding/zed-lang/issues)
[![GitHub license](https://img.shields.io/github/license/zed-coding/zed-lang.svg)](https://github.com/zed-coding/zed-lang/blob/main/LICENSE)
[![Latest Release](https://img.shields.io/github/v/release/zed-coding/zed-lang)](https://github.com/zed-coding/zed-lang/releases)
![Build Status](https://img.shields.io/github/actions/workflow/status/zed-coding/zed-lang/build.yml)

A simple programming language that compiles to x86_64 assembly, featuring functions, control flow, strings, and basic arithmetic operations.

## Quick Start

```bash
# Clone the repository
git clone https://github.com/zed-coding/zed-lang
cd zed-lang

# Build everything
make

# Create a new project
zed new my-project
cd my-project

# Write some code in src/main.zed
println("Hello from Zed!\n");

# Build and run
zed run
```

## Features

- Simple and clean syntax
- Functions with recursion support
- Control flow (if/else, while)
- String literals with escape sequences
- 64-bit integer arithmetic
- Top-level code execution
- Detailed error messages

## Example

```zed
fn factorial(n) {
    if (n < 2) {
        return 1;
    }
    return n * factorial(n - 1);
}

println("Calculating factorial of 5:\n");
result = factorial(5);
println(result);
println("\n");
```

## Documentation

For detailed information about the language, build system, and tools, see [docs/docs.md](docs/docs.md).

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details on how to:
- Build from source
- Submit changes
- Report issues
- Add features
- Improve documentation

## Prerequisites

- Rust compiler and Cargo
- GNU Assembler (as)
- GNU Linker (ld)

## VS Code Extension

Install our VS Code extension for:
- Syntax highlighting
- Bracket matching
- Auto-closing pairs
- Comment toggling
- Multi-line comment support

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.

Copyright 2024 The Zed Programming Language Authors.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you shall be licensed under the Apache License, Version 2.0, without any additional terms or conditions.

## Project Status

Zed is currently in active development. While it's stable enough for learning and experimentation, we recommend against using it in production environments at this time.

## Author

Voltaged (VoltagedDebunked)
Email: rusindanilo@gmail.com
