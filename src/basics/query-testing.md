# Testing a query

Last time we created a new query, now it is time to test it out. We will start with the basics -
the unit test. This approach is simple and doesn't require knowledge besides Rust. Go to the
`src/contract.rs` and add a test in its module:

```rust,noplayground
# use crate::msg::{GreetResp, QueryMsg};
# use cosmwasm_std::{
#     to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
# };
# 
# pub fn instantiate(
#     _deps: DepsMut,
#     _env: Env,
#     _info: MessageInfo,
#     _msg: Empty,
# ) -> StdResult<Response> {
#     Ok(Response::new())
# }
# 
# pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
#     use QueryMsg::*;
# 
#     match msg {
#         Greet {} => to_binary(&query::greet()?),
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
# }
# 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_query() {
        let resp = query::greet().unwrap();
        assert_eq!(
            resp,
            GreetResp {
                message: "Hello World".to_owned()
            }
        );
    }
}
```

If you ever wrote a unit test in Rust, nothing should surprise you here. Just a
simple test-only module contains local function unit tests. The problem is - this
test doesn't build yet. We need to tweak our message types a bit. Update the `src/msg.rs`:

```rust,noplayground
# use serde::{Deserialize, Serialize};
# 
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GreetResp {
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum QueryMsg {
    Greet {},
}
```

I added three new derives to both message types. [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)
is required to allow comparing types
for equality - so we can check if they are equal. The [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
is a trait generating debug-printing
utilities. It is used by [`assert_eq!`](https://doc.rust-lang.org/std/macro.assert_eq.html) to
display information about mismatch if an assertion
fails. Note that because we are not testing the `QueryMsg` in any way, the additional trait derives
are optional. Still, it is a good practice to make all messages both `PartialEq` and `Debug` for
testability and consistency.
The last one, [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) is not needed for now yet,
but it is also good practice to allow messages to be cloned around. We will also require that
later, so I added it already not to go back and forth.

Now we are ready to run our test:

```
$ cargo test

...
running 1 test
test contract::tests::greet_query ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Yay! Test passed!

## Contract as a black box

Now let's go a step further. The Rust testing utility is a friendly tool for building even higher-level
tests. We are currently testing smart contract internals, but if you think about how your smart contract
is visible from the outside world. It is a single entity that is triggered by some input messages. We can
create tests that treat the whole contract as a black box by testing it via our `query` function. Let's
update our test:

```rust,noplayground
# use crate::msg::{GreetResp, QueryMsg};
# use cosmwasm_std::{
#     to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
# };
# 
# pub fn instantiate(
#     _deps: DepsMut,
#     _env: Env,
#     _info: MessageInfo,
#     _msg: Empty,
# ) -> StdResult<Response> {
#     Ok(Response::new())
# }
# 
# pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
#     use QueryMsg::*;
# 
#     match msg {
#         Greet {} => to_binary(&query::greet()?),
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
# }
# 
#[cfg(test)]
mod tests {
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    use super::*;

    #[test]
    fn greet_query() {
        let resp = query(
            mock_dependencies().as_ref(),
            mock_env(),
            QueryMsg::Greet {}
        ).unwrap();
        let resp: GreetResp = from_binary(&resp).unwrap();

        assert_eq!(
            resp,
            GreetResp {
                message: "Hello World".to_owned()
            }
        );
    }
}
```

We needed to produce two entities for the `query` functions: the `deps` and `env` instances.
Hopefully, `cosmwasm-std` provides utilities for testing those -
[`mock_dependencies`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/testing/fn.mock_dependencies.html)
and [`mock_env`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/testing/fn.mock_env.html)
functions.

You may notice the dependencies mock of a type
[`OwnedDeps`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.OwnedDeps.html) instead
of `Deps`, which we need here - this is why the
[`as_ref`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.OwnedDeps.html#method.as_ref)
function is called on it. If we looked for a `DepsMut` object, we would use
[`as_mut`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.OwnedDeps.html#method.as_mut)
instead.

We can rerun the test, and it should still pass. But when we think about that test reflecting
the actual use case, it is inaccurate. The contract is queried, but it was never instantiated!
In software engineering, it is equivalent to calling a getter without constructing an object -
taking it out of nowhere. It is a lousy testing approach. We can do better:

```rust,noplayground

# use crate::msg::{GreetResp, QueryMsg};
# use cosmwasm_std::{
#     to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
# };
# 
# pub fn instantiate(
#     _deps: DepsMut,
#     _env: Env,
#     _info: MessageInfo,
#     _msg: Empty,
# ) -> StdResult<Response> {
#     Ok(Response::new())
# }
# 
# pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
#     use QueryMsg::*;
# 
#     match msg {
#         Greet {} => to_binary(&query::greet()?),
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
# }
# 
#[cfg(test)]
mod tests {
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use super::*;

    #[test]
    fn greet_query() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        instantiate(
            deps.as_mut(),
            env.clone(),
            mock_info("sender", &[]),
            Empty {},
        )
        .unwrap();

        let resp = query(deps.as_ref(), env, QueryMsg::Greet {}).unwrap();
        let resp: GreetResp = from_binary(&resp).unwrap();
        assert_eq!(
            resp,
            GreetResp {
                message: "Hello World".to_owned()
            }
        );
    }
}
```

A couple of new things here. First, I extracted the `deps` and `env` variables to their variables
and passed them to calls. The idea is that those variables represent some blockchain persistent state,
and we don't want to create them for every call. We want any changes to the contract state occurring
in `instantiate` to be visible in the `query`. Also, we want to control how the environment differs
on the query and instantiation.

The `info` argument is another story. The message info is unique for each message sent. To create the
`info` mock, we must pass two arguments to the
[`mock_info`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/testing/fn.mock_info.html) function.

First is the address performing a call. It may look strange to pass `sender` as an address instead of some
mysterious `wasm` followed by hash, but it is a valid address. For testing purposes, such addresses are
typically better, as they are way more verbose in case of failing tests.

The second argument is funds sent with the message. For now, we leave it as an empty slice, as I don't want
to talk about token transfers yet - we will cover it later.

So now it is more a real-case scenario. I see just one problem. I say that the contract is a single black
box. But here, nothing connects the `instantiate` call to the corresponding `query`. It seems that we assume
there is some global contract. But it seems that if we would like to have two contracts instantiated differently
in a single test case, it would become a mess. If only there would be some tool to abstract this for us, wouldn't
it be nice?
