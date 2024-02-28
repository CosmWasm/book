# Good Practices in CosmWasm 2.0
With the foundation set, it's time to refine our approach with best practices that align with CosmWasm 2.0 developments. These practices ensure our contracts are not only efficient and secure but also maintainable and easy to interact with.

## JSON Renaming for Compatibility
CosmWasm embraces Rust's camelCase convention for naming. However, the JSON world predominantly uses snake_case. To bridge this stylistic gap, Serde attributes offer a seamless way to ensure our JSON keys match the expected case without manual string manipulation.
```rust
use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub donation_denom: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddMembers { admins: Vec<String> },
    Leave {},
    Donate {},
}
// Additional structs and enums follow the same pattern...
```
This minor annotation ensures our contract's external API adheres to the JSON naming conventions, enhancing interoperability and ease of use.

## Leveraging JSON Schema for API Documentation
Defining the shape of JSON messages via a schema is a robust practice to document and validate the contract's API. Writing schemas by hand is cumbersome, but automation comes to the rescue with the cosmwasm-schema crate. This tool generates schemas directly from our Rust code, ensuring accuracy and saving time.

To integrate this functionality, we adjust Cargo.toml to include necessary dependencies and make a slight modification to our message structs:
```rust
# In Cargo.toml
[dependencies]
schemars = "0.8.1"
cosmwasm-schema = "1.1.4"
```
Then, we annotate our message structs with JsonSchema to auto-generate the schema documentation:

```rust
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    // Fields...
}
// Repeat for other message structs...
```
Simplifying Boilerplate with `cw_serde`
The `cosmwasm-schema` crate's `cw_serde` macro further simplifies our code by wrapping common derive macros, reducing boilerplate:

```rust
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    // Fields...
}
// Repeat for other message structs...
```
## QueryResponses for Enhanced Clarity
For query messages, correlating variants with their responses is streamlined using the QueryResponses trait, which mandates specifying return types:
```rust
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GreetResp)]
    Greet {},
    // Other variants...
}
```
This explicit mapping enhances clarity and type safety, ensuring that the contract's query API is well-defined and predictable.

## Optimizing Contract Code for Library Use

When our contract is intended for use as a dependency, we must avoid duplicate Wasm entry points. This is achieved by conditionally compiling entry points only when the contract is not used as a library:

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    // Parameters...
) -> StdResult<Response> {
    // Implementation...
}
```
By adding a `library` feature in `Cargo.toml`, we can toggle this behavior based on the contract's use case, ensuring compatibility and preventing compilation issues:
```rust
[features]
library = []
```
## Conclusion

Adopting these good practices not only aligns our CosmWasm contracts with the latest standards but also ensures they are developer-friendly, maintainable, and seamlessly integrate with the broader ecosystem. By leveraging Serde for JSON compatibility, automating schema generation, reducing boilerplate, and optimizing for library usage, we set a solid foundation for building robust and interoperable smart contracts in CosmWasm 2.0.