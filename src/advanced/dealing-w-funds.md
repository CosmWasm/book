# Dealing with Funds 

In the blockchain realm, particularly within the CosmWasm ecosystem, the handling of native tokens and the execution of financial transactions are fundamental capabilities of smart contracts. CosmWasm 2.0 enhances these capabilities, providing developers with a refined toolkit for managing cryptocurrencies—often referred to as tokens—directly within smart contracts. This section of the book explores the intricacies of dealing with funds in CosmWasm 2.0, focusing on a practical example: a donation system that rewards contract administrators.

## Native Tokens and Smart Contracts
Native tokens are digital assets managed by the blockchain's core rather than by individual smart contracts. These tokens can have various roles, from facilitating transaction fees (gas) to acting as stakes in consensus mechanisms. In CosmWasm, smart contracts can own, send, and receive native tokens, enabling a wide range of financial operations.

## Implementing a Donation System
To demonstrate the handling of funds in CosmWasm 2.0, let's implement a Donate function within a smart contract. This function allows users to donate tokens to the contract, which are then equally distributed among the contract's administrators.

- **Step 1:** Defining Messages
First, we need to define the necessary messages for our contract. This includes a new variant in the ExecuteMsg enum for the Donate action and an adjustment to the InstantiateMsg to specify the token denomination for donations.

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub donation_denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    AddMembers { admins: Vec<String> },
    Leave {},
    Donate {},
}
```

- **Step 2:** Managing State
We introduce a new state variable to store the donation denomination, utilizing the cw-storage-plus package for efficient state management.

```rust
use cw_storage_plus::Item;

pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");
pub const DONATION_DENOM: Item<String> = Item::new("donation_denom");
```

- **Step 3:** Handling Donations

The Donate execution logic involves accepting donations and equally distributing them among the administrators. This process requires validating the donated funds, calculating the share per administrator, and performing token transfers.

```rust
use cosmwasm_std::{coins, BankMsg, DepsMut, MessageInfo, Response, StdResult, Addr, Coin};

fn donate(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let denom = DONATION_DENOM.load(deps.storage)?;
    let admins = ADMINS.load(deps.storage)?;

    let total_donation: Coin = info
        .funds
        .iter()
        .find(|&coin| coin.denom == denom)
        .ok_or_else(|| StdError::generic_err("No donations in the specified denom"))?
        .clone();

    let donation_per_admin = Coin {
        denom: denom.clone(),
        amount: total_donation.amount.u128() / (admins.len() as u128),
    };

    let messages: Vec<_> = admins.iter().map(|admin| BankMsg::Send {
        to_address: admin.to_string(),
        amount: vec![donation_per_admin.clone()],
    }).collect();

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "donate")
        .add_attribute("donation_denom", &denom)
        .add_attribute("total_donation", total_donation.amount)
        .add_attribute("donation_per_admin", donation_per_admin.amount))
}
```
- **Step 4:** Testing the Donation Logic
Testing ensures that the donation logic works as expected, verifying the equal distribution of funds among administrators.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_donation_distribution() {
        let mut deps = mock_dependencies(&coins(100, "token"));

        let instantiate_msg = InstantiateMsg {
            admins: vec!["admin1".to_string(), "admin2".to_string()],
            donation_denom: "token".to_string(),
        };

        let info = mock_info("creator", &coins(100, "token"));
        instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();

        let donate_info = mock_info("donor", &coins(50, "token"));
        let donate_msg = ExecuteMsg::Donate {};
        let _ = execute(deps.as_mut(), mock_env(), donate_info, donate_msg).unwrap();

        // Assertions to verify the correct distribution of donations
    }
}
```
## Conclusion

This practical example illustrates the process of managing funds within a CosmWasm 2.0 smart contract, from defining messages and managing state to executing financial transactions and testing. By leveraging the framework's powerful features, developers can implement sophisticated financial mechanisms, such as the donation system described, enhancing the functionality and interactivity of their smart contracts in the Cosmos ecosystem.