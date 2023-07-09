# Building the contract

Now it is time to build our contract. We can use a traditional cargo build
pipeline for local testing purposes: `cargo build` for compiling it and `cargo
test` for running all tests (which we don't have yet, but we'll work on that
soon).

However, we need to create a Wasm binary to upload the contract to blockchain, which we can do by passing an additional argument to the build command:

```
$ cargo build --target wasm32-unknown-unknown --release
```

The `--target` argument tells cargo to perform cross-compilation for a given target instead of
building a native binary for the OS it is running on - in this case, `wasm32-unknown-unknown`,
which is just a fancy name for a Wasm target.

We also passed the `--release` argument to the command - it is not strictly
required, but in most cases debug information is not very useful while running
on-chain. However, it is crucial to reduce the size of the uploaded binary in order to minimize
gas costs for execution. It is worth knowing that there is a [CosmWasm Rust
Optimizer](https://github.com/CosmWasm/rust-optimizer) tool that enables us to build even smaller
binaries. For production, all the contracts should be compiled using this tool, but for
learning purposes it is not essential.

## Aliasing build command

Now, you may be dismayed at the idea of having to building your contracts with some over-complicated command
instead of just the usual simple `cargo build`. Thankfully, this is not necessary. A common practice is to alias
the building command to make it as simple as building a native app.

Let's create the `.cargo/config` file in your contract project directory with the following content:

```toml
[alias]
wasm = "build --target wasm32-unknown-unknown --release"
wasm-debug = "build --target wasm32-unknown-unknown"
```

Now building your Wasm binary is as easy as executing `cargo wasm`! We also added the additional
`wasm-debug` command for the rare cases when we want to build the wasm binary along with its debug information.

## Checking contract validity

When the contract is built, the last step to ensure it is a valid CosmWasm contract is to call
`cosmwasm-check` on it:

```
$ cargo wasm
...
$ cosmwasm-check ./target/wasm32-unknown-unknown/release/contract.wasm
Available capabilities: {"cosmwasm_1_1", "staking", "stargate", "iterator", "cosmwasm_1_2"}

./target/wasm32-unknown-unknown/release/contract.wasm: pass
```
