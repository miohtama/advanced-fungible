Advanced Fungible Token for NEAR blockchain.

Part of Rainbow hackathon: https://gitcoin.co/hackathon/hack-the-rainbow/projects/1497/Advanced-Fungible-Token-Standard-NEP-9000

## Overview

Open source contracts for a golden token standard for NEAR protocol

* Slick user and developer experience with single transaction `send()` vs. `approve()` and `transferFrom()`

* Provide security primitives to safely work with tokens on the NEAR sharded blockchain and promises

* A lot of code examples

## How does it work


## Development

# Run

```sh

# Install rust
brew install rustup
rustup update

# Build and execute contract unit tests
cd contracts
cargo build
cargo test

# Build and execute contract acceptance tests
cd ..
yarn install
yarn jest
```

## Visual Studio Code

Install Rust extension. Choose *Start Rust Server* from the command palette.

# Test cases

JavaScript test cases are written with a custom

# Challenges

NEAR protocol is advertised developer-friendly, but currently the state of the matter is that this statement
is mostly inspirational. A lot of toolchain design mistakes, brokeness and lack of documentation held
back the development.

- The lack of notion that two contracts may be actually needed and they may interact through all the toolchain

- `create-near-app` is hardcoded for a single contract, both JavaScript codebase and Rust codebase

- Rust unit tests cannot test contract interactions

- Contracts are defined in Rust as libraries instead of binaries what they are,
  making dealing with multiple contracts even more difficult and semantically incorrect

- A broken simulation tests suite exists, but is undocumented, way too difficult to use and
  does not support contract interactions

- There is no concept of ABI files, all contract interfaces must be re-described in JavaScript by hand

- near-js-api does not document how to interact with contracts https://near.github.io/near-api-js/modules/_contract_.html

- near-js-api

- Manually specifying gas amounts in contract calls is cumbersome https://github.com/near/core-contracts/blob/master/lockup/src/owner_callbacks.rs#L330

# Further reading

Some other code examples:

https://github.com/near/near-sdk-rs/blob/master/examples/fungible-token/src/lib.rs

https://github.com/near/core-contracts/tree/master/lockup/src

https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html

https://github.com/near-examples/simulation-testing

https://github.com/near-examples/guest-book/tree/master

https://github.com/smartcontractkit/near-protocol-contracts