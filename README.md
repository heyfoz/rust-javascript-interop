# Rust Yew Framework Example with JavaScript Interop

## Project Overview

This project is a basic Rust client-side web application using the Yew framework. It focuses on JavaScript-Rust interop for session ID retrieval, demonstrating how to retrieve a session ID from the URL using both inline JavaScript and an external JavaScript file. NOTE: Dependencies and packages may change over time.

## Features

- Rust Yew Framework for WebAssembly web applications
- Inline JavaScript integration using `wasm_bindgen`
- External JavaScript module import
- Routing with `yew_router`

## Prerequisites

Before starting, ensure you have installed:
- [Rust and Cargo](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Trunk](https://trunkrs.dev/#install) (optional, for building and bundling)
- [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/) (for development on Windows)
  
## Project Structure

- `/src/main.rs`: Main entry point for the Yew application
- `/session.js`: External JavaScript file for session ID retrieval

## Dependencies

Add these and any other relevant dependencies in your `Cargo.toml`:

```toml
[package]
name = "yew_javascript_interop_example"
version = "0.1.0"
edition = "2021"

[dependencies]
yew = "0.19"
yew-router = "0.16"
wasm-bindgen = "0.2"
js-sys = "0.3"
stylist = "0.10"

[lib]
crate-type = ["cdylib", "rlib"]
```

## Build and Run

To build and run the application:

1. Navigate to the project directory.
2. Use `trunk serve` (if using Trunk) or `wasm-pack build` (if using wasm-pack).

## Contributing

Contributions are welcome. Follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or fix.
3. Commit changes.
4. Push to your branch.
5. Open a pull request.

## License

This project is under the [MIT License](LICENSE).



