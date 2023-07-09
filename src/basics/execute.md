# Execution messages

We have been through instantiate and query messages. It is finally time to introduce the last basic entry point -
the execute message. It is similar to what we have done so far, so you can expect this to be quite easy as we are just revisiting our knowledge. We encourage you to try implementing what is described on your own first before checking the source code as an exercise .

The idea of the contract is simple: every contract admin is eligible to call the following two execute messages:

* `AddMembers` admins can call this message to add another address to the admin's list
* `Leave` admins can call this message to remove themselves from the admin list

That doesn't sound too complicated, so let's get coding! We'll start with defining the messages:

```rust,noplayground
# use cosmwasm_std::Addr;
# use serde::{Deserialize, Serialize};
# 
# #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
# pub struct InstantiateMsg {
#     pub admins: Vec<String>,
# }
# 
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ExecuteMsg {
    AddMembers { admins: Vec<String> },
    Leave {},
}
# 
# #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
# pub struct GreetResp {
#     pub message: String,
# }
# 
# #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
# pub struct AdminsListResp {
#     pub admins: Vec<Addr>,
# }
# 
# #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
# pub enum QueryMsg {
#     Greet {},
#     AdminsList {},
# }
```

Next, we implement the entry point:

```rust,noplayground
use crate::msg::{AdminsListResp, ExecuteMsg, GreetResp, InstantiateMsg, QueryMsg};
# use crate::state::ADMINS;
# use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
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
 
#[allow(dead_code)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    use ExecuteMsg::*;

    match msg {
        AddMembers { admins } => exec::add_members(deps, info, admins),
        Leave {} => exec::leave(deps, info),
    }
}

mod exec {
    use cosmwasm_std::StdError;

    use super::*;

    pub fn add_members(
        deps: DepsMut,
        info: MessageInfo,
        admins: Vec<String>,
    ) -> StdResult<Response> {
        let mut curr_admins = ADMINS.load(deps.storage)?;
        if !curr_admins.contains(&info.sender) {
            return Err(StdError::generic_err("Unauthorised access"));
        }

        let admins: StdResult<Vec<_>> = admins
            .into_iter()
            .map(|addr| deps.api.addr_validate(&addr))
            .collect();

        curr_admins.append(&mut admins?);
        ADMINS.save(deps.storage, &curr_admins)?;

        Ok(Response::new())
    }

    pub fn leave(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        ADMINS.update(deps.storage, move |admins| -> StdResult<_> {
            let admins = admins
                .into_iter()
                .filter(|admin| *admin != info.sender)
                .collect();
            Ok(admins)
        })?;

        Ok(Response::new())
    }
}
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
# #[cfg(test)]
# mod tests {
#     use cosmwasm_std::Addr;
#     use cw_multi_test::{App, ContractWrapper, Executor};
# 
#     use crate::msg::AdminsListResp;
# 
#     use super::*;
# 
#     #[test]
#     fn instantiation() {
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
#         let resp: AdminsListResp = app
#             .wrap()
#             .query_wasm_smart(addr, &QueryMsg::AdminsList {})
#             .unwrap();
# 
#         assert_eq!(resp, AdminsListResp { admins: vec![] });
# 
#         let addr = app
#             .instantiate_contract(
#                 code_id,
#                 Addr::unchecked("owner"),
#                 &InstantiateMsg {
#                     admins: vec!["admin1".to_owned(), "admin2".to_owned()],
#                 },
#                 &[],
#                 "Contract 2",
#                 None,
#             )
#             .unwrap();
# 
#         let resp: AdminsListResp = app
#             .wrap()
#             .query_wasm_smart(addr, &QueryMsg::AdminsList {})
#             .unwrap();
# 
#         assert_eq!(
#             resp,
#             AdminsListResp {
#                 admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")],
#             }
#         );
#     }
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
# }
```

The real entry point itself also has to be created in `src/lib.rs`:

```rust,noplayground
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

mod contract;
mod msg;
mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    contract::execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    contract::query(deps, env, msg)
}
```

There are a couple of new things here, but nothing major. The first is how we handled getting the identity of the message sender is so that we could verify that they are actually allowed to carry out the execute functions.  We used the `info.sender` field of `MessageInfo`, which is exactly as it sounds like. As the message is always sent from a proper address, the
`sender` is already of the `Addr` type and there is no need to validate it. The other new thing is the
[`update`](https://docs.rs/cw-storage-plus/0.13.4/cw_storage_plus/struct.Item.html#method.update)
function on an `Item` - it makes the read and update of an entity potentially more efficient. This would also be possible by reading the admins first, followed by updating and storing the result.

You may have noticed that when working with `Item`, we have been always assuming that something
is actually there. However, this may not be warranted, since there's nothing forcing us to initialize the `ADMINS` value on
instantiation! So what would happen in then? Well, both `load` and `update` functions
would return an error. However, there is also the option of using the
[`may_load`](https://docs.rs/cw-storage-plus/0.13.4/cw_storage_plus/struct.Item.html#method.may_load)
function, which returns `StdResult<Option<T>>`.  In the case of empty storage, it returns `Ok(None)`. It is also possible to remove an existing item from storage with the
[`remove`](https://docs.rs/cw-storage-plus/0.13.4/cw_storage_plus/struct.Item.html#method.remove)
function.

One thing we need to improve is the error handling. While validating that the sender is an admin, we are just returning
some arbitrary string as an error. We can do better!

## Error handling

In our contract, we now generate an error whenever a non-admin user tries to execute `AddMembers`. There is no proper error case in
[`StdError`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/enum.StdError.html) to report this
situation, so we have resorted to just returning a generic error with a message, which is not the best approach.

For error reporting, we recommend using the [`thiserror`](https://crates.io/crates/thiserror/1.0.24/dependencies)
crate. Start with updating your dependencies:

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
thiserror = "1"

[dev-dependencies]
cw-multi-test = "0.13.4"
```

Now we define an error type in `src/error.rs`:

```rust,noplayground
use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("{sender} is not contract admin")]
    Unauthorized { sender: Addr },
}
```

We also need to add the new module to `src/lib.rs`:

```rust,noplayground
# use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
# use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
# 
mod contract;
mod error;
mod msg;
mod state;
# 
# #[entry_point]
# pub fn instantiate(
#     deps: DepsMut,
#     env: Env,
#     info: MessageInfo,
#     msg: InstantiateMsg,
# ) -> StdResult<Response> {
#     contract::instantiate(deps, env, info, msg)
# }
# 
# #[entry_point]
# pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
#     contract::execute(deps, env, info, msg)
# }
# 
# #[entry_point]
# pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
#     contract::query(deps, env, msg)
# }
```

Using `thiserror` we define errors just in a simple enum, and the crate ensures
that the type implements the
[`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
trait. A very nice feature of this crate is the inline definition of the
[`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) trait by use of an
`#[error]` attribute. Another helpful thing is the `#[from]` attribute,
which automatically generates a proper
[`From`](https://doc.rust-lang.org/std/convert/trait.From.html) implementation,
allowing us to use the `?` operator with `thiserror` types.

Now we can update the execute endpoint to use our new error type:

```rust,noplayground
use crate::error::ContractError;
use crate::msg::{AdminsListResp, ExecuteMsg, GreetResp, InstantiateMsg, QueryMsg};
# use crate::state::ADMINS;
# use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
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
 
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        AddMembers { admins } => exec::add_members(deps, info, admins),
        Leave {} => exec::leave(deps, info).map_err(Into::into),
    }
}

mod exec {
    use super::*;

    pub fn add_members(
        deps: DepsMut,
        info: MessageInfo,
        admins: Vec<String>,
    ) -> Result<Response, ContractError> {
        let mut curr_admins = ADMINS.load(deps.storage)?;
        if !curr_admins.contains(&info.sender) {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }

        let admins: StdResult<Vec<_>> = admins
            .into_iter()
            .map(|addr| deps.api.addr_validate(&addr))
            .collect();

        curr_admins.append(&mut admins?);
        ADMINS.save(deps.storage, &curr_admins)?;

        Ok(Response::new())
    }
# 
#     pub fn leave(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
#         ADMINS.update(deps.storage, move |admins| -> StdResult<_> {
#             let admins = admins
#                 .into_iter()
#                 .filter(|admin| *admin != info.sender)
#                 .collect();
#             Ok(admins)
#         })?;
# 
#         Ok(Response::new())
#     }
}
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
# #[cfg(test)]
# mod tests {
#     use cosmwasm_std::Addr;
#     use cw_multi_test::{App, ContractWrapper, Executor};
# 
#     use crate::msg::AdminsListResp;
# 
#     use super::*;
# 
#     #[test]
#     fn instantiation() {
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
#         let resp: AdminsListResp = app
#             .wrap()
#             .query_wasm_smart(addr, &QueryMsg::AdminsList {})
#             .unwrap();
# 
#         assert_eq!(resp, AdminsListResp { admins: vec![] });
# 
#         let addr = app
#             .instantiate_contract(
#                 code_id,
#                 Addr::unchecked("owner"),
#                 &InstantiateMsg {
#                     admins: vec!["admin1".to_owned(), "admin2".to_owned()],
#                 },
#                 &[],
#                 "Contract 2",
#                 None,
#             )
#             .unwrap();
# 
#         let resp: AdminsListResp = app
#             .wrap()
#             .query_wasm_smart(addr, &QueryMsg::AdminsList {})
#             .unwrap();
# 
#         assert_eq!(
#             resp,
#             AdminsListResp {
#                 admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")],
#             }
#         );
#     }
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
# }
```

The entry point return type also has to be updated:

```rust,noplayground
# use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use error::ContractError;
# use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

# mod contract;
# mod error;
# mod msg;
# mod state;
# 
# #[entry_point]
# pub fn instantiate(
#     deps: DepsMut,
#     env: Env,
#     info: MessageInfo,
#     msg: InstantiateMsg,
# ) -> StdResult<Response> {
#     contract::instantiate(deps, env, info, msg)
# }
# 
#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    contract::execute(deps, env, info, msg)
}
# 
# #[entry_point]
# pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
#     contract::query(deps, env, msg)
# }
```

## Custom error and multi-test

Using a proper custom error type has one more nice advantage - multi-test maintains the error type using
the [`anyhow`](https://crates.io/crates/anyhow) crate. This is a sibling of `thiserror`, designed
to implement type-erased errors in a way that allows us to get the original error back.

Let's write a test that verifies that a non-admin cannot add themselves to the list:

```rust,noplayground
# use crate::error::ContractError;
# use crate::msg::{AdminsListResp, ExecuteMsg, GreetResp, InstantiateMsg, QueryMsg};
# use crate::state::ADMINS;
# use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
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
# pub fn execute(
#     deps: DepsMut,
#     _env: Env,
#     info: MessageInfo,
#     msg: ExecuteMsg,
# ) -> Result<Response, ContractError> {
#     use ExecuteMsg::*;
# 
#     match msg {
#         AddMembers { admins } => exec::add_members(deps, info, admins),
#         Leave {} => exec::leave(deps, info).map_err(Into::into),
#     }
# }
# 
# mod exec {
#     use super::*;
# 
#     pub fn add_members(
#         deps: DepsMut,
#         info: MessageInfo,
#         admins: Vec<String>,
#     ) -> Result<Response, ContractError> {
#         let mut curr_admins = ADMINS.load(deps.storage)?;
#         if !curr_admins.contains(&info.sender) {
#             return Err(ContractError::Unauthorized {
#                 sender: info.sender,
#             });
#         }
# 
#         let admins: StdResult<Vec<_>> = admins
#             .into_iter()
#             .map(|addr| deps.api.addr_validate(&addr))
#             .collect();
# 
#         curr_admins.append(&mut admins?);
#         ADMINS.save(deps.storage, &curr_admins)?;
# 
#         Ok(Response::new())
#     }
# 
#     pub fn leave(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
#         ADMINS.update(deps.storage, move |admins| -> StdResult<_> {
#             let admins = admins
#                 .into_iter()
#                 .filter(|admin| *admin != info.sender)
#                 .collect();
#             Ok(admins)
#         })?;
# 
#         Ok(Response::new())
#     }
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
#     use crate::msg::AdminsListResp;
# 
#     use super::*;
# 
#     #[test]
#     fn instantiation() {
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
#         let resp: AdminsListResp = app
#             .wrap()
#             .query_wasm_smart(addr, &QueryMsg::AdminsList {})
#             .unwrap();
# 
#         assert_eq!(resp, AdminsListResp { admins: vec![] });
# 
#         let addr = app
#             .instantiate_contract(
#                 code_id,
#                 Addr::unchecked("owner"),
#                 &InstantiateMsg {
#                     admins: vec!["admin1".to_owned(), "admin2".to_owned()],
#                 },
#                 &[],
#                 "Contract 2",
#                 None,
#             )
#             .unwrap();
# 
#         let resp: AdminsListResp = app
#             .wrap()
#             .query_wasm_smart(addr, &QueryMsg::AdminsList {})
#             .unwrap();
# 
#         assert_eq!(
#             resp,
#             AdminsListResp {
#                 admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")],
#             }
#         );
#     }
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
# 
    #[test]
    fn unauthorized() {
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

        let err = app
            .execute_contract(
                Addr::unchecked("user"),
                addr,
                &ExecuteMsg::AddMembers {
                    admins: vec!["user".to_owned()],
                },
                &[],
            )
            .unwrap_err();

        assert_eq!(
            ContractError::Unauthorized {
                sender: Addr::unchecked("user")
            },
            err.downcast().unwrap()
        );
    }
}
```

Executing a contract is very similar to any other call - we use an
[`execute_contract`](https://docs.rs/cw-multi-test/0.13.4/cw_multi_test/trait.Executor.html#method.execute_contract)
function. As the execution may fail, we get an error type out of this call, but
instead of calling `unwrap` to extract a value out of it, we make use of the
[`unwrap_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_err)
call since we are expecting an error to
occur. Now that we have an error value, we can check whether it matches what we
expected with an `assert_eq!`. There is a slight complication here, however: the error
returned from `execute_contract` is an
[`anyhow::Error`](https://docs.rs/anyhow/1.0.57/anyhow/struct.Error.html)
error, but we are expecting a `ContractError`. Thankfully, as mentioned above,
`anyhow` errors can recover their original type using the
[`downcast`](https://docs.rs/anyhow/1.0.57/anyhow/struct.Error.html#method.downcast)
function. The `unwrap` right after it is required because downcasting may fail, because `downcast` doesn't magically know the type kept in the
underlying error. It deduces this by using the context - in this case, it knows we expect it
to be a `ContractError` by the fact that it is being compared to it (type elision
miracles!). If the underlying error was not a `ContractError`, then
`unwrap` would panic.

We have just created a simple failure test for execution, but it is not enough by itself to enable us to claim that the contract is production-ready.
We would need to cover all reasonable Ok-cases to be able to claim that. We encourage you to create some tests and experiment with them as an exercise after this chapter.
