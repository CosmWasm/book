# Setting up the environment

To work with CosmWasm smart contract, you will need rust installed on your
machine. If you don't have one, you can find installation instructions on [the
Rust website](https://www.rust-lang.org/tools/install).

I assume you are working with a stable Rust channel in this book.

Additionally, you will need the Wasm rust compiler backend installed to build
Wasm binaries. To install it, run:

```
rustup target add wasm32-unknown-unknown
```

Optionally if you want to try out your contracts on a testnet, you will need a
[wasmd](https://github.com/CosmWasm/wasmd) binary. We would focus on testing
contracts with Rust unit testing utility throughout the book, so it is not
required to follow. However, seeing the product working in a real-world
environment may be nice.

To install `wasmd`, first install the [golang](https://github.com/golang/go/wiki#working-with-go). Then
clone the `wasmd` and install it:

```
$ git clone git@github.com:CosmWasm/wasmd.git
$ cd ./wasmd
$ make install
```

Also, to be able to upload Rust Wasm Contracts into the blockchain, you will need
to install [docker](https://www.docker.com/). To minimize your contract sizes,
it will be required to run CosmWasm Rust Optimizer; without that, more complex
contracts might exceed a size limit.

## cosmwasm-check utility

An additional helpful tool for building smart contracts is the `cosmwasm-check`[utility](https://github.com/CosmWasm/cosmwasm/tree/main/packages/check). It allows you to check if the wasm binary is a proper smart contract ready to upload into the blockchain. You can install it using cargo:

```
$ cargo install cosmwasm-check
```

If the installation succeeds, you should be able to execute the utility from your command line.

```
$ cosmwasm-check --version
Contract checking 1.2.3
```

## Verifying the installation

To guarantee you are ready to build your smart contracts, you need to make sure you can build examples.
Checkout the [cw-plus](https://github.com/CosmWasm/cw-plus) repository and run the testing command in
its folder:

```
$ git clone git@github.com:CosmWasm/cw-plus.git
$ cd ./cw-plus
cw-plus $ cargo test
```

You should see that everything in the repository gets compiled, and all tests pass. 

`cw-plus` is a great place to find example contracts - look for them in `contracts` directory. The
repository is maintained by CosmWasm creators, so contracts in there should follow good practices.

To verify the `cosmwasm-check` utility, first, you need to build a smart contract. Go to some contract directory, for example, `contracts/cw1-whitelist`, and call `cargo wasm`:

```
cw-plus $ cd contracts/cw1-whitelist
cw-plus/contracts/cw1-whitelist $ cargo wasm
```

You should be able to find your output binary in the `target/wasm32-unknown-unknown/release/`
of the root repo directory - not in the contract directory itself! Now you can check if contract
validation passes:

```
cw-plus/contracts/cw1-whitelist $ cosmwasm-check ../../target/wasm32-unknown-unknown/release/cw1_whitelist.wasm
Available capabilities: {"iterator", "cosmwasm_1_1", "cosmwasm_1_2", "stargate", "staking"}

../../target/wasm32-unknown-unknown/release/cw1_whitelist.wasm: pass

All contracts (1) passed checks!
```

# **Setting Up Your Environment for CosmWasm Development**

A properly configured development environment is crucial to begin working with CosmWasm smart contracts. This guide will walk you through setting up the necessary tools and verifying that everything is in place to build and deploy smart contracts on the Cosmos blockchain.

### Prerequisites

1. **Rust Installation:** You need Rust installed on your machine. You can find the installation instructions on the Rust website if Rust is not already installed. This guide assumes you are using the stable Rust channel.
2. **Wasm Compiler Backend:** The Wasm rust compiler backend is required for building Wasm binaries. Install it using the following Rust toolchain command:
    
    ```rust
    rustup target add wasm32-unknown-unknown
    ```
    
3. **[Wasmd](https://github.com/CosmWasm/wasmd):** If you want to deploy your contracts on a testnet, you'll need the **`wasmd`** binary. While this guide focuses on Rust unit testing for contract validation, testing in a real-world scenario can be beneficial. To install **`wasmd`**:
    - Install [golang](https://github.com/golang/go/wiki#working-with-go) on your machine.
    - Clone and install **`wasmd`** with the following commands:
        
        ```rust
        git clone git@github.com:CosmWasm/wasmd.git
        cd ./wasmd
        make install
        ```
        
4. **[docker](https://www.docker.com/):** Installing Docker is necessary for utilizing the CosmWasm Rust Optimizer, which is crucial for minimizing contract sizes. Complex contracts without optimization might exceed the blockchain's size limit.
5. **cosmwasm-check [utility](https://github.com/CosmWasm/cosmwasm/tree/main/packages/check)** A valuable tool for smart contract development is **`cosmwasm-check`**. It checks if the wasm binary is a valid smart contract ready for blockchain deployment. Install it using cargo:
    
    ```rust
    cargo install cosmwasm-check
    ```
    
    Verify the installation by checking the utility's version:
    
    ```rust
    cosmwasm-check --version
    ```
    
    You should see an output like **`Contract checking 1.2.3`**.
    

### Verifying Installation

To ensure your development environment is correctly set up, it's essential to test building and running examples:

1. **Test Building with cw-plus:** Clone the **`cw-plus`** repository and run tests to confirm everything compiles and passes as expected.
    
    ```rust
    git clone git@github.com:CosmWasm/cw-plus.git
    cd ./cw-plus
    cargo test
    ```
    
    The **`cw-plus`** repository contains example contracts and is maintained by the CosmWasm team, adhering to best practices.
    
2. **Verifying cosmwasm-check Utility:** Build a smart contract to test the **`cosmwasm-check`** utility. For example, navigate to the **`contracts/cw1-whitelist`** directory and build the contract:
    
    ```rust
    cd contracts/cw1-whitelist
    cargo wasm
    ```
    
    Then, check if the contract validation passes:
    
    ```rust
    cosmwasm-check ../../target/wasm32-unknown-unknown/release/cw1_whitelist.wasm
    ```
    
    Successful output should list available capabilities and confirm that the contract passed checks.
    

Following these steps will set your environment up for CosmWasm smart contract development. You are ready to build, test, and deploy smart contracts on the Cosmos blockchain.