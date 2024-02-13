# 6. Interacting with Contracts

## Effective Contract Interaction

This section is dedicated to understanding how to interact with smart contracts in CosmWasm effectively. We'll cover managing contract state, executing operations, and the nuances of data interaction.

## Managing Contract State

Efficient state management is crucial for the smooth operation of smart contracts. This part discusses state management in CosmWasm contracts.

### State Management Techniques

- **State Storage Strategies**: Best practices for storing and retrieving state within your contracts.
- **State Migration**: How to handle state migration during contract upgrades.

```rust
#[cw_serde]
struct Config {
    owner: Addr,
    amount: Uint128,
    // using a struct for more complex state allows you to add more fields later on without breaking the contract
}

const CONFIG: Item<Config> = Item::new("config");

/// Maps an address to a balance
/// You can use this to store a balance per address and load it later
const BALANCE: Map<Addr, Uint128> = Map::new("balance");
```

### Sending and Querying Data

Interacting with contracts involves sending transactions and querying data. This section provides detailed guidance on these operations.

### Sending Transactions

- Executing Contract Functions: How to send transactions to execute contract functions.
- Handling Responses: Understanding the responses from contract execution.

```rust
// Example Rust code for sending a transaction
#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    // To send sub-messages to other contracts or the chain, you add them to the Response that you return
    Ok(Response::new().add_message(WasmMsg::Execute {
        contract_addr: msg.contract_addr,
        // OtherContractMsg is the message type that the other contract will understand.
        // Many open source contracts will provide their types as a crate you can use.
        msg: to_json_binary(&OtherContractMsg::Send {
            amount: Uint128::new(100),
        })?,
        funds: vec![],
    }))
}
```

### Querying Contract Data

- Query Mechanics: How to query data from a contract.
- Response Formats: Understanding the different formats of query responses.
- Off-Chain Interactions: Handling interactions with off-chain data sources.

```rust
// Example Rust code for querying contract data
#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    // OtherContractQuery and OtherContractExchangeRateResp are the types the other contract expects / returns
    // Many open source contracts will provide their types as a crate you can use.
    let response: OtherContractExchangeRateResp = deps
        .querier
        .query_wasm_smart(msg.contract_addr, &OtherContractQuery::ExchangeRate {})?;

    // now you can use the response data for your contract logic
    let amount = response.rate * Uint128::new(100);
    // ...

    Ok(Response::default())
}
```

You can also query data from the blockchain using the `deps.querier` object:

```rust
// Example Rust code for querying chain data
#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Response, ContractError> {
    // Query the balance of the sender for the given denomination
    let coin = deps.querier.query_balance(info.sender, msg.denom)?;
    // now you can use the response data for your contract logic
    to_json_binary(&coin.amount)
}
```
