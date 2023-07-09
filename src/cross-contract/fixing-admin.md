# Fixing admin contract

Now that we know what we want to achieve, we can start by converting the
contract we already have into an admin contract. It is mostly
fine at this point, but we want to clean it up.

## Cleaning up queries

The first thing to do is to get rid of the `Greet` query - it was good as a
starter query example, but it has no practical purpose and only generates noise.

Let's remove the unnecessary variant from the query enum:

```rust
# use cosmwasm_schema::{cw_serde, QueryResponses};
# use cosmwasm_std::Addr;
# 
# #[cw_serde]
# pub struct InstantiateMsg {
#     pub admins: Vec<String>,
#     pub donation_denom: String,
# }
# 
# #[cw_serde]
# pub enum ExecuteMsg {
#     Leave {},
#     Donate {},
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
    #[returns(AdminsListResp)]
    AdminsList {},
}
```

We should also remove the invalid path in the query dispatcher:

```rust
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        AdminsList {} => to_binary(&query::admins_list(deps)?),
    }
}
```

Finally, we remove the unnecessary handler from the `contract::query` module.
We also need to make sure all references to it are gone (e.g. any references
in the tests if they exist).

## Generating the library output

At the very beginning of the book, we set the `crate-type` in `Cargo.toml` to
`"cdylib"`. This was required to generate the wasm output, but it comes with a
drawback - dynamic libraries cannot be used as dependencies in
other crates. It wasn;t a problem before, but in practice we often want to
enable contracts that depend on others to have access to some of their types - for example,
defined messages.

Lucky for us, it's easy to fix. You may have noticed that the `crate-type` is an array,
not a single string. The reason for that is that our project can emit several
targets - in particular, we can add the default `"rlib"` crate type to it. This 
makes it generate a "rust library" output which is what we need to be able to use it as a
dependency. Let's update our `Cargo.toml`:

```toml
[package]
name = "admin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]
# 
# [features]
# library = []
# 
# [dependencies]
# cosmwasm-std = { version = "1.1.4", features = ["staking"] }
# serde = { version = "1.0.103", default-features = false, features = ["derive"] }
# cw-storage-plus = "0.15.1"
# thiserror = "1"
# schemars = "0.8.1"
# cw-utils = "0.15.1"
# cosmwasm-schema = "1.1.4"
# 
# [dev-dependencies]
# cw-multi-test = "0.15.1"
```

Also note the change of contract name - "contract" is not very descriptive, so
we've updated it to "admin".

## Project structure

Last but not least,  we want to improve the structure of our project. Up to now we only had
one contract to deal with, which is why we just worked on it as the whole project. Now we want to create a
directory tree that reflects relations between the contracts we create.

First, let's create a directory for the project. Then we'll want to create a "contracts"
subdirectory inside it. This is not technically required from Rust's point-of-view, but there
are tools in our environment such as the workspace optimizer which will look for contracts there. This is a common pattern you will see in CosmWasm contracts repos.

Next we copy the whole project directory from the previous chapter into the
`contracts` folder, renaming it to `admin`.

Finally, we want to group all our projects (for now, we have just one, but we know
there will be more). To do so, we create a workspace-level `Cargo.toml`
file in the top-level project directory:

```toml
[workspace]
members = ["contracts/*"]
resolver = "2"
```

This `Cargo.toml` differs slightly from the typical project-level one - it
defines a workspace. The most important field here is `members` - it
defines projects as being part of the workspace.

The other field is the `resolver`. This is something it is important to remember to add - it
instructs Cargo to use version 2 of the dependency resolver. This has been the
default for non-workspaces since Rust 2021, but for compatibility reasons
the default couldn't be changed for workspaces.  It is generally recommended to add it to
every newly created workspace.

The last field which might be useful for some workspaces is exclude - it allows us to
create projects in the workspace directory tree which are not a part of this
workspace.  We won't use it, but it can be useful to know about it.

Now just for clarity, let's see the top-level directory structure:

```none
.
├── Cargo.lock
├── Cargo.toml
├── contracts
│  └── admin
└── target
   ├── CACHEDIR.TAG
   └── debug
```

You can see the target directory and `Cargo.lock` files exist in the tree - this is
because we already built and ran tests for the `admin` contract. In Rust workspaces,
Cargo knows to build everything at the top level even if it was initially built from
an inner directory.
