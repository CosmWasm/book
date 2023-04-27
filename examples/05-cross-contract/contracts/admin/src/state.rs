use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};

pub const ADMINS: Map<&Addr, Timestamp> = Map::new("admins");
pub const DONATION_DENOM: Item<String> = Item::new("donation_denom");
