# Building the contract

Now it is time to build our contract. We can use a traditional cargo build pipeline for local
testing purposes: `cargo build` for compiling it and `cargo test` for running all tests (which
we don't have yet, but we will work on that soon).

However, to upload the contract to the blockchain, we need to create a wasm binary. We can do
it by passing an additional argument to the build command:

```rust
cargo build --target wasm32-unknown-unknown --release
```

The `--target` argument tells cargo to perform cross-compilation for a given target instead of
building a native binary for an OS it is running on - in this case, `wasm32-unknown-unknown`,
which is a fancy name for Wasm target.

Additionally, I passed the `--release` argument to the command - it is not required, but in
most cases, debug information is not very useful while running on-chain. It is crucial to reduce
the uploaded binary size for gas cost minimization. It is worth knowing that there is a CosmWasm
Rust Optimizer tool that takes care of building even smaller binaries. For the production, all
the contracts should be compiled using this tool, but for learning purposes, it is not an essential
thing to do.

## Aliasing build command

Now I can see you are disappointed in building your contracts with some overcomplicated command
instead of simple `cargo build`. Hopefully, it is not the case. The common practice is to alias
the building command to make it as simple as building a native app.

Let's create the `.cargo/config` file in your contract project directory with the following content:

```toml
[alias]
wasm = "build --target wasm32-unknown-unknown --release"
wasm-debug = "build --target wasm32-unknown-unknown"
```

Now, building your Wasm binary is as easy as executing `cargo wasm`. We also added the additional
`wasm-debug` command for rare cases when we want to build the wasm binary, including debug information.

