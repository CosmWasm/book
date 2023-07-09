# Creating a query

We have already created a simple contract reacting to an empty `instantiate` message. Unfortunately, it's
not very useful. Let's make it a bit more reactive.

First, we need to add the [`serde`](https://crates.io/crates/serde) crate to our dependencies. It will help us with the serialization and deserialization of query messages. Update the `Cargo.toml`:

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

[dev-dependencies]
cw-multi-test = "0.13.4"
```

Now go to your `src/lib.rs` file, and add a new query entry point:

```rust,noplayground
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo,
    Response, StdResult,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct QueryResp {
    message: String,
}

# #[entry_point]
# pub fn instantiate(
#     _deps: DepsMut,
#     _env: Env,
#     _info: MessageInfo,
#     _msg: Empty,
# ) -> StdResult<Response> {
#     Ok(Response::new())
# }
#
#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
    let resp = QueryResp {
        message: "Hello World".to_owned(),
    };

    to_binary(&resp)
}
```

Note that we omitted the previously created `instantiate` endpoint for simplicity so as
not to overload you with code. From now on, we shall often only show lines in the code that actually
changed.

We first need a structure we will return from our query. Since we always want to return something
which is serializable, here we are just deriving the
[`Serialize`](https://docs.serde.rs/serde/trait.Serialize.html) and
[`Deserialize`](https://docs.serde.rs/serde/trait.Deserialize.html) traits using the `serde` crate.

Next we need to implement our entry point. It is very similar to the `instantiate` one. The
first significant difference is an argument of type `Deps`. For `instantiate`, it was a `DepMut`,
but here we use a [`Deps`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Deps.html)
object. This is because the query can never alter the smart
contract's internal state. It can only read the state. This has some consequences - for example,
it is impossible to implement caching for future queries (as this would require some sort of data cache
to write to).

The other difference is the lack of the `info` argument. An entry point which
performs actions (such as instantiation or execution) may alter how the action is performed based
on the message metadata - for example checking who can perform an action - but this is not the case queries. Queries are purely supposed to return some
transformed contract state. It can be calculated based on some chain metadata (so the state can
"automatically" change after some time), but is not based on message info.

Note that our entry point still has the same `Empty` type for its `msg` argument.  This means that the
query message we would send to the contract is still an empty JSON: `{}`

The last thing that we've changed is the return type. Instead of returning the `Response` type in the success
case, we return an arbitrary serializable object. This is because queries are not using a typical Actor
Model message flow (discussed later) and they cannot trigger any actions nor communicate with other contracts other than just
querying them (which is handled by the `Deps` argument). The query always returns plain data, which
should be presented directly to the enquirer.

Now take a look at the implementation. Nothing complicated happens here - we create an object we want
to return and encode it to the [`Binary`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Binary.html)
type using the [`to_binary`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/fn.to_binary.html) function.

## Improving the message

Although we have a query, there is a problem with the query message. It is always an empty JSON. This isn't 
great - if we would like to add another query in the future, it would be difficult to distinguish
between query variants.

In practice, we address this by using a non-empty query message type. Here is an improved version of our contract:

```rust,noplayground
# use cosmwasm_std::{
#     entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
# };
# use serde::{Deserialize, Serialize};
# 
# #[derive(Serialize, Deserialize)]
# struct QueryResp {
#     message: String,
# }
#
#[derive(Serialize, Deserialize)]
pub enum QueryMsg {
    Greet {},
}

# #[entry_point]
# pub fn instantiate(
#     _deps: DepsMut,
#     _env: Env,
#     _info: MessageInfo,
#     _msg: Empty,
# ) -> StdResult<Response> {
#     Ok(Response::new())
# }
#
#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {} => {
            let resp = QueryResp {
                message: "Hello World".to_owned(),
            };

            to_binary(&resp)
        }
    }
}
```

Now we have introduced a proper message type for the query message. It is an
[enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html), and by
default, it would serialize to a JSON with a single field. The name of the field
will be an enum variant (in our case - always "Greet" - at least for now). The
value of this field would be an object assigned to this enum variant.

Note that our enum has no type assigned to the only `Greet` variant. Typically
in Rust, we create such variants without the additional `{}` after the variant name. Here the
curly braces serve a purpose: without them, the variant would serialize to just a string
type - so instead of `{ "greet": {} }`, the JSON representation of this variant would be
`"greet"`. This behavior leads to inconsistency in the message schema. It is generally
a good habit to always add the `{}` to serde-serializable empty enum variants in order to
ensure better and more consistent JSON representation.

We can improve the code still further. Right now, the `query` function has two
responsibilities. The first is obvious - handling the query itself. This was the first
assumption, and it is still there. But there is another thing happening here as well - the query
message dispatching. It may not be obvious, as there is just a single variant in our code, but
the query function is an excellent way to create massive unreadable `match` statements. To make
the code more [SOLID](https://en.wikipedia.org/wiki/SOLID), we will refactor it and
give responsibility for handling of the `Greet` message to a separate function.

```rust,noplayground
# use cosmwasm_std::{
#     entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
# };
# use serde::{Deserialize, Serialize};
# 
#[derive(Serialize, Deserialize)]
pub struct GreetResp {
    message: String,
}

# #[derive(Serialize, Deserialize)]
# pub enum QueryMsg {
#     Greet {},
# }
# 
# #[entry_point]
# pub fn instantiate(
#     _deps: DepsMut,
#     _env: Env,
#     _info: MessageInfo,
#     _msg: Empty,
# ) -> StdResult<Response> {
#     Ok(Response::new())
# }
# 
#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {} => to_binary(&query::greet()?),
    }
}

mod query {
    use super::*;

    pub fn greet() -> StdResult<GreetResp> {
        let resp = GreetResp {
            message: "Hello World".to_owned(),
        };

        Ok(resp)
    }
}
```

Now it looks much better. Note that there are a couple of additional improvements here as well. We
renamed the query-response type `GreetResp` as we may have different responses
for different queries. We want the name to relate only to the variant, not the
whole message.

The other improvement involves enclosing the new function in the module `query`. This
makes it easier to avoid name collisions - we can have the same variant for queries and
execution messages in the future, and their handlers would lie in separate namespaces.

A questionable decision may be returning `StdResult` instead of `GreetResp`
from the `greet` function, as it would never return an error. It is a matter of
style, since you may prefer consistency over the message handler as the majority of
them would have failure cases e.g. when reading the state.

Also, you might pass `deps` and `env` arguments to all your query handlers for
consistency. On the one hand, you may consider that it introduces unnecessary boilerplate
which doesn't read well. It's up to you - we leave it up to your judgment.

## Structuring the contract

You can see that our contract is becoming a bit bigger now. About 50 lines are maybe
not so much, but there are many different entities in a single file, and we
can do better. We can already distinguish three different types of entity in the code:
entry points, messages, and handlers. In most contracts, we would divide them across
three files. Let's start with extracting all the messages to `src/msg.rs`:

```rust,noplayground
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GreetResp {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub enum QueryMsg {
    Greet {},
}
```

You'll have noticed that the `GreetResp` fields are now public. This is because they now
have to be accessed from a different module. Next we can move on to the `src/contract.rs` file:

```rust,noplayground
use crate::msg::{GreetResp, QueryMsg};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {} => to_binary(&query::greet()?),
    }
}

mod query {
    use super::*;

    pub fn greet() -> StdResult<GreetResp> {
        let resp = GreetResp {
            message: "Hello World".to_owned(),
        };

        Ok(resp)
    }
}
```

We have moved most of the logic here, so that `src/lib.rs` is now just a very thin library entry with nothing
other than module definitions and entry point definitions. The `#[entry_point]` attribute
has been removed from the `query` function in `src/contract.rs`. We shall appply this attribute to another function instead. The main objective here was to split the functions even further by responsibility: now the `contract::query` function is the top-level query handler responsible for dispatching the query message, while the `query` function
on crate-level is only an entry point. It is a subtle distinction, but will make sense in the future
when we would like to keep dispatching functions without generating entry points. The split is introduced
at this point in order to demonstrate the typical contract structure.

Finally, the `src/lib.rs` file:

```rust,noplayground
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

mod contract;
mod msg;

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty)
  -> StdResult<Response>
{
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg)
  -> StdResult<Binary>
{
    contract::query(deps, env, msg)
}
```

This is a straightforward top-level module, containing nothing more than the definition of submodules and entry points.

Now that we have the contract ready to do something, let's go ahead and test it!