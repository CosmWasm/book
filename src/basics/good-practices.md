# Good practices

All the relevant basics are covered. Now let's talk about some good practices.

## JSON renaming

Due to Rust style, all the message variants we create are spelled in a
[camel-case](https://en.wikipedia.org/wiki/CamelCase). It is standard practice,
but it has a drawback - all messages are serialized and deserialized by serde using those variant names. The
problem is that it is more common to use [snake cases](https://en.wikipedia.org/wiki/Snake_case) for field
names in the JSON world. Hopefully, there is an
effortless way to tell serde, to change the names casing for serialization purposes. Let's update our messages
with a `#[serde]` attribute:

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

Talking about JSON API, it is worth mentioning JSON Schema. It is a way of defining a shape for JSON messages.
It is good practice to provide a way to generate schemas for contract API. The problem is that writing JSON
schemas by hand is a pain. The good news is that there is a crate that would help us with that. Go to the `Cargo.toml`:

```toml
[package]
name = "contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
cosmwasm-std = { version = "1.0.0-beta8", features = ["staking"] }
serde = { version = "1.0.103", default-features = false, features = ["derive"] }
cw-storage-plus = "0.13.4"
thiserror = "1"
schemars = "0.8.1"

[dev-dependencies]
cw-multi-test = "0.13.4"
cosmwasm-schema = { version = "1.0.0" }
```

There is one additional change in this file - in `crate-type` I added "rlib". "cdylib" crates cannot be used as typical
Rust dependencies. As a consequence, it is impossible to create examples for such crates.

Now go back to `src/msg.rs` and add new derive for all messages:

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

We also want to make `msg` module public and accessible by crates depending on our contract (in this case - for
schema example). Update a `src/lib.rs`:

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

I changed the visibility of all modules - as our crate can now be used as a dependency.
If someone would like to do so, he may need access to handlers or state. 

The next step is to create a tool generating actual schemas. We will do it by creating
an example application. Create new `examples/schema.rs` file:

```rust,noplayground
use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{
    export_schema, export_schema_with_title,
    remove_schemas, schema_for
};

use contract::msg::*;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema_with_title(
        &schema_for!(InstantiateMsg), &out_dir, "InstantiateMsg"
    );
    export_schema_with_title(
        &schema_for!(ExecuteMsg), &out_dir, "ExecuteMsg"
    );
    export_schema_with_title(
        &schema_for!(QueryMsg), &out_dir, "QueryMsg"
    );
    export_schema(&schema_for!(AdminsListResp), &out_dir);
    export_schema(&schema_for!(GreetResp), &out_dir);
}
```

Now we can generate our schemas:

```
$ cargo run --example schema

...
Created /home/hashed/confio/git/book/examples/03-basics/schema/instantiate_msg.json
Created /home/hashed/confio/git/book/examples/03-basics/schema/execute_msg.json
Created /home/hashed/confio/git/book/examples/03-basics/schema/query_msg.json
Created /home/hashed/confio/git/book/examples/03-basics/schema/admins_list_resp.json
Created /home/hashed/confio/git/book/examples/03-basics/schema/greet_resp.json
```

I encourage you to go to generated files to see what the schema looks like.

Now it's time to last touch - we can add an alias for schema generation. Go to `.cargo/config`
and add new entry:

```toml
[alias]
wasm = "build --target wasm32-unknown-unknown --release"
wasm-debug = "build --target wasm32-unknown-unknown"
schema = "run --example schema"
```

Now you can generate a schema with a simple `cargo schema`.

## Disabling entry points for libraties

Since we added the "rlib" target for the contract, it is, as mentioned before, useable as a dependency.
The problem is that the contract depended on ours would have Wasm entry points generated twice - once
in the dependency and once in the final contract. We can work this around by disabling generating Wasm
entry points for the contract if the crate is used as a dependency. We would use
[feature flags](https://doc.rust-lang.org/cargo/reference/features.html) for that.

Start with updating `Cargo.toml`:

```toml
[features]
library = []
```

This way, we created a new feature for our crate. Now we want to disable the `entry_point` attribute on
entry points - we will do it by a slight update of `src/lib.rs`:

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
a conditional compilation attribute, similar to `cfg` we used before for the test. It expands to the given attribute if
the condition expands to true. In our case - it would expand to nothing if the feature "library" is enabled, or it
would expand just to `#[entry_point]` in another case.

Since now to add this contract as a dependency, don't forget to enable the feature like this:

```toml
[dependencies]
my_contract = { version = "0.1", features = ["library"] }
```
