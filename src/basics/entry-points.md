# Entry points

Typical Rust application starts with the `fn main()` function called by the operating system.
Smart contracts are not significantly different. When the message is sent to the contract, a
function called "entry point" is called. Unlike native applications, which have only a single
`main` entry point, smart contracts have a couple corresponding to different message types:
`instantiate`, `execute`, `query`, `sudo`, `migrate` and more.

To start, we will go with three basic entry points:

* `instantiate` which is called once per smart contract lifetime - you can think about it as
  a constructor or initializer of a contract.
* `execute` for handling messages which are able to modify contract state - they are used to
  perform some actual actions.
* `query` for handling messages requesting some information from a contract; unlike `execute`,
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

In fact, `instantiate` is the only entry point required for a smart contract to be valid. It is not
very useful in this form, but it is a start. Let's take a closer look at the entry point structure.

First, we start with importing couple of types just for more consistent usage. Then we define our
entry point. The `instantiate` takes four arguments:

* [`deps: DepsMut`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.DepsMut.html)
  is a utility type for communicating with the outer world - it allows querying
  and updating the contract state, querying other contracts state, and gives access to an `Api`
  object with a couple of helper functions for dealing with CW addresses.
* [`env: Env`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Env.html)
  is an object representing the blockchains state when executing the message - the
  chain height and id, current timestamp, and the called contract address.
* [`info: MessageInfo`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.MessageInfo.html)
  contains metainformation about the message which triggered an execution -
  an address that sends the message, and chain native tokens sent with the message.
* [`msg: Empty`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Empty.html)
  is the message triggering execution itself - for now, it is `Empty` type that
  represents `{}` JSON, but the type of this argument can be anything that is deserializable,
  and we will pass more complex types here in the future.

If you are new to the blockchain, those arguments may not have much sense to you, but while
progressing with this guide, I will explain their usage one by one.

Notice an essential attribute decorating our entry point
[`#[entry_point]`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/attr.entry_point.html). Its purpose is to
wrap the whole entry point to the form Wasm runtime understands. The proper Wasm entry points
can use only basic types supported natively by Wasm specification, and Rust structures and enums
are not in this set. Working with such entry points would be rather overcomplicated, so CosmWasm
creators delivered the `entry_point` macro. It creates the raw Wasm entry point, calling the
decorated function internally and doing all the magic required to build our high-level Rust arguments
from arguments passed by Wasm runtime.

The next thing to look at is the return type. I used
[`StdResult<Response>`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/type.StdResult.html) for this simple example,
which is an alias for `Result<Response, StdError>`. The return entry point type would always be a
[`Result`](https://doc.rust-lang.org/std/result/enum.Result.html) type, with some error type implementing
[`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html) trait and a well-defined type for success
case. For most entry points, an "Ok" case would be the
[`Response`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Response.html) type that allows fitting the contract
into our actor model, which we will discuss very soon.

The body of the entry point is as simple as it could be - it always succeeds with a trivial empty response.

# **Entry Points in CosmWasm Smart Contracts**

In CosmWasm, the concept of an entry point is fundamental to the operation of smart contracts. Unlike traditional Rust applications that start with a **`fn main()`** function, smart contracts utilize specific functions called entry points to interact with the blockchain. These entry points are crucial for the lifecycle of a smart contract, allowing it to be deployed, executed, and queried securely and predictably.

### **Overview of Entry Points**

Entry points in CosmWasm smart contracts are defined functions the blockchain calls in response to various actions, such as a contract's deployment, message execution, or information requests. The primary entry points are:

1. **[Instantiate](https://chat.openai.com/c/dcbf6b7b-aadb-452d-9e1d-cae2b89df3bd#instantiate)**
2. **[Execute](https://chat.openai.com/c/dcbf6b7b-aadb-452d-9e1d-cae2b89df3bd#execute)**
3. **[Query](https://chat.openai.com/c/dcbf6b7b-aadb-452d-9e1d-cae2b89df3bd#query)**

Each entry point serves a specific purpose in the contract's lifecycle and interaction with the blockchain.

### **Instantiate Function**

The **`instantiate`** function acts as the smart contract's constructor. It is called once when the contract is first deployed to the blockchain, allowing for the initial setup of its state and configurations.

```rust
#[entry_point]
pub fn instantiate(
deps: [DepsMut](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.DepsMut.html),
env: [Env](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Env.html),
info: [MessageInfo](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.MessageInfo.html),
msg: [InstantiateMsg](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.InstantiateMsg.html),
) -> [StdResult<Response>](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/type.StdResult.html) {
// Contract initialization logic
Ok(Response::new().add_attribute("method", "instantiate"))
}
```

This function sets the foundation of the contract, establishing necessary initial conditions or parameters.

### **Execute Function**

The **`execute`** function is where the contract's business logic resides. It processes messages that cause state changes within the contract, such as updating data or transferring tokens.

```rust
#[entry_point]
pub fn execute(
    deps: [DepsMut](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.DepsMut.html),
    env: [Env](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Env.html),
    info: [MessageInfo](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.MessageInfo.html),
    msg: [ExecuteMsg](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.ExecuteMsg.html),
) -> [StdResult<Response>](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/type.StdResult.html) {
    // Handling of different messages to perform state changes
    Ok(Response::new().add_attribute("action", "execute"))
}
```

This function can be invoked multiple times throughout the contract's life, responding to user or contract interactions.

### **Query Function**

The **`query`** function allows reading data from the contract without modifying its state. It fetches information, like contract configurations or current state details, based on the query message received.

```rust
#[entry_point]{
pub fn query(
    deps: [Deps](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Deps.html),
    env: [Env](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.Env.html),
    msg: [QueryMsg](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.QueryMsg.html),
) -> [StdResult<Binary>](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/type.StdResult.html) {
    // Logic to handle different queries
    to_binary(&"Query response")
  
}
```

Queries are essential for external clients or contracts to understand the contract's current state or configurations without initiating a state change.

### **Implementing Entry Points**

When defining these entry points in CosmWasm, the **`#[entry_point]`** attribute is used to designate the corresponding functions. Developers must ensure that each function correctly processes its intended operations, adhering to the expected inputs and outputs defined by the CosmWasm standard.

### **Conclusion**

Understanding and implementing entry points are critical for developing smart contracts in CosmWasm. These functions enable the contract to interact seamlessly with the blockchain, ensuring it can be initialized, executed, and queried as intended. By following the guidelines outlined in this chapter, developers can create efficient, secure, and functional smart contracts for the CosmWasm ecosystem.