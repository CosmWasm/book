# Contract state

The contract we are working on already has some behavior that can answer a query. Unfortunately, it is
very predictable with its answers, and it has nothing to alter them. In this chapter, I introduce the
notion of state, which would allow us to bring true life to a smart contract.

The state would still be static for now - it would be initialized on contract instantiation. The state
would contain a list of admins who would be eligible to execute messages in the future.

The first thing to do is to update `Cargo.toml` with yet another dependency - the
[`storage-plus`](https://crates.io/crates/cw-storage-plus) crate with high-level bindings for CosmWasm
smart contracts state management:

```toml
[package]
name = "contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
cosmwasm-std = { version = "1.0.0-beta8", features = ["staking"] }
serde = { version = "1.0.103", default-features = false, features = ["derive"] }
cw-storage-plus = "0.13.4"

[dev-dependencies]
cw-multi-test = "0.13.4"
```

Now create a new file where you will keep a state for the contract - we typically call it `src/state.rs`:

```rust,noplayground
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");
```

And make sure we declare the module in `src/lib.rs`:

```rust,noplayground
# use cosmwasm_std::{
#     entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
# };
# 
# mod contract;
# mod msg;
mod state;
#
# #[entry_point]
# pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty)
#   -> StdResult<Response>
# {
#     contract::instantiate(deps, env, info, msg)
# }
#
# #[entry_point]
# pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg)
#   -> StdResult<Binary>
# {
#     contract::query(deps, env, msg)
# }
```

The new thing we have here is the `ADMINS` constant of type `Item<Vec<Addr>>`. You could ask an excellent
question here - how is the state constant? How do I modify it if it is a constant value?

The answer is tricky - this constant is not keeping the state itself. The state is stored in the
blockchain and is accessed via the `deps` argument passed to entry points. The storage-plus constants
are just accessor utilities helping us access this state in a structured way.

In CosmWasm, the blockchain state is just massive key-value storage. The keys are prefixed with metainformation
pointing to the contract which owns them (so no other contract can alter them in any way), but even after
removing the prefixes, the single contract state is a smaller key-value pair.

`storage-plus` handles more complex state structures by additionally prefixing items keys intelligently. For now,
we just used the simplest storage entity - an
[`Item<_>`](https://docs.rs/cw-storage-plus/0.13.4/cw_storage_plus/struct.Item.html), which holds a single optional
value of a given type -
`Vec<Addr>` in this case. And what would be a key to this item in the storage? It doesn't matter to us - it would
be figured out to be unique, based on a unique string passed to the
[`new`](https://docs.rs/cw-storage-plus/0.13.4/cw_storage_plus/struct.Item.html#method.new) function.

Before we would go into initializing our state, we need some better instantiate message. Go to `src/msg.rs` and create one:

```rust,noplayground
# use cosmwasm_std::Addr;
# use serde::{Deserialize, Serialize};
# 
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
}
# 
# #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
# pub struct GreetResp {
#     pub message: String,
# }
# 
# #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
# pub enum QueryMsg {
#     Greet {},
# }
```

Now go forward to instantiate the entry point in `src/contract.rs`, and initialize our state to whatever we got in the instantiation message:

```rust,noplayground
# use crate::msg::{GreetResp, InstantiateMsg, QueryMsg};
use crate::state::ADMINS;
// --snip--
# use cosmwasm_std::{
#     to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
# };
# 
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
# 
# pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
#     use QueryMsg::*;
# 
#     match msg {
#         Greet {} => to_binary(&query::greet()?),
#     }
# }
# 
# #[allow(dead_code)]
# pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
#     unimplemented!()
# }
# 
# mod query {
#     use super::*;
# 
#     pub fn greet() -> StdResult<GreetResp> {
#         let resp = GreetResp {
#             message: "Hello World".to_owned(),
#         };
# 
#         Ok(resp)
#     }
# }
# 
# #[cfg(test)]
# mod tests {
#     use cosmwasm_std::Addr;
#     use cw_multi_test::{App, ContractWrapper, Executor};
# 
#     use super::*;
# 
#     #[test]
#     fn greet_query() {
#         let mut app = App::default();
# 
#         let code = ContractWrapper::new(execute, instantiate, query);
#         let code_id = app.store_code(Box::new(code));
# 
#         let addr = app
#             .instantiate_contract(
#                 code_id,
#                 Addr::unchecked("owner"),
#                 &Empty {},
#                 &[],
#                 "Contract",
#                 None,
#             )
#             .unwrap();
# 
#         let resp: GreetResp = app
#             .wrap()
#             .query_wasm_smart(addr, &QueryMsg::Greet {})
#             .unwrap();
# 
#         assert_eq!(
#             resp,
#             GreetResp {
#                 message: "Hello World".to_owned()
#             }
#         );
#     }
# }
```

We also need to update the message type on entry point in `src/lib.rs`:

```rust,noplayground
# use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use msg::InstantiateMsg;
// --snip--
# 
# mod contract;
# mod msg;
# mod state;
# 
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}
# 
# #[entry_point]
# pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
#     contract::query(deps, env, msg)
# }
```

Voila, that's all that is needed to update the state!

First, we need to transform the vector of strings into the vector of addresses to be stored. We cannot take
addresses as a message argument because not every string is a valid address. It might be a bit confusing when
we were working on tests. Any string could be used in the place of address. Let me explain.

Every string can be technically considered an address. However, not every string is an actual existing blockchain
address. When we keep anything of type `Addr` in the contract, we assume it is a proper address in the blockchain.
That is why the [`addr_validate`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/trait.Api.html#tymethod.addr_validate)
function exits - to check this precondition.

Having data to store, we use the [`save`](https://docs.rs/cw-storage-plus/0.13.4/cw_storage_plus/struct.Item.html#method.save)
function to write it into the contract state. Note that the first argument of `save` is
[`&mut Storage`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/trait.Storage.html), which is actual blockchain
storage. As emphasized, the `Item` object stores nothing and is just an accessor. It determines how to store the data
in the storage given to it. The second argument is the serializable data to be stored.

It is a good time to check if the regression we have passes - try running our tests:
```
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
```

Damn, we broke something! But be calm. Let's start with carefully reading an error message:

> Error parsing into type contract::msg::InstantiateMsg: missing field `admins`', src/contract.rs:80:14

The problem is that in the test, we send an empty instantiation message in our test, but right now, our endpoint expects
to have an `admin` field. Multi-test framework tests contract from the entry point to results, so sending messages using MT
functions first serializes them. Then the contract deserializes them on the entry. But now it tries to deserialize the
empty JSON to some non-empty message! We can quickly fix it by updating the test:

```rust,noplayground
# use crate::msg::{GreetResp, InstantiateMsg, QueryMsg};
# use crate::state::ADMINS;
# use cosmwasm_std::{
#     to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
# };
# 
# pub fn instantiate(
#     deps: DepsMut,
#     _env: Env,
#     _info: MessageInfo,
#     msg: InstantiateMsg,
# ) -> StdResult<Response> {
#     let admins: StdResult<Vec<_>> = msg
#         .admins
#         .into_iter()
#         .map(|addr| deps.api.addr_validate(&addr))
#         .collect();
#     ADMINS.save(deps.storage, &admins?)?;
# 
#     Ok(Response::new())
# }
# 
# pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
#     use QueryMsg::*;
# 
#     match msg {
#         Greet {} => to_binary(&query::greet()?),
#         AdminsList {} => to_binary(&query::admins_list(deps)?),
#     }
# }
# 
# #[allow(dead_code)]
# pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
#     unimplemented!()
# }
# 
# mod query {
#     use crate::msg::AdminsListResp;
# 
#     use super::*;
# 
#     pub fn greet() -> StdResult<GreetResp> {
#         let resp = GreetResp {
#             message: "Hello World".to_owned(),
#         };
# 
#         Ok(resp)
#     }
# 
#     pub fn admins_list(deps: Deps) -> StdResult<AdminsListResp> {
#         let admins = ADMINS.load(deps.storage)?;
#         let resp = AdminsListResp { admins };
#         Ok(resp)
#     }
# }
# 
# #[cfg(test)]
# mod tests {
#     use cosmwasm_std::Addr;
#     use cw_multi_test::{App, ContractWrapper, Executor};
# 
#     use super::*;
# 
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
# }
```

## Testing state

When the state is initialized, we want a way to test it. We want to provide a
query to check if the instantiation affects the state. Just create a simple one
listing all admins. Start with adding a variant for query message and a corresponding response message in `src/msg.rs`. We'll call the variant `AdminsList`, the response `AdminsListResp`, and have it return a vector of `cosmwasm_std::Addr`:

```rust,noplayground
# use cosmwasm_std::Addr;
# use serde::{Deserialize, Serialize};
# 
# #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
# pub struct InstantiateMsg {
#     pub admins: Vec<Addr>,
# }
# 
# #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
# pub struct GreetResp {
#     pub message: String,
# }
# 
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AdminsListResp  {
    pub admins: Vec<Addr>,
}

[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
    Greet {},
    AdminsList {},
}
```

And implement it in `src/contract.rs`:

```rust,noplayground
use crate::msg::{AdminsListResp, GreetResp, InstantiateMsg, QueryMsg};
# use crate::state::ADMINS;
# use cosmwasm_std::{
#     to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
# };
# 
# pub fn instantiate(
#     deps: DepsMut,
#     _env: Env,
#     _info: MessageInfo,
#     msg: InstantiateMsg,
# ) -> StdResult<Response> {
#     let admins: StdResult<Vec<_>> = msg
#         .admins
#         .into_iter()
#         .map(|addr| deps.api.addr_validate(&addr))
#         .collect();
#     ADMINS.save(deps.storage, &admins?)?;
# 
#     Ok(Response::new())
# }
# 
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {} => to_binary(&query::greet()?),
        AdminsList {} => to_binary(&query::admins_list(deps)?),
    }
}
 
# #[allow(dead_code)]
# pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
#     unimplemented!()
# }
#
mod query {
#    use super::*;
#
#    pub fn greet() -> StdResult<GreetResp> {
#        let resp = GreetResp {
#            message: "Hello World".to_owned(),
#        };
#
#        Ok(resp)
#    }
#
    pub fn admins_list(deps: Deps) -> StdResult<AdminsListResp> {
        let admins = ADMINS.load(deps.storage)?;
        let resp = AdminsListResp { admins };
        Ok(resp)
    }
}

# #[cfg(test)]
# mod tests {
#     use cosmwasm_std::Addr;
#     use cw_multi_test::{App, ContractWrapper, Executor};
# 
#     use super::*;
# 
#     #[test]
#     fn greet_query() {
#        let mut app = App::default();
#
#        let code = ContractWrapper::new(execute, instantiate, query);
#        let code_id = app.store_code(Box::new(code));
#
#        let addr = app
#            .instantiate_contract(
#                code_id,
#                Addr::unchecked("owner"),
#                &InstantiateMsg { admins: vec![] },
#                &[],
#                "Contract",
#                None,
#            )
#            .unwrap();
#
#        let resp: GreetResp = app
#            .wrap()
#            .query_wasm_smart(addr, &QueryMsg::Greet {})
#            .unwrap();
#
#        assert_eq!(
#            resp,
#            GreetResp {
#                message: "Hello World".to_owned()
#            }
#        );
#     }
# }
```

Now when we have the tools to test the instantiation, let's write a test case:

```rust,noplayground
use crate::msg::{AdminsListResp, GreetResp, InstantiateMsg, QueryMsg};
# use crate::state::ADMINS;
# use cosmwasm_std::{
#     to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
# };
# 
# pub fn instantiate(
#     deps: DepsMut,
#     _env: Env,
#     _info: MessageInfo,
#     msg: InstantiateMsg,
# ) -> StdResult<Response> {
#     let admins: StdResult<Vec<_>> = msg
#         .admins
#         .into_iter()
#         .map(|addr| deps.api.addr_validate(&addr))
#         .collect();
#     ADMINS.save(deps.storage, &admins?)?;
# 
#     Ok(Response::new())
# }
# 
# pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
#     use QueryMsg::*;
# 
#     match msg {
#         Greet {} => to_binary(&query::greet()?),
#         AdminsList {} => to_binary(&query::admins_list(deps)?),
#     }
# }
# 
# #[allow(dead_code)]
# pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
#     unimplemented!()
# }
# 
# mod query {
#     use super::*;
# 
#     pub fn greet() -> StdResult<GreetResp> {
#         let resp = GreetResp {
#             message: "Hello World".to_owned(),
#         };
# 
#         Ok(resp)
#     }
# 
#     pub fn admins_list(deps: Deps) -> StdResult<AdminsListResp> {
#         let admins = ADMINS.load(deps.storage)?;
#         let resp = AdminsListResp { admins };
#         Ok(resp)
#     }
# }
# 
#[cfg(test)]
mod tests {
#     use cosmwasm_std::Addr;
#     use cw_multi_test::{App, ContractWrapper, Executor};
# 
#     use super::*;
# 
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
# 
#     #[test]
#     fn greet_query() {
#         let mut app = App::default();
# 
#         let code = ContractWrapper::new(execute, instantiate, query);
#         let code_id = app.store_code(Box::new(code));
# 
#         let addr = app
#             .instantiate_contract(
#                 code_id,
#                 Addr::unchecked("owner"),
#                 &InstantiateMsg { admins: vec![] },
#                 &[],
#                 "Contract",
#                 None,
#             )
#             .unwrap();
# 
#         let resp: GreetResp = app
#             .wrap()
#             .query_wasm_smart(addr, &QueryMsg::Greet {})
#             .unwrap();
# 
#         assert_eq!(
#             resp,
#             GreetResp {
#                 message: "Hello World".to_owned()
#             }
#         );
#     }
}
```

The test is simple - instantiate the contract twice with different initial admins, and ensure the query result
is proper each time. This is often the way we test our contract - we execute bunch o messages on the contract,
and then we query it for some data, verifying if query responses are like expected.

We are doing a pretty good job developing our contract. Now it is time to use the state and allow for some executions.
