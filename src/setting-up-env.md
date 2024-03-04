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