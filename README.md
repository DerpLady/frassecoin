# FrasseCoin 🐾


<img width="314" height="512" alt="frassecoinlogo" src="https://github.com/user-attachments/assets/bca7f6e7-ef17-4814-97e6-3182a44f3ec8" />

## About

FrasseCoin is a **meme coin smart contract** built using [Rust](https://www.rust-lang.org/) and [ink!](https://use.ink/).  
It’s my **first Rust project**, created partly as a learning experience and partly as a fun experiment.  

⚠️ **Full disclosure**: I used an LLM (AI assistant) during the planning and development of this project to guide me through Rust, ink!, and good practices for publishing open-source code.

---

## Features

- Written in **Rust** with the **ink! smart contract framework**
- Basic meme coin logic (minting, balances, transfers)
- Lightweight project structure (cargo-based)
- Frasse approved

---

## Why?

Because Frasse is the cat of all cats.
And because writing a coin contract from scratch is a great way to **learn Rust + blockchain development**.

---

## Project Status

- ✅ Core contract compiles
- ✅ Builds to WebAssembly (`.contract` artifact)
- 🛠️ Work-in-progress: further testing and refinement
- 🚧 Not audited — **do not use in production or with real funds**

## Building

Requirements:
- Rust toolchain (latest stable)  
- `cargo-contract` installed  
- Add the Wasm target:
  ```bash
  rustup target add wasm32-unknown-unknown

## License

![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)
Licensed under the [MIT License](LICENSE).