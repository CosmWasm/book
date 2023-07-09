# Good practices

All the relevant basics have been covered, so now let's talk about some good practices.

## JSON renaming

Due to Rust style, all our message variants are spelled using
[camel-case](https://en.wikipedia.org/wiki/CamelCase). This is standard practice,
but it has a drawback since all messages are serialized and deserialized by serde
using those variant names. The problem is that in the JSON world it is more common to use [snake
cases](https://en.wikipedia.org/wiki/Snake_case) for field names. Thankfully, there is an effortless way to tell serde to change the names
casing for serialization purposes. Let's update our messages with a `#[serde]`
attribute:

```rust,noplayground
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

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct GreetResp {
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AdminsListResp {
    pub admins: Vec<Addr>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Greet {},
    AdminsList {},
}
```

## JSON schema

Talking of the JSON API, it is worth mentioning JSON Schema. This is a way of defining a shape for JSON messages.
It is good practice to provide a way to generate schemas for our contract API. The problem is that writing JSON
schemas by hand is a pain. The good news is that there is a crate that can help us. Go to the `Cargo.toml` and update as follows:

```toml
[package]
name = "contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
cosmwasm-std = { version = "1.1.4", features = ["staking"] }
serde = { version = "1.0.103", default-features = false, features = ["derive"] }
cw-storage-plus = "0.15.1"
thiserror = "1"
schemars = "0.8.1"
cosmwasm-schema = "1.1.4"

[dev-dependencies]
cw-multi-test = "0.13.4"
```

There is one additional change in this file - in `crate-type` we've added "rlib". "cdylib" crates cannot be used as normal
Rust dependencies. As a consequence, it is impossible to create examples for such crates.

Now go back to `src/msg.rs` and add a new derive for all messages:

```rust,noplayground
# use cosmwasm_std::Addr;
use schemars::JsonSchema;
# use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub donation_denom: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddMembers { admins: Vec<String> },
    Leave {},
    Donate {},
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GreetResp {
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AdminsListResp {
    pub admins: Vec<Addr>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Greet {},
    AdminsList {},
}
```

All those derives look slightly clunky, but thankfully the
[`cosmwasm-schema`](https://docs.rs/cosmwasm-schema/1.1.4/cosmwasm_schema/#)
crate delivers a utility `cw_serde` macro which we can use to reduce the amount of
boilerplate:

```rust,noplayground
# use cosmwasm_std::Addr;
use cosmwasm_schema::cw_serde

#[cw_serde]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub donation_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    AddMembers { admins: Vec<String> },
    Leave {},
    Donate {},
}

#[cw_serde]
pub struct GreetResp {
    pub message: String,
}

#[cw_serde]
pub struct AdminsListResp {
    pub admins: Vec<Addr>,
}

#[cw_serde]
pub enum QueryMsg {
    Greet {},
    AdminsList {},
}
```

Additionally, we have to derive the `QueryResponses` trait for our
query message to be able to correlate the message variants with responses we shall
generate for them:

```rust,noplayground
# use cosmwasm_std::Addr;
use cosmwasm_schema::{cw_serde, QueryResponses}

# #[cw_serde]
# pub struct InstantiateMsg {
#     pub admins: Vec<String>,
#     pub donation_denom: String,
# }
# 
# #[cw_serde]
# pub enum ExecuteMsg {
#     AddMembers { admins: Vec<String> },
#     Leave {},
#     Donate {},
# }
# 
# #[cw_serde]
# pub struct GreetResp {
#     pub message: String,
# }
# 
# #[cw_serde]
# pub struct AdminsListResp {
#     pub admins: Vec<Addr>,
# }
# 
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GreetResp)]
    Greet {},
    #[returns(AdminsListResp)]
    AdminsList {},
}
```

`QueryResponses` is a trait that requires the `#[returns(...)]` attribute
to all your query variants to generate additional information about the
query-response relationship.

Now, we need to make the `msg` module public and accessible by crates that depend
on our contract (in this case - for schema example). Update `src/lib.rs`:

```rust,noplayground
# use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
# use error::ContractError;
# use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
# 
pub mod contract;
pub mod error;
pub mod msg;
pub mod state;
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
# pub fn execute(
#     deps: DepsMut,
#     env: Env,
#     info: MessageInfo,
#     msg: ExecuteMsg,
# ) -> Result<Response, ContractError> {
#     contract::execute(deps, env, info, msg)
# }
# 
# #[entry_point]
# pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
#     contract::query(deps, env, msg)
# }
```

We've changed the visibility of all modules so our crate can now be used as a dependency.
Someone using it as a dependency may need access to handlers or state. 

The next step is to create a tool generating actual schemas. We'll do this by creating
a binary in our crate. Create a new `bin/schema.rs` file:

```rust,noplayground
use contract::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cosmwasm_schema::write_api;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg
    }
}
```

Cargo is smart enough to recognize files in `src/bin` directory as utility
binaries for the crate. Now we can generate our schemas:

```
$ cargo run schema
    Finished dev [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/schema schema`
Removing "/home/hashed/confio/git/book/examples/03-basics/schema/contract.json" …
Exported the full API as /home/hashed/confio/git/book/examples/03-basics/schema/contract.json
```

You're encouraged to go and look at the generated file to see what the schema looks like.

Unforunately, creating this binary makes our project fail
to compile on the Wasm target - which is, in the end, the most important one!
Thankfully, we don't need to build the schema binary for the Wasm target, so let's
align the `.cargo/config` file:

```toml
[alias]
wasm = "build --target wasm32-unknown-unknown --release --lib"
wasm-debug = "build --target wasm32-unknown-unknown --lib"
schema = "run schema"
```

The `--lib` flag added to `wasm` cargo aliases tells the toolchain to build
only the library target and skip building any binaries. In addition we have 
added the convenience `schema` alias so that we can generate the schema simply by calling
`cargo schema`.

## Disabling entry points for libraries

As mentioned before, since we have added the "rlib" target for the contract, it is now useable as a dependency.
The problem is that a contract dependent on ours would have Wasm entry points generated twice - once
in the dependency and once in the final contract! We can work around this by disabling generating Wasm
entry points for the contract if the crate is used as a dependency. We use
[feature flags](https://doc.rust-lang.org/cargo/reference/features.html) for this.

Start with updating `Cargo.toml`:

```toml
[features]
library = []
```

The above adjustment creates a new feature for our crate. Next we want to disable the `entry_point` attribute on
entry points, which we will do with a slight update of `src/lib.rs`:

```rust,noplayground
# use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
# use error::ContractError;
# use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
# 
# pub mod contract;
# pub mod error;
# pub mod msg;
# pub mod state;
# 
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    contract::execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    contract::query(deps, env, msg)
}
```

The [`cfg_attr`](https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute) attribute is
a conditional compilation attribute, similar to the `cfg` we used before for the test. It expands to the given attribute if
the condition expands to true. In our case, it would expand to nothing if the feature "library" is enabled, otherwise
it would expand to `#[entry_point]`.

When adding this contract as a dependency, don't forget to enable the feature like this:

```toml
[dependencies]
my_contract = { version = "0.1", features = ["library"] }
```
