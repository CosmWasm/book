# Setting up the Environment for CosmWasm 2.0

Setting up the right environment is crucial for developing with CosmWasm 2.0. This section covers all the necessary steps and tools required to get started.

## Rust Installation

CosmWasm 2.0 development requires Rust. If it's not installed on your machine, follow the instructions on [the Rust website](insert link here).

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rust-lang.org | sh
```

# Wasm Compiler Backend

You'll need the Wasm Rust compiler backend to build Wasm binaries for CosmWasm. Install it with the following command:
```
rustup target add wasm32-unknown-unknown

```

# Testing with wasmd

For testing contracts on a testnet, the wasmd binary is required. Though this book primarily focuses on Rust unit testing, deploying contracts to a testnet can be beneficial.
```rust
// Install `wasmd`
git clone https://github.com/CosmWasm/wasmd.git
cd wasmd
make install
```

# Docker Installation

Docker is needed for uploading Rust Wasm contracts to the blockchain and for running the CosmWasm Rust Optimizer. This is essential to ensure your contracts do not exceed size limits.

# Install Docker
# Visit [Docker's official website](insert link here) for specific installation instructions

# Cosmwasm-Check Utility

The cosmwasm-check utility helps verify if your wasm binary is ready for blockchain deployment. Install it using cargo:
```rust
cargo install cosmwasm-check
```
To check if the installation was successful, run:
```rust
cosmwasm-check --version
```
# Verifying the Installation

To ensure your environment is correctly set up for CosmWasm 2.0 development, build example contracts from the cw-plus repository:
```rust
git clone https://github.com/CosmWasm/cw-plus.git
cd cw-plus
cargo test
```
The cw-plus repository, maintained by CosmWasm creators, is an excellent source for example contracts.

To verify the cosmwasm-check utility, build a contract and check its validity:

```rust
cd contracts/cw1-whitelist
cargo wasm
cosmwasm-check ../../target/wasm32-unknown-unknown/release/cw1_whitelist.wasm
```
This process will ensure that you are fully equipped and ready to start developing with CosmWasm 2.0, taking advantage of all its new features and improvements.
