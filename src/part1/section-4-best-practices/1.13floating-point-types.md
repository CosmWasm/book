# Floating Point Types and Their Limitations in CosmWasm
As you venture into the realm of smart contract development with CosmWasm, it's crucial to understand certain limitations, particularly regarding floating-point numbers. The use of floating-point types is a significant consideration that developers must navigate carefully.

## The Fundamental Restriction
In the context of CosmWasm smart contracts, floating-point numbers (e.g., `f32`, `f64`) are fundamentally prohibited. This restriction is by design, as the CosmWasm virtual machine explicitly omits the implementation of floating-point WebAssembly instructions, including basic operations like `F32Load`. The rationale behind this decision is rooted in the need for deterministic and safe operations in the blockchain environment.

Attempting to deploy a contract containing floating-point operations to a blockchain will result in an error during the upload process. This error indicates the presence of unsupported floating-point operations within the contract code. To assist developers in ensuring contract compliance, the `cosmwasm-check` utility can be used to validate contracts against these constraints.

## Navigating the Floating-Point Constraint
Given this limitation, developers are encouraged to employ alternative strategies for numerical operations within their contracts. Specifically, CosmWasm provides `Decimal` and `Decimal256` types to facilitate decimal or fixed-point arithmetic, offering a robust solution for handling numerical calculations without relying on floating-point types.

## Watch Out for External Crates
A less obvious implication of the floating-point restriction is the need for vigilance when incorporating external crates into your contract. Certain operations or types within popular crates might inadvertently introduce floating-point operations into your contract code. For example, serde's deserialization of `usize` (or `isize`) employs floating-point operations, making these types unsuitable for contracts requiring serialization or deserialization.

Additionally, the deserialization of untagged enums using `serde` can also lead to issues. To circumvent this, developers can leverage the `serde-cw-value` crate, a fork of serde-value specifically modified for CosmWasm to avoid generating floating-point instructions. This crate provides a pathway for custom deserialization processes that maintain compliance with CosmWasm's limitations.

## Practical Advice for Developers

- **Utilize `Decimal` and `Decimal256`:** For numerical calculations, prefer CosmWasm's built-in types that are designed to work within the platform's constraints.
- **Audit External Dependencies:** Carefully review any external crates for potential floating-point operations, especially those involved in serialization and deserialization.
- **Employ `serde-cw-value` for Custom Deserialization:** When dealing with complex data structures that require deserialization, `serde-cw-value` offers a compliant solution.

## Conclusion
Understanding and adapting to the limitations regarding floating-point types in CosmWasm is essential for developing secure, deterministic, and blockchain-compatible smart contracts. By leveraging the provided alternatives and being mindful of the implications of external dependencies, developers can navigate these constraints effectively, ensuring their contracts are robust and compliant with the CosmWasm platform.