# Create a Rust project

As smart contracts are just Rust library crates, so let's start by creating one:

```
$ cargo new --lib ./empty-contract
```

We created a simple Rust library, but it is not yet ready to be a smart contract. The first thing
to do is to update the `Cargo.toml` file:

```toml
[package]
name = "contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
cosmwasm-std = { version = "1.0.0-beta8", features = ["staking"] }

```

As you can see, we added a `crate-type` field for the library section. Generating the `cdylib` is
required to create a proper web assembly binary. The downside of this is that such a library cannot
be used as a dependency for other Rust crates.  We don't need this anyway just now, but later we shall
demonstrate an approach allowing us to re-use contracts as dependencies.

Additionally, there is one core dependency for smart contracts that we need: `cosmwasm-std`. This crate is a
standard library for smart contracts. It provides essential utilities for communication with the
outside world and a couple of helper functions and types. Every smart contract we build will
use this dependency.

