use cosmwasm_std::{Addr, Empty};
use cw_storage_plus::{Item, Map};

pub const ADMINS: Map<Addr, Empty> = Map::new("admins");
pub const DONATION_DENOM: Item<String> = Item::new("donation_denom");
