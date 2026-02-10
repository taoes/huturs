# Huturs

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

## Usage

### String Operations

```rust
use huturs_core::string;

fn main() {
    // Check if string is empty
    let empty = string::is_empty("");
    
    // Convert to uppercase
    let upper = string::to_uppercase("hello");
    
    // Reverse string
    let reversed = string::reverse("rust");
    
    // Split string
    let parts = string::split("a,b,c", ",");
}
```

### Date/Time Operations

```rust
use huturs_core::date;

fn main() {
    // Get current timestamp in seconds
    let timestamp = date::current_timestamp();
    
    // Get current timestamp in milliseconds
    let timestamp_ms = date::current_timestamp_millis();
    
    // Check if timestamp is in the future
    let is_future = date::is_future(timestamp + 1000);
    
    // Calculate difference between two timestamps
    let timestamp1 = date::current_timestamp();
    let timestamp2 = timestamp1 - 3600; // 1 hour ago
    let diff = date::diff_seconds(timestamp1, timestamp2);
    
    // Add seconds to timestamp
    let future_time = date::add_seconds(timestamp, 3600);
    
    // Get days from timestamp
    let days = date::get_days(timestamp);
}
```

### Math Operations

```rust
use huturs_core::math;

fn main() {
    // Basic operations
    let sum = math::add(5, 3);
    let product = math::multiply(4, 6);
    
    // Array operations
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let avg = math::average(&numbers);
    let max_val = math::max_in_array(&numbers);
    
    // Check parity
    let is_even = math::is_even(4);
}
```

## Modules

### `string`
- Empty and blank checks
- Case conversion (uppercase/lowercase)
- Trimming operations
- String reversal
- Pattern matching (contains, starts_with, ends_with)
- String replacement and splitting
- String joining and repetition

### `date`
- Current timestamp retrieval (seconds/milliseconds)
- Timestamp formatting
- Time difference calculations
- Future/past timestamp checks
- Time arithmetic (add/subtract seconds)
- Time unit conversions (minutes, hours, days)

### `math`
- Basic arithmetic operations (add, subtract, multiply, divide)
- Absolute value calculations
- Max/min values
- Power operations (square, cube, power)
- Parity checks (even/odd)
- Array operations (sum, average, max, min)

## Project Structure

```
huturs/
├── huturs-core/       # Core library
│   ├── src/
│   │   ├── string/    # String utilities
│   │   ├── date/      # Date/time utilities
│   │   └── math/      # Math utilities
│   └── tests/         # Integration tests
├── example/           # Example usage
└── Cargo.toml         # Workspace configuration
```

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
