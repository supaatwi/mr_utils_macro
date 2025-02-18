# MR Utils Macro

A collection of practical derive macros for Rust to enhance struct functionality and reduce boilerplate code.

## Features

### ToVec Derive Macro
Automatically implements a `to_vec` method for structs that converts selected or all fields into a vector of strings. Useful for data conversion, serialization, and integration with other systems.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mr_utils_macro = "0.1.0"
```

## Feature Roadmap
- [x] ToVec derive macro
- [ ] FromRow derive macro (planned)
- [ ] ToMap derive macro (planned)
- [ ] Custom field attributes support (planned)

## License

MIT

## Contributing

Feel free to:
- Submit bug reports and feature requests
- Submit pull requests
- Propose new utility macros