# Rust Yew Framework Example with JavaScript Interop

## Project Overview

This project is a basic Rust web application using the Yew framework. It focuses on JavaScript-Rust interop for session ID retrieval, demonstrating how to retrieve a session ID from the URL using both inline JavaScript and an external JavaScript file. 

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

## Project Structure

- `src/main.rs`: Main entry point for the Yew application
- `src/session.js`: External JavaScript file for session ID retrieval

## Dependencies

Add these dependencies in your `Cargo.toml`:

```toml
[package]
name = "yew_session_id_example"
version = "0.1.0"
edition = "2018"

[dependencies]
yew = "0.19"
yew-router = "0.16"
wasm-bindgen = "0.2"
js-sys = "0.3"
stylist = "0.10"

[lib]
crate-type = ["cdylib", "rlib"]
