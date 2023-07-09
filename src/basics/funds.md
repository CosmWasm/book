# Dealing with funds

When you hear smart contracts, you think blockchain. When you hear blockchain,
you often think of cryptocurrencies. Although they are not exactly the same thing, crypto assets (or
`tokens` as they are often called) are indeed very closely connected to the blockchain.
CosmWasm has the notion of a native token. Native tokens are assets managed by
the blockchain core instead of by smart contracts. Often such assets have some
special use-case, such as being used for paying [gas
fees](https://docs.cosmos.network/master/basics/gas-fees.html) or
[staking](https://en.wikipedia.org/wiki/Proof_of_stake) for a consensus
algorithm, but they can also be just any arbitrary assets.

Native tokens are assigned to their owners but can also be transferred. Everything that has an address on the blockchain is eligible to have (own) native tokens. As a consequence - tokens can also be assigned to smart contracts!
Every message sent to the smart contract can have some funds sent along with it. In
this chapter, we will take advantage of this and create a way to reward hard
work performed by admins. We will create a new message - `Donate`, which can be
used by anyone to donate some funds to admins, which will be divided equally between them.

## Preparing messages

As before, we need to prepare our messages. We need to create a new
`ExecuteMsg` variant, but we will also modify the `Instantiate` message a bit as we need some way of defining the name of the native token we'll use
for donations. It would be possible to allow users to send any tokens they
want, but we want to simplify things for now.

```rust,noplayground
# use cosmwasm_std::Addr;
# use serde::{Deserialize, Serialize};
# 
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub donation_denom: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ExecuteMsg {
    AddMembers { admins: Vec<String> },
    Leave {},
    Donate {},
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

We also need to add a new state part, to keep the `donation_denom`:

```rust,noplayground
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");
pub const DONATION_DENOM: Item<String> = Item::new("donation_denom");
```

And instantiate it properly:

```rust,noplayground
# use crate::error::ContractError;
# use crate::msg::{AdminsListResp, ExecuteMsg, GreetResp, InstantiateMsg, QueryMsg};
use crate::state::{ADMINS, DONATION_DENOM};
# use cosmwasm_std::{
#     to_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
# };

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
    DONATION_DENOM.save(deps.storage, &msg.donation_denom)?;

    Ok(Response::new())
}
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
#         let events = admins
#             .iter()
#             .map(|admin| Event::new("admin_added").add_attribute("addr", admin));
#         let resp = Response::new()
#             .add_events(events)
#             .add_attribute("action", "add_members")
#             .add_attribute("added_count", admins.len().to_string());
# 
#         let admins: StdResult<Vec<_>> = admins
#             .into_iter()
#             .map(|addr| deps.api.addr_validate(&addr))
#             .collect();
# 
#         curr_admins.append(&mut admins?);
#         ADMINS.save(deps.storage, &curr_admins)?;
# 
#         Ok(resp)
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
# 
#     #[test]
#     fn unauthorized() {
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
#         let err = app
#             .execute_contract(
#                 Addr::unchecked("user"),
#                 addr,
#                 &ExecuteMsg::AddMembers {
#                     admins: vec!["user".to_owned()],
#                 },
#                 &[],
#             )
#             .unwrap_err();
# 
#         assert_eq!(
#             ContractError::Unauthorized {
#                 sender: Addr::unchecked("user")
#             },
#             err.downcast().unwrap()
#         );
#     }
# 
#     #[test]
#     fn add_members() {
#         let mut app = App::default();
# 
#         let code = ContractWrapper::new(execute, instantiate, query);
#         let code_id = app.store_code(Box::new(code));
# 
#         let addr = app
#             .instantiate_contract(
#                 code_id,
#                 Addr::unchecked("owner"),
#                 &InstantiateMsg {
#                     admins: vec!["owner".to_owned()],
#                 },
#                 &[],
#                 "Contract",
#                 None,
#             )
#             .unwrap();
# 
#         let resp = app
#             .execute_contract(
#                 Addr::unchecked("owner"),
#                 addr,
#                 &ExecuteMsg::AddMembers {
#                     admins: vec!["user".to_owned()],
#                 },
#                 &[],
#             )
#             .unwrap();
# 
#         let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();
#         assert_eq!(
#             wasm.attributes
#                 .iter()
#                 .find(|attr| attr.key == "action")
#                 .unwrap()
#                 .value,
#             "add_members"
#         );
#         assert_eq!(
#             wasm.attributes
#                 .iter()
#                 .find(|attr| attr.key == "added_count")
#                 .unwrap()
#                 .value,
#             "1"
#         );
# 
#         let admin_added: Vec<_> = resp
#             .events
#             .iter()
#             .filter(|ev| ev.ty == "wasm-admin_added")
#             .collect();
#         assert_eq!(admin_added.len(), 1);
# 
#         assert_eq!(
#             admin_added[0]
#                 .attributes
#                 .iter()
#                 .find(|attr| attr.key == "addr")
#                 .unwrap()
#                 .value,
#             "user"
#         );
#     }
# }
```

What also need to update some of the tests since instantiate messages now have a new field. We leave this to you as an exercise.

Now we have everything we need to implement donating funds to admins. First, a minor update to the `Cargo.toml` as we
will use an additional utility crate:

```toml
[package]
name = "contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[features]
library = []

[dependencies]
cosmwasm-std = { version = "1.0.0-beta8", features = ["staking"] }
serde = { version = "1.0.103", default-features = false, features = ["derive"] }
cw-storage-plus = "0.13.4"
thiserror = "1"
schemars = "0.8.1"
cw-utils = "0.13"

[dev-dependencies]
cw-multi-test = "0.13.4"
cosmwasm-schema = { version = "1.0.0" }
```

Then we can implement the donate handler:

```rust,noplayground
# use crate::error::ContractError;
# use crate::msg::{AdminsListResp, ExecuteMsg, GreetResp, InstantiateMsg, QueryMsg};
# use crate::state::{ADMINS, DONATION_DENOM};
use cosmwasm_std::{
    coins, to_binary, BankMsg, Binary, Deps, DepsMut, Env, Event, MessageInfo,
    Response, StdResult,
};
 
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
#     DONATION_DENOM.save(deps.storage, &msg.donation_denom)?;
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
        Donate {} => exec::donate(deps, info),
    }
}

mod exec {
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
#         let events = admins
#             .iter()
#             .map(|admin| Event::new("admin_added").add_attribute("addr", admin));
#         let resp = Response::new()
#             .add_events(events)
#             .add_attribute("action", "add_members")
#             .add_attribute("added_count", admins.len().to_string());
# 
#         let admins: StdResult<Vec<_>> = admins
#             .into_iter()
#             .map(|addr| deps.api.addr_validate(&addr))
#             .collect();
# 
#         curr_admins.append(&mut admins?);
#         ADMINS.save(deps.storage, &curr_admins)?;
# 
#         Ok(resp)
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
# 
    pub fn donate(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let denom = DONATION_DENOM.load(deps.storage)?;
        let admins = ADMINS.load(deps.storage)?;

        let donation = cw_utils::must_pay(&info, &denom)?.u128();

        let donation_per_admin = donation / (admins.len() as u128);

        let messages = admins.into_iter().map(|admin| BankMsg::Send {
            to_address: admin.to_string(),
            amount: coins(donation_per_admin, &denom),
        });

        let resp = Response::new()
            .add_messages(messages)
            .add_attribute("action", "donate")
            .add_attribute("amount", donation.to_string())
            .add_attribute("per_admin", donation_per_admin.to_string());

        Ok(resp)
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
#                 &InstantiateMsg {
#                     admins: vec![],
#                     donation_denom: "eth".to_owned(),
#                 },
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
#                     donation_denom: "eth".to_owned(),
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
#                 &InstantiateMsg {
#                     admins: vec![],
#                     donation_denom: "eth".to_owned(),
#                 },
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
#     #[test]
#     fn unauthorized() {
#         let mut app = App::default();
# 
#         let code = ContractWrapper::new(execute, instantiate, query);
#         let code_id = app.store_code(Box::new(code));
# 
#         let addr = app
#             .instantiate_contract(
#                 code_id,
#                 Addr::unchecked("owner"),
#                 &InstantiateMsg {
#                     admins: vec![],
#                     donation_denom: "eth".to_owned(),
#                 },
#                 &[],
#                 "Contract",
#                 None,
#             )
#             .unwrap();
# 
#         let err = app
#             .execute_contract(
#                 Addr::unchecked("user"),
#                 addr,
#                 &ExecuteMsg::AddMembers {
#                     admins: vec!["user".to_owned()],
#                 },
#                 &[],
#             )
#             .unwrap_err();
# 
#         assert_eq!(
#             ContractError::Unauthorized {
#                 sender: Addr::unchecked("user")
#             },
#             err.downcast().unwrap()
#         );
#     }
# 
#     #[test]
#     fn add_members() {
#         let mut app = App::default();
# 
#         let code = ContractWrapper::new(execute, instantiate, query);
#         let code_id = app.store_code(Box::new(code));
# 
#         let addr = app
#             .instantiate_contract(
#                 code_id,
#                 Addr::unchecked("owner"),
#                 &InstantiateMsg {
#                     admins: vec!["owner".to_owned()],
#                     donation_denom: "eth".to_owned(),
#                 },
#                 &[],
#                 "Contract",
#                 None,
#             )
#             .unwrap();
# 
#         let resp = app
#             .execute_contract(
#                 Addr::unchecked("owner"),
#                 addr,
#                 &ExecuteMsg::AddMembers {
#                     admins: vec!["user".to_owned()],
#                 },
#                 &[],
#             )
#             .unwrap();
# 
#         let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();
#         assert_eq!(
#             wasm.attributes
#                 .iter()
#                 .find(|attr| attr.key == "action")
#                 .unwrap()
#                 .value,
#             "add_members"
#         );
#         assert_eq!(
#             wasm.attributes
#                 .iter()
#                 .find(|attr| attr.key == "added_count")
#                 .unwrap()
#                 .value,
#             "1"
#         );
# 
#         let admin_added: Vec<_> = resp
#             .events
#             .iter()
#             .filter(|ev| ev.ty == "wasm-admin_added")
#             .collect();
#         assert_eq!(admin_added.len(), 1);
# 
#         assert_eq!(
#             admin_added[0]
#                 .attributes
#                 .iter()
#                 .find(|attr| attr.key == "addr")
#                 .unwrap()
#                 .value,
#             "user"
#         );
#     }
# }
```

Sending the funds to another contract is performed by adding bank messages to
the response. The blockchain will expect any message that is returned in the
contract response as a part of an execution. This design is related to the "Actor
Model" implemented by CosmWasm. The model will be described in
detail later. For now, you can assume that this is a convenient way to handle token transfers.
Before sending the tokens to the admins, we have to calculate the amount of donation
per admin. This is done by searching the funds for an entry describing our donation
token and dividing the number of tokens sent by the number of admins. Since
the integral division always rounds down, it is possible that not all of the tokens sent as a donation would
end up being sent an admin's account. Any remainder would be left on our contract
account forever. There are a number of different ways of dealing with this issue - figuring
out one of them will be a great exercise!

The last missing part is updating the `ContractError` - the `must_pay` call
returns a `cw_utils::PaymentError` that we can't convert to our custom error type
yet:

```rust,noplayground
use cosmwasm_std::{Addr, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("{sender} is not contract admin")]
    Unauthorized { sender: Addr },
    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),
}
```

As you can see, to handle incoming funds we used the utility function - you're 
encouraged to take a look at [its
implementation](https://docs.rs/cw-utils/0.13.4/src/cw_utils/payment.rs.html#32-39).  This should give you a good understanding of how incoming funds are structured in `MessageInfo`.

Now it's time to check if the funds are being distributed correctly by writing another test.

```rust,noplayground
# use crate::error::ContractError;
# use crate::msg::{AdminsListResp, ExecuteMsg, GreetResp, InstantiateMsg, QueryMsg};
# use crate::state::{ADMINS, DONATION_DENOM};
# use cosmwasm_std::{
#     coins, to_binary, BankMsg, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
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
#     DONATION_DENOM.save(deps.storage, &msg.donation_denom)?;
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
#         Donate {} => exec::donate(deps, info),
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
#         let events = admins
#             .iter()
#             .map(|admin| Event::new("admin_added").add_attribute("addr", admin));
#         let resp = Response::new()
#             .add_events(events)
#             .add_attribute("action", "add_members")
#             .add_attribute("added_count", admins.len().to_string());
# 
#         let admins: StdResult<Vec<_>> = admins
#             .into_iter()
#             .map(|addr| deps.api.addr_validate(&addr))
#             .collect();
# 
#         curr_admins.append(&mut admins?);
#         ADMINS.save(deps.storage, &curr_admins)?;
# 
#         Ok(resp)
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
# 
#     pub fn donate(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
#         let denom = DONATION_DENOM.load(deps.storage)?;
#         let admins = ADMINS.load(deps.storage)?;
# 
#         let donation = cw_utils::must_pay(&info, &denom)
#             .map_err(|err| StdError::generic_err(err.to_string()))?
#             .u128();
# 
#         let donation_per_admin = donation / (admins.len() as u128);
# 
#         let messages = admins.into_iter().map(|admin| BankMsg::Send {
#             to_address: admin.to_string(),
#             amount: coins(donation_per_admin, &denom),
#         });
# 
#         let resp = Response::new()
#             .add_messages(messages)
#             .add_attribute("action", "donate")
#             .add_attribute("amount", donation.to_string())
#             .add_attribute("per_admin", donation_per_admin.to_string());
# 
#         Ok(resp)
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
#                 &InstantiateMsg {
#                     admins: vec![],
#                     donation_denom: "eth".to_owned(),
#                 },
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
#                     donation_denom: "eth".to_owned(),
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
#                 &InstantiateMsg {
#                     admins: vec![],
#                     donation_denom: "eth".to_owned(),
#                 },
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
#     #[test]
#     fn unauthorized() {
#         let mut app = App::default();
# 
#         let code = ContractWrapper::new(execute, instantiate, query);
#         let code_id = app.store_code(Box::new(code));
# 
#         let addr = app
#             .instantiate_contract(
#                 code_id,
#                 Addr::unchecked("owner"),
#                 &InstantiateMsg {
#                     admins: vec![],
#                     donation_denom: "eth".to_owned(),
#                 },
#                 &[],
#                 "Contract",
#                 None,
#             )
#             .unwrap();
# 
#         let err = app
#             .execute_contract(
#                 Addr::unchecked("user"),
#                 addr,
#                 &ExecuteMsg::AddMembers {
#                     admins: vec!["user".to_owned()],
#                 },
#                 &[],
#             )
#             .unwrap_err();
# 
#         assert_eq!(
#             ContractError::Unauthorized {
#                 sender: Addr::unchecked("user")
#             },
#             err.downcast().unwrap()
#         );
#     }
# 
#     #[test]
#     fn add_members() {
#         let mut app = App::default();
# 
#         let code = ContractWrapper::new(execute, instantiate, query);
#         let code_id = app.store_code(Box::new(code));
# 
#         let addr = app
#             .instantiate_contract(
#                 code_id,
#                 Addr::unchecked("owner"),
#                 &InstantiateMsg {
#                     admins: vec!["owner".to_owned()],
#                     donation_denom: "eth".to_owned(),
#                 },
#                 &[],
#                 "Contract",
#                 None,
#             )
#             .unwrap();
# 
#         let resp = app
#             .execute_contract(
#                 Addr::unchecked("owner"),
#                 addr,
#                 &ExecuteMsg::AddMembers {
#                     admins: vec!["user".to_owned()],
#                 },
#                 &[],
#             )
#             .unwrap();
# 
#         let wasm = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();
#         assert_eq!(
#             wasm.attributes
#                 .iter()
#                 .find(|attr| attr.key == "action")
#                 .unwrap()
#                 .value,
#             "add_members"
#         );
#         assert_eq!(
#             wasm.attributes
#                 .iter()
#                 .find(|attr| attr.key == "added_count")
#                 .unwrap()
#                 .value,
#             "1"
#         );
# 
#         let admin_added: Vec<_> = resp
#             .events
#             .iter()
#             .filter(|ev| ev.ty == "wasm-admin_added")
#             .collect();
#         assert_eq!(admin_added.len(), 1);
# 
#         assert_eq!(
#             admin_added[0]
#                 .attributes
#                 .iter()
#                 .find(|attr| attr.key == "addr")
#                 .unwrap()
#                 .value,
#             "user"
#         );
#     }
# 
    #[test]
    fn donations() {
        let mut app = App::new(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &Addr::unchecked("user"), coins(5, "eth"))
                .unwrap()
        });

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec!["admin1".to_owned(), "admin2".to_owned()],
                    donation_denom: "eth".to_owned(),
                },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::Donate {},
            &coins(5, "eth"),
        )
        .unwrap();

        assert_eq!(
            app.wrap()
                .query_balance("user", "eth")
                .unwrap()
                .amount
                .u128(),
            0
        );

        assert_eq!(
            app.wrap()
                .query_balance(&addr, "eth")
                .unwrap()
                .amount
                .u128(),
            1
        );

        assert_eq!(
            app.wrap()
                .query_balance("admin1", "eth")
                .unwrap()
                .amount
                .u128(),
            2
        );

        assert_eq!(
            app.wrap()
                .query_balance("admin2", "eth")
                .unwrap()
                .amount
                .u128(),
            2
        );
    }
}
```

This is fairly straightforward, though you may not particularly appreciate that every balance check is
eight lines of code!  It can be improved by enclosing this assertion in a
separate function, probably with the use of the
[`#[track_caller]`](https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-track_caller-attribute)
attribute.

The critical thing to discuss is how `app` creation changed. As we need
some initial tokens in a `user`'s account, instead of using the default
constructor we have to provide it with an initializer function. Unfortunately, the
[`new`](https://docs.rs/cw-multi-test/0.13.4/cw_multi_test/struct.App.html#method.new)
documentation is not very easy to follow, even if our required function is not very
complicated. What it takes as an argument is a closure with three arguments -
the
[`Router`](https://docs.rs/cw-multi-test/0.13.4/cw_multi_test/struct.Router.html)
with all modules supported by multi-test, the API object, and the state. This
function is called once during contract instantiation. The `router` object
contains some generic fields - we are interested in `bank` in particular. It
has a type of
[`BankKeeper`](https://docs.rs/cw-multi-test/0.13.4/cw_multi_test/struct.BankKeeper.html),
where the
[`init_balance`](https://docs.rs/cw-multi-test/0.13.4/cw_multi_test/struct.BankKeeper.html#method.init_balance)
function sits.

## Plot Twist!

As we have now covered the most important basics required to buildi smart contracts in Rust, we have a serious exercise for you.

The contract we built has an exploitable bug. All donations are distributed equally across admins. However, every
admin is eligible to add another admin... and nothing is preventing the admin from adding himself to the list and
receiving twice as many rewards as the others!

Try to write a test that detects such a bug, then fix it and ensure the bug no longer occurs!

Of course, even if the admin cannot add the same address to the list, they could always create new accounts and add them, but this
is something unpreventable on the contract level, so you don't need to prevent that here. Handling this kind of case is done by the
proper design of whole applications which is out of scope for this chapter.
