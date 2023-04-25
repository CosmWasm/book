# Floating point types

Now you are ready to create smart contracts on your own. It is time to discuss an important limitation of CosmWasm
smart contracts - floating-point numbers.

The story is short: you cannot use floating-point types in smart contracts. Never. CosmWasm virtual machine on purpose
does not implement floating-point Wasm instructions, even such basics as `F32Load`. The reasoning is simple: they are
not safe to work with in the blockchain world.

The biggest problem is that contract will compile, but uploading it to the blockchain would fail with an error message claiming there is a floating-point operation in the contract. A tool that verifies if the contract is valid (it does not contain any fp operations but also has all needed entry points and so on) is called `cosmwasm-check` [utility](https://github.com/CosmWasm/cosmwasm/tree/main/packages/check).

This limitation has two implications. First, you always have to use decimal of fixed-point arithmetic in your contracts.
It is not a problem, considering that `cosmwasm-std` provides you with the
[`Decimal`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Decimal.html) and
[Decimal256](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Decimal256.html) types.

The other implication is tricky - you must be careful with the crates you use. In particular, one gotcha in the `serde`
crate - deserialization of `usize` type is using floating-point operations. That means you can never use `usize` (or `isize`)
types in your deserialized messages in the contract.

Another thing that will not work with serde is untagged enums deserialization. The workaround is to create custom
deserialization of such enums using [`serde-cw-value`](https://crates.io/crates/serde-cw-value) crate. It is a fork of
[`serde-value`](https://crates.io/crates/serde-value) crate which avoids generating floating-point instructions.

