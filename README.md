# FrasseCoin 🐾

![FrasseCoin Logo](frassecoinlogo.png)

## About

FrasseCoin is a **meme coin smart contract** built using [Rust](https://www.rust-lang.org/) and [ink!](https://use.ink/).  
It’s my **first Rust project**, created partly as a learning experience and partly as a fun experiment.  

⚠️ **Full disclosure**: I used an LLM (AI assistant) during the planning and development of this project to guide me through Rust, ink!, and good practices for publishing open-source code.

---

## Features

- Written in **Rust** with the **ink! smart contract framework**
- Basic meme coin logic (minting, balances, transfers)
- Lightweight project structure (cargo-based)
- Fun branding (because who doesn’t love cats 🐱)

---

## Why?

Because memes rule the internet.  
And because writing a coin contract from scratch is a great way to **learn Rust + blockchain development**.

---

## Project Status

- ✅ Core contract compiles
- ✅ Builds to WebAssembly (`.contract` artifact)
- 🛠️ Work-in-progress: further testing and refinement
- 🚧 Not audited — **do not use in production or with real funds**

---

## Building

Make sure you have the required toolchain:

- Rust (latest stable, plus `wasm32-unknown-unknown` target)
- [`cargo-contract`](https://github.com/paritytech/cargo-contract)

Then:

```b
