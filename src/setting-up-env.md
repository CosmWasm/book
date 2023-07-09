# Setting up the environment

To work with a CosmWasm smart contract, you will need Rust installed on your
machine. If you don't have it, you can find installation instructions on [the
Rust website](https://www.rust-lang.org/tools/install).

I assume you are working with a stable Rust version in this book.

Additionally, you will need the Wasm Rust compiler backend installed to build
Wasm binaries. To install it, run:

```
rustup target add wasm32-unknown-unknown
```

Optionally, if you also want to try out your contracts on a testnet then you will need the
[wasmd](https://github.com/CosmWasm/wasmd) binary. We will focus on testing
contracts using Rust's unit testing utility throughout the book, so it is not
required to follow along. However, seeing the product working in a real-world
environment may be nice!

To install `wasmd`, first install [golang](https://github.com/golang/go/wiki#working-with-go). Then
clone the `wasmd` and install it:

```
$ git clone git@github.com:CosmWasm/wasmd.git
$ cd ./wasmd
$ make install
```

Also, to be able to upload Rust Wasm Contracts onto the blockchain, you will need
to install [docker](https://www.docker.com/). To minimize your contract sizes,
it will be required to run CosmWasm Rust Optimizer, otherwise more complex
contracts might exceed a size limit.

## cosmwasm-check utility

An additional helpful tool for building smart contracts is the `cosmwasm-check`[utility](https://github.com/CosmWasm/cosmwasm/tree/main/packages/check). It allows you to check if the wasm binary is a proper smart contract ready to upload onto the blockchain. You can install it using Cargo:

```
$ cargo install cosmwasm-check
```

If the installation succeeds, you should be able to execute the utility from your command line.

```
$ cosmwasm-check --version
Contract checking 1.2.3
```

## Verifying the installation

To guarantee you are ready to build your smart contracts, you need to make sure you can build the examples.
Checkout the [cw-plus](https://github.com/CosmWasm/cw-plus) repository and run the testing command in
its folder:

```
$ git clone git@github.com:CosmWasm/cw-plus.git
$ cd ./cw-plus
cw-plus $ cargo test
```

You should see that everything in the repository gets compiled correctly, and all tests pass. 

`cw-plus` is a great place to find example contracts - look for them in the `contracts` directory. The
repository is maintained by CosmWasm creators, so contracts in there should follow good practices.

To verify the `cosmwasm-check` utility, you will first need to build a smart contract. Go to some contract's directory (for example, `contracts/cw1-whitelist`), and call `cargo wasm`:

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
