# Map storage

There is one thing to be immediately improved in the admin contract. Let's
check the contract state:

```rust
# use cosmwasm_std::Addr;
# use cw_storage_plus::Item;
# 
pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");
pub const DONATION_DENOM: Item<String> = Item::new("donation_denom");
```

Note that we keep our admin list as a single vector. However, in the whole
contract, in most cases, we access only a single element of this vector.

This is not ideal, as now, whenever we want to access the single admin entry,
we have first to deserialize the list containing all of them and then iterate
over them until we find the interesting one. This might consume a serious
amount of gas and is completely unnecessary overhead - we can avoid that using
the [Map](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html)
storage accessor.

## The `Map` storage

First, let's define a map - in this context, it would be a set of keys with values
assigned to them, just like a `HashMap` in Rust or dictionaries in many languages.
We define it as similar to an `Item`, but this time we need two types - the key type
and the value type:

```rust
use cw_storage_plus::Map;

pub const STR_TO_INT_MAP: Map<String, u64> = Map::new("str_to_int_map");
```

Then to store some items on the [`Map`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html),
we use a
[`save`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html#method.save)
method - same as for an `Item`:

```rust
STR_TO_INT_MAP.save(deps.storage, "ten".to_owned(), 10);
STR_TO_INT_MAP.save(deps.storage, "one".to_owned(), 1);
```

Accessing entries in the map is also as easy as reading an item:

```rust
let ten = STR_TO_INT_MAP.load(deps.storage, "ten".to_owned())?;
assert_eq!(ten, 10);

let two = STR_TO_INT_MAP.may_load(deps.storage, "two".to_owned())?;
assert_eq!(two, None);
```

Obviously, if the element is missing in the map, the
[`load`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html#method.load)
function will result in an error - just like for an item. On the other hand -
[`may_load`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html#method.may_load)
returns a `Some` variant when element exits.

Another very useful accessor that is specific to the map is the
[`has`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html#method.has)
function, which checks for the existence of the key in the map:

```rust
let contains = STR_TO_INT_MAP.has(deps.storage, "three".to_owned())?;
assert!(!contains);
```

Finally, we can iterate over elements of the maps - either its keys or key-value
pairs:

```rust
use cosmwasm_std::Order;

for k in STR_TO_INT_MAP.keys(deps.storage, None, None, Order::Ascending) {
    let _addr = deps.api.addr_validate(k?);
}

for item in STR_TO_INT_MAP.range(deps.storage, None, None, Order::Ascending) {
    let (_key, _value) = item?;
}
```

First, you might wonder about extra values passed to
[`keys`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html#method.keys)
and
[`range`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html#method.range) -
those are in order: lower and higher bounds of iterated elements, and the order
elements should be traversed.

While working with typical Rust iterators, you would probably first create an
iterator over all the elements and then somehow skip those you are not
interested in. After that, you will stop after the last interesting element.

It would more often than not require accessing elements you filter out, and
this is the problem - it requires reading the element from the storage. And
reading it from the storage is the expensive part of working with data, which
we try to avoid as much as possible. One way to do it is to instruct the Map
where to start and stop deserializing elements from storage so it never reaches
those outside the range.

Another critical thing to notice is that the iterator returned by both keys and
range functions are not iterators over elements - they are iterators over `Result`s.
It is a thing because, as it is rare, it might be that item is supposed to exist,
but there is some error while reading from storage - maybe the stored value is
serialized in a way we didn't expect, and deserialization fails. This is actually
a real thing that happened in one of the contracts I worked on in the past - we
changed the value type of the Map, and then forgot to migrate it, which caused
all sorts of problems.

## Maps as sets

So I imagine you can call me crazy right now - why do I spam about a `Map`, while
we are working with vector? It is clear that those two represent two distinct
things! Or do they?

Let's reconsider what we keep in the `ADMINS` vector - we have a list of objects
which we expect to be unique, which is a definition of a mathematical set. So
now let me bring back my initial definition of the map:

> First, let's define a map - in this context, it would be a *set* of keys with
> values assigned to them, just like a HashMap in Rust or dictionaries in many languages.

I purposely used the word "set" here - the map has the set built into it. It is
a generalization of a set or reversing the logic - the set is a particular case
of a map. If you imagine a set that map every single key to the same value, then
the values become irrelevant, and such a map becomes a set semantically.

How can you make a map mapping all the keys to the same value? We pick a type
with a single value. Typically in Rust, it would be a unit type (`()`), but in
CosmWasm, we tend to use the
[`Empty`](https://docs.rs/cosmwasm-std/1.2.4/cosmwasm_std/struct.Empty.html)
type from CW standard crate:

```rust
use cosmwasm_std::{Addr, Empty};
use cw_storage_plus::Map;

pub const ADMINS: Map<Addr, Empty> = Map::new("admins");
```

We now need to fix the usage of the map in our contract. Let's start with contract
instantiation:

```rust
use crate::msg::InstantiateMsg;
use crate::state::{ADMINS, DONATION_DENOM};
use cosmwasm_std::{
    DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    for addr in msg.admins {
        let admin = deps.api.addr_validate(&addr)?;
        ADMINS.save(deps.storage, admin, &Empty {})?;
    }
    DONATION_DENOM.save(deps.storage, &msg.donation_denom)?;

    Ok(Response::new())
}
```

It didn't simplify much, but we no longer need to collect our address. Then
let's move to the leaving logic:

```rust
use crate::state::ADMINS;
use cosmwasm_std::{DepsMut, MessageInfo};

pub fn leave(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    ADMINS.remove(deps.storage, info.sender.clone());

    let resp = Response::new()
        .add_attribute("action", "leave")
        .add_attribute("sender", info.sender.as_str());

    Ok(resp)
}
```

Here we see a difference - we don't need to load a whole vector. We remove a
single entry with the
[`remove`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html#method.remove)
function.

What I didn't emphasize before, and what is relevant, is that `Map` stores every
single key as a distinct item. This way, accessing a single element will be
cheaper than using a vector.

However, this has its downside - accessing all the elements is more
gas-consuming using Map! In general, we tend to avoid such situations - the
linear complexity of the contract might lead to very expensive executions
(gas-wise) and potential vulnerabilities - if the user finds a way to create
many dummy elements in such a vector, he may make the execution cost exceeding
any gas limit.

Unfortunately, we have such an iteration in our contract - the distribution flow
becomes as follows:

```rust
use crate::error::ContractError;
use crate::state::{ADMINS, DONATION_DENOM};
use cosmwasm_std::{
    coins, BankMsg,DepsMut, MessageInfo, Order, Response
};

pub fn donate(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let denom = DONATION_DENOM.load(deps.storage)?;
    let admins: Result<Vec<_>, _> = ADMINS
        .keys(deps.storage, None, None, Order::Ascending)
        .collect();
    let admins = admins?;

    let donation = cw_utils::must_pay(&info, &denom)?.u128();

    let donation_per_admin = donation / (admins.len() as u128);

    let messages = admins.into_iter().map(|admin| BankMsg::Send {
        to_address: admin.to_string(),
        amount: coins(donation_per_admin, &denom),
    });

    let resp = Response::new()
        .add_messages(messages)
        .add_attribute("action", "donate")
        .add_attribute("amount", donation.to_string())
        .add_attribute("per_admin", donation_per_admin.to_string());

    Ok(resp)
}
```

If I had to write a contract like this, and this `donate` would be a critical,
often called flow, I would advocate for going for an `Item<Vec<Addr>>` here.
Hopefully, it is not the case - the distribution does not have to be linear in
complexity! It might sound a bit crazy, as we have to iterate over all receivers
to distribute funds, but this is not true - there is a pretty nice way to do so
in constant time, which I will describe later in the book. For now, we will
leave it as it is, acknowledging the flaw of the contract, which we will fix later.

The final function to fix is the `admins_list` query handler:

```rust
use crate::state::ADMINS;
use cosmwasm_std::{Deps, Order, StdResult};

pub fn admins_list(deps: Deps) -> StdResult<AdminsListResp> {
    let admins: Result<Vec<_>, _> = ADMINS
        .keys(deps.storage, None, None, Order::Ascending)
        .collect();
    let admins = admins?;
    let resp = AdminsListResp { admins };
    Ok(resp)
}
```

Here we also have an issue with linear complexity, but it is far less of a problem.

First, queries are often purposed to be called on local nodes, with no gas cost -
we can query contracts as much as we want.

And then, even if we have some limit on execution time/cost, there is no reason to
query all the items every single time! We will fix this function later, adding
pagination - to limit the execution time/cost of the query caller would be able to
ask for a limited amount of items starting from the given one. Knowing this chapter,
you can probably figure implementation of it right now, but I will show the common
way we do that when I go through common CosmWasm practices.

## Reference keys

There is one subtlety to improve in our map usage.

The thing is that right now, we index the map with the owned Addr key. That forces
us to clone it if we want to reuse the key (particularly in the leave implementation).
This is not a huge cost, but we can avoid it - we can define the key of the map
to be a reference:

```rust
use cosmwasm_std::{Addr, Empty};
use cw_storage_plus::Map;

pub const ADMINS: Map<&Addr, Empty> = Map::new("admins");
pub const DONATION_DENOM: Item<String> = Item::new("donation_denom");
```

Finally, we need to fix the usages of the map in two places:

```rust
# use crate::state::{ADMINS, DONATION_DENOM};
# use cosmwasm_std::{
#     DepsMut, Empty, Env, MessageInfo, Response, StdResult,
# };
#
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    for addr in msg.admins {
        let admin = deps.api.addr_validate(&addr)?;
        ADMINS.save(deps.storage, &admin, &Empty {})?;
    }

    // ...

#    DONATION_DENOM.save(deps.storage, &msg.donation_denom)?;
#
   Ok(Response::new())
}

pub fn leave(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    ADMINS.remove(deps.storage, &info.sender);

    // ...

#    let resp = Response::new()
#        .add_attribute("action", "leave")
#        .add_attribute("sender", info.sender.as_str());
#
   Ok(resp)
}
```
