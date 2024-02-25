# Managing Contract State in CosmWasm 2.0

In this chapter, we delve into the concept of state in CosmWasm smart contracts, enabling dynamic and interactive contract behavior. Initially, the state will be initialized upon contract instantiation, containing a list of admins with exclusive execution privileges.

## Updating Dependencies
First, ensure your Cargo.toml reflects the latest dependencies, crucial for state management in CosmWasm 2.0:

```rust
[package]
name = "contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
cosmwasm-std = "2.0.0" # Updated for CosmWasm 2.0
serde = { version = "1.0", default-features = false, features = ["derive"] }
cw-storage-plus = "0.14.0" # Updated version for enhanced state management

[dev-dependencies]
cw-multi-test = "0.14.0" # Ensure compatibility with CosmWasm 2.0

```

## State Definition

Create src/state.rs to define the contract's state structure:

```rust
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// Admins storage, utilizing the Item type for simple key-value storage
pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");

// Declare the state module in src/lib.rs:

mod state;
```
The `ADMINS` constant, an `Item<Vec<Addr>>`, doesn't store the state itself but acts as an accessor to the blockchain's state managed through the ``deps` argument in entry points.

## Instantiation Message Update

Revise `src/msg.rs` to include an `InstantiateMsg` that lists admins:
```rust
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
}
```
## State Initialization

Update the instantiation logic in src/contract.rs to initialize the state with the provided admins list:
```rust
use crate::state::ADMINS;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let admins: StdResult<Vec<_>> = msg
        .admins
        .into_iter()
        .map(|addr| deps.api.addr_validate(&addr))
        .collect();
    ADMINS.save(deps.storage, &admins?)?;

    Ok(Response::new())
}
```
Ensure the entry point in src/lib.rs is updated accordingly
```rust
use msg::InstantiateMsg;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}
```
## Testing State Initialization
Implement a test in `src/contract.rs` to verify state initialization affects the contract as expected. Include a query to list all admins:

```rust
// Add a variant for the query message and a response message
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AdminsListResp {
    pub admins: Vec<Addr>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
    Greet {},
    AdminsList {},
}

// Implement the query in the contract
mod query {
    pub fn admins_list(deps: Deps) -> StdResult<AdminsListResp> {
        let admins = ADMINS.load(deps.storage)?;
        Ok(AdminsListResp { admins })
    }
}
```

The new thing we have here is the ADMINS constant of type Item<Vec<Addr>>. You could ask an excellent question here - how is the state constant? How do I modify it if it is a constant value?

The answer is tricky - this constant is not keeping the state itself. The state is stored in the blockchain and is accessed via the deps argument passed to entry points. The storage-plus constants are just accessor utilities helping us access this state in a structured way.

In CosmWasm, the blockchain state is just massive key-value storage. The keys are prefixed with metainformation pointing to the contract which owns them (so no other contract can alter them in any way), but even after removing the prefixes, the single contract state is a smaller key-value pair.

storage-plus handles more complex state structures by additionally prefixing items keys intelligently. For now, we just used the simplest storage entity - an Item<_>, which holds a single optional value of a given type - Vec<Addr> in this case. And what would be a key to this item in the storage? It doesn't matter to us - it would be figured out to be unique, based on a unique string passed to the new function.

Before we would go into initializing our state, we need some better instantiate message. Go to src/msg.rs and create one:

```rust
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
}

// Now go forward to instantiate the entry point in src/contract.rs, and initialize our state to whatever we got in the instantiation message:


use crate::state::ADMINS;
// --snip--
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let admins: StdResult<Vec<_>> = msg
        .admins
        .into_iter()
        .map(|addr| deps.api.addr_validate(&addr))
        .collect();
    ADMINS.save(deps.storage, &admins?)?;

    Ok(Response::new())
}
```
We also need to update the message type on entry point in src/lib.rs:

```rust
use msg::InstantiateMsg;
// --snip--
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}
```
Voila, that's all that is needed to update the state!

First, we need to transform the vector of strings into the vector of addresses to be stored. We cannot take addresses as a message argument because not every string is a valid address. It might be a bit confusing when we were working on tests. Any string could be used in the place of address. Let me explain.

Every string can be technically considered an address. However, not every string is an actual existing blockchain address. When we keep anything of type Addr in the contract, we assume it is a proper address in the blockchain. That is why the addr_validate function exits - to check this precondition.

Having data to store, we use the save function to write it into the contract state. Note that the first argument of save is &mut Storage, which is actual blockchain storage. As emphasized, the Item object stores nothing and is just an accessor. It determines how to store the data in the storage given to it. The second argument is the serializable data to be stored.

It is a good time to check if the regression we have passes - try running our tests:


> cargo test

...

running 1 test
test contract::tests::greet_query ... FAILED

failures:

---- contract::tests::greet_query stdout ----
thread 'contract::tests::greet_query' panicked at 'called `Result::unwrap()` on an `Err` value: error executing WasmMsg:
sender: owner
Instantiate { admin: None, code_id: 1, msg: Binary(7b7d), funds: [], label: "Contract" }

Caused by:
    Error parsing into type contract::msg::InstantiateMsg: missing field `admins`', src/contract.rs:80:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    contract::tests::greet_query

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
Damn, we broke something! But be calm. Let's start with carefully reading an error message:

Error parsing into type contract::msg::InstantiateMsg: missing field admins', src/contract.rs:80:14

The problem is that in the test, we send an empty instantiation message in our test, but right now, our endpoint expects to have an admin field. Multi-test framework tests contract from the entry point to results, so sending messages using MT functions first serializes them. Then the contract deserializes them on the entry. But now it tries to deserialize the empty JSON to some non-empty message! We can quickly fix it by updating the test:

```rust
    #[test]
    fn greet_query() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { admins: vec![] },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: GreetResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Greet {})
            .unwrap();

        assert_eq!(
            resp,
            GreetResp {
                message: "Hello World".to_owned()
            }
        );
    }
```

## Testing state
When the state is initialized, we want a way to test it. We want to provide a query to check if the instantiation affects the state. Just create a simple one listing all admins. Start with adding a variant for query message and a corresponding response message in src/msg.rs. We'll call the variant AdminsList, the response AdminsListResp, and have it return a vector of cosmwasm_std::Addr:

```rust
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AdminsListResp  {
    pub admins: Vec<Addr>,
}

[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
    Greet {},
    AdminsList {},
}
And implement it in src/contract.rs:


use crate::msg::{AdminsListResp, GreetResp, InstantiateMsg, QueryMsg};
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {} => to_binary(&query::greet()?),
        AdminsList {} => to_binary(&query::admins_list(deps)?),
    }
}
 
mod query {
    pub fn admins_list(deps: Deps) -> StdResult<AdminsListResp> {
        let admins = ADMINS.load(deps.storage)?;
        let resp = AdminsListResp { admins };
        Ok(resp)
    }
}
```

Now when we have the tools to test the instantiation, let's write a test case:

```rust
use crate::msg::{AdminsListResp, GreetResp, InstantiateMsg, QueryMsg};
#[cfg(test)]
mod tests {
    #[test]
    fn instantiation() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { admins: vec![] },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: AdminsListResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::AdminsList {})
            .unwrap();

        assert_eq!(resp, AdminsListResp { admins: vec![] });

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec!["admin1".to_owned(), "admin2".to_owned()],
                },
                &[],
                "Contract 2",
                None,
            )
            .unwrap();

        let resp: AdminsListResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::AdminsList {})
            .unwrap();

        assert_eq!(
            resp,
            AdminsListResp {
                admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")],
            }
        );
    }
}
```
The test is simple - instantiate the contract twice with different initial admins, and ensure the query result is proper each time. This is often the way we test our contract - we execute bunch o messages on the contract, and then we query it for some data, verifying if query responses are like expected.

We are doing a pretty good job developing our contract. Now it is time to use the state and allow for some executions.