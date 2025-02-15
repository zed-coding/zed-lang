# Contributing to Zed

We love your input! We want to make contributing to Zed as easy and transparent as possible, whether it's:

- Reporting a bug
- Discussing the current state of the code
- Submitting a fix
- Proposing new features
- Becoming a maintainer

## Development Process

We use GitHub to host code, to track issues and feature requests, as well as accept pull requests.

1. Fork the repo and create your branch from `master`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes
5. Make sure your code lints
6. Issue that pull request!

## Any contributions you make will be under the Apache 2.0 License
In short, when you submit code changes, your submissions are understood to be under the same [Apache 2.0 License](LICENSE) that covers the project. Feel free to contact the maintainers if that's a concern.

## Report bugs using GitHub's [issue tracker](https://github.com/zed-coding/zed-lang/issues)
We use GitHub issues to track public bugs. Report a bug by [opening a new issue](https://github.com/zed-coding/zed-lang/issues/new).

## Write bug reports with detail, background, and sample code

**Great Bug Reports** tend to have:

- A quick summary and/or background
- Steps to reproduce
  - Be specific!
  - Give sample code if you can
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening, or stuff you tried that didn't work)

## Development Setup

1. Install prerequisites:
   ```bash
   # Install Rust and Cargo
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Install GNU tools (Ubuntu/Debian)
   sudo apt-get install build-essential
   # Or on macOS
   brew install gcc
   ```

2. Clone and build:
   ```bash
   git clone https://github.com/zed-coding/zed-lang
   cd zed-lang
   make
   ```

3. Run tests (once we have them):
   ```bash
   make test
   ```

## Directory Structure
```
zed-lang/
├── compiler/           # The Zed compiler (zedc)
│   ├── src/           # Compiler source code
│   └── tests/         # Compiler tests
├── zed-build/         # Build system tool
├── vscode/            # VSCode extension
├── examples/          # Example programs
├── docs/             # Documentation
└── tests/            # Integration tests
```

## Coding Style

- Follow the standard Rust formatting guidelines
- Use `rustfmt` to format your code
- Run `clippy` for additional lints:
  ```bash
  cargo clippy --all-targets --all-features
  ```

## Testing

### ⚠️ Important Note
Currently, Zed has no formal test suite. If you're contributing, please help us establish a testing framework! We need:

- Unit tests for the compiler components
- Integration tests for full programs
- Test infrastructure setup
- CI configuration for automated testing

### Areas Needing Tests

1. Lexer
   - Token recognition
   - String literal parsing
   - Comment handling
   - Error conditions

2. Parser
   - AST construction
   - Syntax validation
   - Error recovery
   - Edge cases

3. Code Generation
   - Assembly output verification
   - Function calling conventions
   - Stack management
   - String handling

4. Integration
   - Full program compilation
   - Build system functionality
   - Error messages
   - Edge cases

### Proposed Test Structure
```
tests/
├── compiler/
│   ├── lexer/           # Lexer unit tests
│   ├── parser/          # Parser unit tests
│   └── codegen/         # Code generation tests
├── integration/         # Full program tests
└── examples/           # Example programs that should compile
```

If you're interested in helping establish the test framework, please:
1. Open an issue to discuss testing approaches
2. Propose a testing framework (e.g., Rust's built-in testing, custom framework)
3. Submit a PR with initial test infrastructure

## Documentation

- Update documentation for any changed functionality
- Add documentation for new features
- Ensure code examples in documentation work
- Update [docs/docs.md](docs/docs.md) if necessary

## Making a Pull Request

1. Update your fork to the latest upstream:
   ```bash
   git remote add upstream https://github.com/zed-coding/zed-lang.git
   git fetch upstream
   git checkout master
   git merge upstream/master
   ```

2. Create a feature branch:
   ```bash
   git checkout -b feature-name
   ```

3. Commit your changes:
   ```bash
   git commit -m "Description of changes"
   ```

4. Push to your fork:
   ```bash
   git push origin feature-name
   ```

5. Open a Pull Request from your `feature-name` branch to our `master` branch

Your PR should:
- Include a description of changes
- Link any relevant issues
- Update documentation as needed
- Add or update tests (once we have them)
- Pass all CI checks (once we have them)

## Community

- Be welcoming to newcomers
- Be respectful of differing viewpoints and experiences
- Accept constructive criticism gracefully
- Focus on what is best for the community

## License

By contributing, you agree that your contributions will be licensed under the Apache License, Version 2.0.
