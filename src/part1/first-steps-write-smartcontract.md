# 3. First Steps: Writing a Simple Contract

## Building Your First Contract

### Tutorial on Basic Contract Structure

CosmWasm smart contracts are written in Rust and follow a specific structure. Key components include:

- **Instantiate Function**: Initializes the contract state.
- **Execute Function**: Contains the logic that alters the contract state.
- **Query Function**: Used for reading contract state without making changes.

```rust
// Example of a simple CosmWasm contract structure
use cosmwasm_std::{
    ensure_eq, entry_point, to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult,
};
use cw_storage_plus::Item;

const OWNER: Item<Addr> = Item::new("owner");

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    // Set initial state (owner in this case)
    // In most cases, you will also want to save some data from the InstantiateMsg
    OWNER.save(deps.storage, &info.sender)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetOwner { owner } => {
            // validate that the sender is the current owner
            let current_owner = OWNER.load(deps.storage)?;
            ensure_eq!(info.sender, current_owner, ContractError::Unauthorized {});
            // validate the new owner address
            let owner = deps.api.addr_validate(&owner)?;
            // Set the owner
            OWNER.save(deps.storage, &owner)?;

            Ok(Response::default())
        }
        // ... other execute message handlers
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner {} => to_json_binary(&OWNER.load(deps.storage)?),
        // ... other query message handlers
    }
}
```

## Tips for Writing Clean and Efficient Code

Follow Rust Best Practices: Utilize Rust's features like ownership, types, and error handling to write safe and efficient code.

- Keep it Simple: Start with a simple logic and gradually add complexity.
- Testing: Regularly test your code to catch and fix errors early.

## Deploying and Testing the Contract

### Compilation and Deployment

Compile your contract to WebAssembly (WASM) and deploy it either locally or on a testnet.

# Compile the contract to

```
cargo wasm

# Deploy using CosmWasm tooling (specific commands vary based on the deployment target)
```

# Introduction to Testing Frameworks

CosmWasm provides testing frameworks that allow you to write unit tests for your contracts.

```rust
// Example of a simple unit test in CosmWasm
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{from_json, testing::*};

    #[test]
    fn instantiate_sets_owner() {
        let mut deps = mock_dependencies();
        // create a valid address from an arbitrary string
        let owner = deps.api.addr_make("owner");
        let info = mock_info(owner.as_str(), &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, InstantiateMsg {}).unwrap();
        assert_eq!(0, res.messages.len());

        // query the owner
        let queried_owner = query(deps.as_ref(), mock_env(), QueryMsg::Owner {}).unwrap();
        let queried_owner: Addr = from_json(&queried_owner).unwrap();

        // ensure it was properly stored
        assert_eq!(owner, queried_owner);
    }
}
```

# Multi-test examples
