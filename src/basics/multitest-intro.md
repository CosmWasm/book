# Introducing multitest

Let me introduce the [`multitest`](https://crates.io/crates/cw-multi-test) -
library for creating tests for smart contracts in Rust.

The core idea of `multitest` is abstracting an entity of contract and
simulating the blockchain environment for testing purposes. The purpose of this
is to be able to test communication between smart contracts. It does its job
well, but it is also an excellent tool for testing single-contract scenarios.

First, we need to add a multitest to our `Cargo.toml`.

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

I added a new
[`[dev-dependencies]`](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies)
section with dependencies not used by the final binary
but which may be used by tools around the development process - for example, tests.

When we have the dependency ready, update our test to use the framework:

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
#[allow(dead_code)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty
) -> StdResult<Response> {
    unimplemented!()
}

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
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use super::*;

    #[test]
    fn greet_query() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &Empty {},
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: GreetResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Greet {})
            .unwrap();

        assert_eq!(
            resp,
            GreetResp {
                message: "Hello World".to_owned()
            }
        );
    }
}
```

You probably notice that I added the function for an `execute` entry point. I didn't add the entry point
itself or the function's implementation, but for the multitest purposes contract has to contain at least
instantiate, query, and execute handlers. I attributed the function as
[`#[allow(dead_code)]`](https://doc.rust-lang.org/reference/attributes/diagnostics.html#lint-check-attributes),
so, `cargo` will not complain about it not being used anywhere. Enabling it for tests only with `#[cfg(test)]`
would also be a way.

Then at the beginning of the test, I created the
[`App`](https://docs.rs/cw-multi-test/0.13.4/cw_multi_test/struct.App.html#)
object. It is a core multitest entity representing the virtual blockchain on
which we will run our contracts. As you can see, we can call functions on it
just like we would interact with blockchain using `wasmd`!

Right after creating `app`, I prepared the representation of the `code`, which
would be "uploaded" to the blockchain. As multitests are just native Rust
tests, they do not involve any Wasm binaries, but this name matches well what
happens in a real-life scenario. We store this object in the blockchain with
the [`store_code`](https://docs.rs/cw-multi-test/0.13.4/cw_multi_test/struct.App.html#method.store_code)
function, and as a result, we are getting the code id - we would need it to
instantiate a contract.

Instantiation is the next step. In a single
[`instantiate_contract`](https://docs.rs/cw-multi-test/0.13.4/cw_multi_test/trait.Executor.html#method.instantiate_contract)
call, we provide everything we would provide via `wasmd` - the contract code id, the address which performs instantiation,

the message triggering it, and any funds sent with the message (again - empty for now). We are adding the contract label
and its admin for migrations - `None`, as we don't need it yet.

And after the contract is online, we can query it. The
[`wrap`](https://docs.rs/cw-multi-test/0.13.4/cw_multi_test/struct.App.html?search=in#method.wrap) function is an accessor
for querying Api (queries are handled a bit differently than other calls), and the
[`query_wasm_smart`](https://docs.rs/cosmwasm-std/1.0.0/cosmwasm_std/struct.QuerierWrapper.html#method.query_wasm_smart)
queries are given a contract with the message. Also, we don't need to care about query results as `Binary` - multitest
assumes that we would like to deserialize them to some response type, so it takes advantage of Rust type elision to
provide us with a nice Api.

Now it's time to rerun the test. It should still pass, but now we nicely abstracted the testing contract as a whole,
not some internal functions. The next thing we should probably cover is making the contract more interesting
by adding some state.
