# Creating a query

We have already created a simple contract reacting to an empty instantiate message. Unfortunately, it
is not very useful. Let's make it a bit reactive.

First, we need to add [`serde`](https://crates.io/crates/serde) crate to our dependencies. It would help us with the serialization and
deserialization of query messages. Update the `Cargo.toml`:

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

Note that I omitted the previously created instantiate endpoint for simplicity -
not to overload you with code, I will always only show lines that changed in the code.

We first need a structure we will return from our query. We always want to return something
which is serializable. We are just deriving the
[`Serialize`](https://docs.serde.rs/serde/trait.Serialize.html) and
[`Deserialize`](https://docs.serde.rs/serde/trait.Deserialize.html) traits from `serde` crate.

Then we need to implement our entry point. It is very similar to the `instantiate` one. The
first significant difference is a type of `deps` argument. For `instantiate`, it was a `DepMut`,
but here we went with a [`Deps`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Deps.html)
object. That is because the query can never alter the smart
contract's internal state. It can only read the state. It comes with some consequences - for example,
it is impossible to implement caching for future queries (as it would require some data cache to write
to).

The other difference is the lack of the `info` argument. The reason here is that the entry point which
performs actions (like instantiation or execution) can differ how an action is performed based on the
message metadata - for example, they can limit who can perform an action (and do so by checking the
message `sender`). It is not a case for queries. Queries are supposed just purely to return some
transformed contract state. It can be calculated based on some chain metadata (so the state can
"automatically" change after some time), but not on message info.

Note that our entry point still has the same `Empty` type for its `msg` argument - it means that the
query message we would send to the contract is still an empty JSON: `{}`

The last thing that changed is the return type. Instead of returning the `Response` type on the success
case, we return an arbitrary serializable object. This is because queries are not using a typical actor
model message flow - they cannot trigger any actions nor communicate with other contracts in ways different
than querying them (which is handled by the `deps` argument). The query always returns plain data, which
should be presented directly to the querier.

Now take a look at the implementation. Nothing complicated happens there - we create an object we want
to return and encode it to the [`Binary`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Binary.html)
type using the [`to_binary`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/fn.to_binary.html) function.

## Improving the message

We have a query, but there is a problem with the query message. It is always an empty JSON. It is terrible -
if we would like to add another query in the future, it would be difficult to have any reasonable distinction
between query variants.

In practice, we address this by using a non-empty query message type. Improve our contract:

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

Now we introduced a proper message type for the query message. It is an
[enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html), and by
default, it would serialize to a JSON with a single field - the name of the field
will be an enum variant (in our case - always "greet" - at least for now), and the
value of this field would be an object assigned to this enum variant.

Note that our enum has no type assigned to the only `Greet` variant. Typically
in Rust, we create such variants without additional `{}` after the variant name. Here the
curly braces have a purpose - without them, the variant would serialize to just a string
type - so instead of `{ "greet": {} }`, the JSON representation of this variant would be
`"greet"`. This behavior brings inconsistency in the message schema. It is, generally,
a good habit to always add the `{}` to serde serializable empty enum variants - for better
JSON representation.

But now, we can still improve the code further. Right now, the `query` function has two
responsibilities. The first is obvious - handling the query itself. It was the first
assumption, and it is still there. But there is a new thing happening there - the query
message dispatching. It may not be obvious, as there is a single variant, but the query
function is an excellent way to become a massive unreadable `match` statement. To make
the code more [SOLID](https://en.wikipedia.org/wiki/SOLID), we will refactor it and
take out handling the `greet` message to a
separate function.

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

Now it looks much better. Note there are a couple of additional improvements. I
renamed the query-response type `GreetResp` as I may have different responses
for different queries. I want the name to relate only to the variant, not the
whole message.

Next is enclosing my new function in the module `query`. It makes it easier to
avoid name collisions - I can have the same variant for queries and execution
messages in the future, and their handlers would lie in separate namespaces.

A questionable decision may be returning `StdResult` instead of `GreetResp`
from `greet` function, as it would never return an error. It is a matter of
style, but I prefer consistency over the message handler, and the majority of
them would have failure cases - e.g. when reading the state.

Also, you might pass `deps` and `env` arguments to all your query handlers for
consistency. I'm not too fond of this, as it introduces unnecessary boilerplate
which doesn't read well, but I also agree with the consistency argument - I
leave it to your judgment.

## Structuring the contract

You can see that our contract is becoming a bit bigger now. About 50 lines are maybe
not so much, but there are many different entities in a single file, and I think we
can do better. I can already distinguish three different types of entities in the code:
entry points, messages, and handlers. In most contracts, we would divide them across
three files. Start with extracting all the messages to `src/msg.rs`:

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

You probably noticed that I made my `GreetResp` fields public. It is because they have
to be now accessed from a different module. Now move forward to the `src/contract.rs` file:

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

I moved most of the logic here, so my `src/lib.rs` is just a very thin library entry with nothing
else but module definitions and entry points definition. I removed the `#[entry_point]` attribute
from my `query` function in `src/contract.rs`. I will have a function with this attribute.
Still, I wanted to split functions' responsibility further - not the `contract::query` function is
the top-level query handler responsible for dispatching the query message. The `query` function on
crate-level is only an entry point. It is a subtle distinction, but it will make sense in the future
when we would like not to generate the entry points but to keep the dispatching functions. I introduced
the split now to show you the typical contract structure.

Now the last part, the `src/lib.rs` file:

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

Straightforward top-level module. Definition of submodules and entry points, nothing more.

Now, when we have the contract ready to do something, let's go and test it.
