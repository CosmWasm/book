# Map storage

There is one thing we can immediately improve in the admin contract. Let's
check the contract state:

```rust
# use cosmwasm_std::Addr;
# use cw_storage_plus::Item;
# 
pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");
pub const DONATION_DENOM: Item<String> = Item::new("donation_denom");
```

Note that we keep our admin list as a single vector. However, in most cases we only need to access a single element of this vector.  This is not ideal since it means that whenever we want to access a single admin entry,
we first have to deserialize the entire list of them and then iterate
over them until we find the one we are interested in. This may consume a serious
amount of gas and is completely unnecessary overhead that we can avoid by using
the [Map](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html)
storage accessor.

## The `Map` storage accessor

First, let's define a map - in this context, it will be a set of keys with values
assigned to them, just like a `HashMap` in Rust or dictionaries in many languages.
We define it in a similar way to how we define an `Item`, but this time we need two types, a key type
and a value type:

```rust
use cw_storage_plus::Map;

pub const STR_TO_INT_MAP: Map<String, u64> = Map::new("str_to_int_map");
```

To store some items on the [`Map`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html),
we use the 
[`save`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html#method.save)
method just as we did for an `Item`:

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

Obviously, if the element is missing from the map, the
[`load`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html#method.load)
function will result in an error - just as for an item. Alternatively we can use 
[`may_load`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html#method.may_load)
which returns a `Some` variant when an element exits.

Another very useful accessor that is specific to the map is the
[`has`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html#method.has)
function, which checks for the existence of a key in the map:

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

You may be wondering about those extra values we passed to
[`keys`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html#method.keys)
and
[`range`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html#method.range) -
in order, these are: lower and higher bounds of iterated elements, and the order
in which elements should be traversed.

When working with typical Rust iterators, you would probably first create an
iterator over all the elements and then somehow skip those you are not
interested in. After that, you stop after the last interesting element.

This would more often than not require accessing elements you filter out, and
this is the problem - it requires reading that element from the storage. Reading from the storage is the expensive part of working with data, which
we try to avoid as much as possible. One way to do that is to instruct the Map
where exactly to start and stop deserializing elements from storage so it never reaches
those outside the range.

Another critical thing to notice is that the iterators returned by both keys and
range functions are not actually iterators over elements - they are iterators over `Result`s.
Although it is rare, it's possible that an item is supposed to exist 
but there is some error while reading from storage - maybe the stored value is
serialized in a way we didn't expect and deserialization fails. This is actually
a real thing that happened in one of the contracts the authors worked on in the past - we
changed the value type of the Map, and then forgot to migrate it, which caused
all sorts of problems.

## Maps as sets

So you may well be thinking we're crazy right now - why are we going on about `Map` so much when
we are actually working with a vector? They clearly represent two distinct
things! Or do they?

Let's reconsider what we keep in the `ADMINS` vector - we have a list of objects
that we expect to be unique, which is the definition of a mathematical set. So
now let us bring back our initial definition of a map:

> First, let's define a map - in this context, it will be a *set* of keys with values
> assigned to them, just like a `HashMap` in Rust or dictionaries in many languages.


We purposely used the word "set" here - a map has a set built into it. It is
a generalization of a set, or reversing the logic, a set is a particular case
of a map. If you imagine a set that maps every single key to the same value, then
the values become irrelevant, and such a map becomes a set semantically.

How can we make a map that maps all the keys to the same value? We pick a type
with a single value. Typically in Rust, this would be a unit type (`()`), but in
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

It didn't simplify much, but we no longer need to collect our addresses. Then
let's move on to the leaving logic:

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

Here we notice the difference - we don't need to load a whole vector. We remove a
single entry with the
[`remove`](https://docs.rs/cw-storage-plus/1.0.1/cw_storage_plus/struct.Map.html#method.remove)
function.

Something relevant that we didn't point out before is that `Map` stores every
single key as a distinct item. This way, accessing a single element will be
cheaper than using a vector.

However, this has its downside as well - accessing all the elements consumes more
gas using Map! In general, we tend to try to avoid such situations - the
linear complexity of the contract might lead to very expensive executions
(gas-wise) and potential vulnerabilities.  If the user finds a way to create
many dummy elements in such a vector they can make the execution cost exceed
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

If writing a contract like this in which this `donate` is a critical common part of the
flow, we would advise going for an `Item<Vec<Addr>>` here.
Thankfully, this is not the case - the distribution does not have to be linear in
complexity! It might sound a bit unbelievable, since we have to iterate over all receivers
to distribute funds, but there is a pretty nice way to do so
in constant time which I will describe later in the book. For now, we will
leave it as it is, just acknowledging this flaw in the contract that we will fix later.

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

First of all, queries are often purposed to be called on local nodes, with no gas cost.  This means that
we can query contracts as much as we want.

Even if we do have some limit on execution time/cost, there is no reason to
query all the items every single time! We will fix this function later, adding
pagination - to limit the execution time/cost of the query caller by being able to
ask for a limited amount of items starting from a given one. Having gone through this chapter,
you can probably figure implementation of it right now, but I will show the common
way we do it when we get to looking at common CosmWasm practices.

## Reference keys

There is another subtlety to improve in our map usage.

The thing is that right now, we index the map with the owned Addr key. This forces
us to clone it if we want to reuse the key (particularly in the leave implementation).
This is not a huge cost, but we can avoid it - we can define the key of the map
to be a reference:

```rust
use cosmwasm_std::{Addr, Empty};
use cw_storage_plus::Map;

pub const ADMINS: Map<&Addr, Empty> = Map::new("admins");
pub const DONATION_DENOM: Item<String> = Item::new("donation_denom");
```

Finally, we need to fix the usage of the map in two places:

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
