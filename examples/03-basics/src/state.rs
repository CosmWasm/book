use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");
