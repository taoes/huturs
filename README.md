
[🇨🇳 中文](README_CN.md)
[🇬🇧 English](README.md)
---
# HutuRs

A comprehensive Rust utility library providing common tools for string manipulation, date/time operations, and mathematical calculations.

## Overview

Huturs is a collection of reusable utility functions designed to simplify common programming tasks in Rust. It provides a clean, efficient API for everyday operations.

## Features

- **String Utilities**: Common string operations including trimming, case conversion, searching, and more
- **Date/Time Utilities**: Timestamp handling, date formatting, and time calculations
- **Math Utilities**: Basic mathematical operations, array calculations, and numeric utilities

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
huturs-core = "0.1.0"
```

## Documentation

For detailed API documentation and usage examples, please visit:

📚 [https://taoes.github.io/huturs/huturs_core/index.html](https://taoes.github.io/huturs/huturs_core/index.html)

## Development

### Build

```bash
cargo build
```

### Run Tests

```bash
# Run all tests
cargo test

# Or use makefile
make test
```

### Clean

```bash
cargo clean
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Status

This project is currently in active development. Version 0.1.0 is the initial release with core functionality.

## Roadmap

- [ ] Add more string manipulation utilities
- [ ] Enhance date/time formatting options
- [ ] Add advanced mathematical functions
- [ ] Implement file I/O utilities
- [ ] Add comprehensive documentation and examples

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests to ensure everything works (`cargo test`)
5. Commit your changes (`git commit -m 'Add some amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Development Guidelines

- Write clear, readable code with proper documentation
- Add tests for new features
- Follow Rust best practices and conventions
- Keep functions focused and simple