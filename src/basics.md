# Basics

In this chapter, we will go through creating essential smart contracts step by step. I will explain the core ideas behind CosmWasm and the typical contract structure.

### Creating a Rust Project for CosmWasm Smart Contracts

Developing smart contracts for the CosmWasm platform requires a structured approach, starting from setting up a Rust project. This dedicated guide will walk you through creating a Rust library crate designed to work as a CosmWasm smart contract, including configuring the cargo.toml file to meet the requirements of compiling to WebAssembly (Wasm), the format needed for deploying on the CosmWasm platform.

### Step 1: Initialize Your Rust Library

You'll need to create a new Rust library to kick off your smart contract development. Open your terminal and execute the following command, which creates a new directory named empty-contract and initializes a Rust project configured as a library:

cargo new --lib ./empty-contract

This command sets up a basic Rust project structure, including a Cargo. The toml file will be used to manage your project's settings and dependencies, and a src directory will be used where your Contract's Rust source code will reside.

### Step 2: Configuring the Cargo.toml file

After setting up your project, the next crucial step is configuring the cargo.toml file. This file resides at the root of your project and dictates how your project is built. Navigate to the empty-contract directory and open the cargo.toml file in your preferred editor to make the following adjustments:

[package] name = "contract" version = "0.1.0" edition = "2021" [lib] crate-type = ["cdylib"] [dependencies] cosmwasm-std = { version = "1.0.0-beta8", features = ["staking"] }

### Explanation of Key Configurations:

- **[package]**: This section defines basic metadata about your project. The edition field specifies which edition of Rust you are targeting, with "2021" being the most recent as of this writing.
- **[lib]**: By specifying crate-type as ["cdylib"], you are instructing Rust to compile your library into a dynamic library, specifically a WebAssembly (Wasm) binary. This is required for the Contract to run in the CosmWasm environment. Note that this configuration means the compiled library cannot be included as a dependency in other Rust crates.
- **[dependencies]**: The cosmwasm-std dependency is essential for smart contract development on the CosmWasm platform. It acts as the standard library, providing the types, functions, and utilities necessary to interact with the blockchain. The version should be the latest stable version compatible with your project's requirements, and the features field enables specific functionalities, such as staking in this case, which may be necessary depending on your Contract's use case.

### Final Steps:

With your Rust project correctly configured, you can start writing your smart contract code within the src/lib.rs file. This involves implementing the Contract's logic, including handling initialization, execute, and query operations according to the CosmWasm standard.

As you develop your smart Contract, regularly compile and test your code to ensure that it meets the expected functionalities and security standards required for deployment on the blockchain. Utilizing the comprehensive tooling and resources available in the Rust and CosmWasm communities will aid in this process, helping you to develop robust, efficient, and secure smart contracts.