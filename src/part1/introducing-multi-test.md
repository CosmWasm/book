# Multi-test examples

## Leveraging cw-multi-test for Contract Testing
In the CosmWasm ecosystem, rigorous testing of smart contracts is essential to ensure their correctness and reliability. The `cw-multi-test` library offers a powerful toolset for simulating a blockchain environment, allowing developers to test both individual contracts and interactions between multiple contracts efficiently.

## Getting Started with cw-multi-test
To incorporate cw-multi-test into your project for development and testing purposes, you must add it to your Cargo.toml under [dev-dependencies]. This ensures that cw-multi-test is utilized during the development phase without being included in your production builds. Update your Cargo.toml file as follows:
```rust
[dev-dependencies]
cw-multi-test = "0.13.4" # Ensure this version matches the latest release

```
Creating Your First Test with cw-multi-test
cw-multi-test simplifies the process of writing tests by providing a virtual blockchain environment where contracts can be deployed, interacted with, and observed. Here's an example to guide you through writing a basic test:
```rust
#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};
    use super::*;

    #[test]
    fn basic_contract_interaction() {
        // Initialize the App to simulate the blockchain
        let mut app = App::default();

        // Create a wrapper for your contract's functionalities
        let contract = ContractWrapper::new(execute, instantiate, query);

        // Store and retrieve the contract's code ID within the app
        let code_id = app.store_code(Box::new(contract));

        // Simulate contract instantiation on the blockchain
        let contract_addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("contract_owner"),
                &Empty {},
                &[],
                "My CosmWasm Contract",
                None,
            )
            .unwrap();

        // Example of querying the instantiated contract
        let query_response: MyQueryResponse = app
            .wrap()
            .query_wasm_smart(contract_addr, &MyQueryMsg {})
            .unwrap();

        // Assertions can be made here based on the expected query response
        assert_eq!(query_response, expected_response);
    }
}

```
This example demonstrates how to initialize the `cw-multi-test` app, wrap your contract functions, simulate storing the contract on the blockchain, instantiate the contract, and perform a query. Testing with `cw-multi-test` enables thorough interaction testing within a simulated environment, providing confidence in contract behavior before deployment.

## Advancing with Complex Interactions and Multiple Contracts
`cw-multi-test` excels in scenarios involving complex contract interactions and the orchestration of multiple contracts. It allows developers to simulate intricate blockchain states and interactions, ensuring that contracts perform as expected in a composite ecosystem.

By leveraging cw-multi-test, developers can craft detailed tests that mimic real-world contract usage, interactions, and dependencies, providing a robust framework for ensuring contract reliability and functionality within the CosmWasm ecosystem.