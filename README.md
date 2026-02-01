# Load!64

A Commodore 64 loader utility that converts binary files into BASIC programs, making it easy to load custom machine code on your C64.

## Overview

Load!64 helps you create C64 BASIC programs that contain embedded binary data. These programs can be loaded on a Commodore 64 (or emulator) and will automatically load your binary data into the correct memory location.

This is the initial "Hello World" skeleton implementation. Future versions will add the core binary conversion functionality.

## Documentation

For detailed information about the project architecture, design principles, and tech stack, please visit the [GitHub Wiki](https://github.com/cressie176/Load64/wiki):

- [Home](https://github.com/cressie176/Load64/wiki)
- [Design Principles](https://github.com/cressie176/Load64/wiki/Design-Principles)
- [Tech Stack](https://github.com/cressie176/Load64/wiki/Tech-Stack)

## Prerequisites

### Required Dependencies

1. **Rust and Cargo**

   Install Rust using rustup:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   Or visit [https://rustup.rs/](https://rustup.rs/) for more installation options.

2. **System Dependencies (for iced GUI)**

   - **macOS**: No additional dependencies needed
   - **Linux**: Install development libraries
     ```bash
     # Ubuntu/Debian
     sudo apt-get install libxkbcommon-dev libwayland-dev

     # Fedora
     sudo dnf install libxkbcommon-devel wayland-devel
     ```
   - **Windows**: No additional dependencies needed

### Optional Dependencies

- **VICE Emulator** (for testing generated C64 programs)

  Download from [https://vice-emu.sourceforge.io/](https://vice-emu.sourceforge.io/)

  Or install via package manager:
  ```bash
  # macOS
  brew install vice

  # Ubuntu/Debian
  sudo apt-get install vice

  # Fedora
  sudo dnf install vice
  ```

## Developer Commands

### Running the Application
```bash
cargo run
```
Launches the GUI application displaying "Hello Load!64"

### Running Tests
```bash
cargo test
```
Runs all unit and integration tests

### Formatting Code
```bash
cargo fmt
```
Formats all Rust code according to project style

### Checking Formatting
```bash
cargo fmt --check
```
Checks if code is properly formatted without making changes

### Running Linter
```bash
cargo clippy
```
Runs the Clippy linter to catch common mistakes and suggest improvements

### Running Linter Strictly
```bash
cargo clippy -- -D warnings
```
Runs Clippy and treats all warnings as errors (used in CI)

### Auto-fixing Linter Issues
```bash
cargo clippy --fix --allow-dirty --allow-staged -- -D warnings
```
Automatically fixes clippy issues where possible (used in pre-commit hook)

### Building Release Binary
```bash
cargo build --release
```
Creates an optimized release build in `target/release/load64`

### All Checks (like CI)
```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo test
```
Runs all quality checks that are executed in CI/CD

## Development Workflow

This project uses pre-commit hooks to ensure code quality. When you commit changes, the following steps run automatically:

1. Auto-fix code formatting (`cargo fmt`)
2. Auto-fix clippy issues where possible (`cargo clippy --fix`)
3. Restage fixed files (`git add -u`)
4. Run tests (`cargo test`)

The hook automatically fixes formatting and linting issues, then restages the changes. If tests fail, the commit is blocked and you'll need to fix the issues manually.

### Setting Up Pre-commit Hook

The pre-commit hook is located at `.git/hooks/pre-commit` and should be executable. If you cloned the repository, you may need to make it executable:

```bash
chmod +x .git/hooks/pre-commit
```

## Project Structure

```
Load64/
├── src/
│   └── main.rs           # Application entry point
├── .claude/
│   └── rules/            # Guidelines for AI agents
├── .github/
│   └── workflows/
│       └── ci.yml        # GitHub Actions CI/CD
├── Cargo.toml            # Rust project manifest
├── rustfmt.toml          # Formatting configuration
└── README.md             # This file
```

## CI/CD

This project uses GitHub Actions for continuous integration. On every push and pull request, the following checks run:

1. **Format Check**: Ensures code is properly formatted
2. **Clippy Linting**: Catches common mistakes and enforces best practices
3. **Tests**: Runs all unit and integration tests
4. **Build**: Verifies the project compiles successfully

## Contributing

This project follows strict design principles:

- Small, explicit, easy-to-reason-about code
- Clean domain model with clear boundaries
- Pure testable logic separated from infrastructure
- Test-driven development (TDD)
- Fail fast and loud with clear error messages

Please read the [Design Principles](https://github.com/cressie176/Load64/wiki/Design-Principles) wiki page before contributing.

## License

TBD

## Links

- [GitHub Repository](https://github.com/cressie176/Load64)
- [GitHub Wiki](https://github.com/cressie176/Load64/wiki)
- [Issue Tracker](https://github.com/cressie176/Load64/issues)
