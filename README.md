# Rust Yew Framework Example with JavaScript Interop

## Project Overview

This Rust-based, multilingual (EN, ES), client-side web application is developed using the Yew framework. It demonstrates JavaScript-Rust interoperability for session ID retrieval from URLs, employing both inline and external JavaScript methods. This represents a modern approach to web app development, leveraging both Rust and JavaScript.

The project also features embedded CSS within Rust, showcasing front-end styling capabilities in the Rust ecosystem. 

First, the project is compiled using cargo build, which compiles the Rust code and its dependencies into WebAssembly (wasm). 

Next, the trunk serve command is used to build and bundle the project. This step creates a dist folder containing the WebAssembly (package_name-unique_hash_bg.wasm) and JavaScript (package_name-unique_hash.js) files, packaging everything needed for deployment. 

NOTE: Dependencies and packages may evolve over time.

## Features

- Atomic design component structure: atoms, molecules, organisms
- Inline JavaScript integration using [wasm_bindgen](https://docs.rs/wasm-bindgen/latest/wasm_bindgen/)
- External JavaScript module import in [main.rs](https://github.com/ffm5113/rust_javascript_interop/blob/main/src/main.rs)
- Example of embedded CSS with Yew in [global_style.rs](https://github.com/ffm5113/rust_javascript_interop/blob/main/src/styles/global_style.rs)
- Routing with [yew_router](https://yew.rs/docs/concepts/router)
- Language localization support with [text1.json](https://github.com/ffm5113/rust_javascript_interop/blob/main/src/text1.json) providing text for multiple languages
- Text localization using [text_provider.rs](https://github.com/ffm5113/rust_javascript_interop/blob/main/src/contexts/text_provider.rs) to pull the appropriate text dynamically

## Prerequisites

Before starting, ensure you have installed:
- [Rust and Cargo](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Trunk](https://trunkrs.dev/#install) (optional, for building and bundling)
- [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/) (for development on Windows)
  
## Project Structure with same JavaScript Function in 3 Examples

- `/src/main.rs`: Main entry point for the Yew application containing inline JavaScript session ID retrieval
- `/session2.js`: External JavaScript file for session ID retrieval
- `/src/static/session3.js`: Internal JavaScript file exmple imported for session ID retrieval

## Dependencies

Add these and any other relevant dependencies in your `Cargo.toml`, naming your package according to your project:

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
2. Use 'carg build' for initial compile.
3. Then, use `trunk serve` (if using Trunk) or `wasm-pack build` (if using wasm-pack) to prepare for serving.

For detailed examples of building with Cargo and serving with Trunk, including both Bash and PowerShell scripts, check out this repository: [Rust Cargo Build and Trunk Serve Examples](https://github.com/ffm5113/rust_cargo_build_trunk_serve).

The provided repository contains practical examples to help you get started with the build and serve process on different platforms.

## Contributing

Contributions are welcome. Follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or fix.
3. Commit changes.
4. Push to your branch.
5. Open a pull request.

## License

This project is under the [MIT License](LICENSE).
