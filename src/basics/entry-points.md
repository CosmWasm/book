# Entry points

A typical Rust application starts with a `fn main()` function called by the operating system.
Smart contracts are not very different. When the message is sent to the contract, a
function called an "entry point" is called. Unlike native applications, which have only a single
`main` entry point, smart contracts have a number of them, corresponding to different message
types: `instantiate`, `execute`, `query`, `sudo`, `migrate` and others.

To start, we will go with three basic entry points:

* `instantiate` which is called once per smart contract lifetime. You can think of it as
  a constructor or initializer of a contract.
* `execute` for handling messages which are able to modify the contract's state. They are used to
  perform actions.
* `query` for handling messages requesting some information from a contract. Unlike `execute`,
  they can never affect any contract state, and are used just like database queries.

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

`instantiate` is actually the only entry point required for a smart contract to be valid. The contract is of course not
particularly useful in this form, but it's a start! Let's take a closer look at the entry point structure.

We start with importing a couple of types just for consistency, followed by defining our
entry point. The `instantiate` function takes four arguments:

* [`deps: DepsMut`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.DepsMut.html)
  is a utility type for communicating with the outer world - it allows querying
  and updating the contract state, querying the state of other contracts, and also gives access to an `Api`
  object with a couple of helper functions for dealing with CosmWasm addresses.
* [`env: Env`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Env.html)
  is an object representing the blockchains state when executing the message - the
  chain height and id, current timestamp, and the address of the called contract.
* [`info: MessageInfo`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.MessageInfo.html)
  contains meta-information about the message which triggered the execution -
  the address that sent the message, and any chain native tokens sent with the message.
* [`msg: Empty`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Empty.html)
  is the message triggering the execution itself.  For now, it is just `Empty`
  (represented by `{}` in JSON), but the type of this argument can be anything that is deserializable,
  and we will pass more complex types here in the future.

If you are new to the blockchain, those arguments may not make much sense to you yet but as we progress
through this guide we shall explain the usage of each in turn.

Notice an essential attribute decorating our entry point
[`#[entry_point]`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/attr.entry_point.html). Its purpose is to
wrap the whole entry point so that it is in a form that the Wasm runtime understands. The real Wasm entry points
can use only basic types supported natively by the Wasm specification, and Rust structures and enums
are not in this set. Working with such entry points would be unecessarily complicated, so CosmWasm's
creators created the `entry_point` macro. It creates the raw Wasm entry point, calling the
decorated function internally and doing all the magic required to build our high-level Rust arguments
from arguments passed by Wasm runtime.

The next thing to look at is the return type. We used
[`StdResult<Response>`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/type.StdResult.html) for this simple example,
which is an alias for `Result<Response, StdError>`. The return entry point type should always be a
[`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) type, with some error type implementing the
[`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html) trait and a well-defined type for the success
case. For most entry points, an "Ok" case would be the
[`Response`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Response.html) that allows the contract to fit into our "Actor Model", which we shall discuss very soon.

The body of the entry point is as simple as it can be - it always succeeds with a trivial empty response.
