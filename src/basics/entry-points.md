# Entry points

Typical Rust application starts with the `fn main()` function called by the operating system.
Smart contracts are not significantly different. When the message is sent to the contract, a
function called "entry point" is called. Unlike native applications, which have only a single
`main` entry point, smart contracts have a couple corresponding to different message types:
`instantiate`, `execute`, `query`, `sudo`, `migrate` and more.

For start, we will go with three basic entry points:

* `instantiate` which is called once per smart contract lifetime - you can think about it as
  a constructor or initializer of a contract.
* `execute` for handling messages which are able to modify contract state - they are used to
  perform some actual actions.
* `query` for handling messages requesting some information from a contract; unlike `execute`,
  they never can affect any contract state, and they are used just like database queries.

Go to your `src/lib.rs` file, and start with an `instantiate` entry point:

```rust,noplayground
use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}
```

In fact, `instantiate` is the only entry point required for smart contract to be valid. It is not
very useful in this form, but it is a start. Let's take closer look at the entry point structure.

First, we start with importing couple of types just for more consistent usage. Then we define our
entry point. The `instantiate` takes four arguments:

* `deps: DepsMut` is a utility type for communicating with the outer world - it allows querying
  and updating the contract state, querying other contracts state, and gives access to an `Api`
  object with a couple of helper functions for dealing with CW addresses.
* `env: Env` is an object representing the blockchains state when executing the message - the
  chain height and id, current timestamp, and the called contract address.
* `info: MessageInfo` contains metainformation about the message which triggered an execution -
  an address that sends the message, and chain native tokens sent with the message.
* `msg: Empty` is the message triggering execution itself - for now, it is `Empty` type that
  represents `{}` JSON, but the type of this argument can be anything that is deserializable,
  and we will pass more complex types here in future.

If you are new to the blockchain, those arguments may not have much sense to you, but while
progressing with this guide, I will explain their usage of them one by one.

Notice an essential attribute decorating our entry point `#[entry_point]`. Its purpose is to
wrap the whole entry point to the form Wasm runtime understands. The proper Wasm entry points
can use only basic types supported natively by Wasm specification, and Rust structures and enums
are not in this set. Working with such entry points would be rather overcomplicated, so CosmWasm
creators delivered the `entry_point` macro. It creates the raw Wasm entry point, calling the
decorated function internally and doing all the magic required to build our high-level Rust arguments
from arguments passed by Wasm runtime.

The next thing to look at is the return type. I used `StdResult<Response>` for this simple example,
which is an alias for `Result<Response, StdError>`. The return entry point type would always be a
`Result` type, with some error type implementing `ToString` trait and a well-defined type for success
case. For most entry points, an "Ok" case would be the `Response` type that allows fitting the contract
into our actor model, which we will discuss very soon.

The body of the entry point is as simple as it could be - it always succeeds with a trivial empty response.
