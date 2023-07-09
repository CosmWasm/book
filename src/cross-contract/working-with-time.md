# Working with time

The concept of time in the blockchain is tricky - as in
every distributed system, it is not easy to synchronize the
clocks of all of the nodes.

There is a notion of a time that it is should be
monotonic, meaning that it should never go "backwards"
between executions. Also, it is important that time is
always unique throughout not just the whole transaction, but also
the entire block, which of course can be built up from multiple transactions.

Time is encoded in the
[`Env`](https://docs.rs/cosmwasm-std/1.2.4/cosmwasm_std/struct.Env.html)
type in its
[`block`](https://docs.rs/cosmwasm-std/1.2.4/cosmwasm_std/struct.BlockInfo.html)
field, which looks like this:

```rust
pub struct BlockInfo {
    pub height: u64,
    pub time: Timestamp,
    pub chain_id: String,
}
```

You can see the `time` field, which is the timestamp of the
processed block. The `height` field is also worth
mentioning as it contains a sequence number of the processed
block. It is sometimes more useful than time, as the `height` field is guaranteed to increase
between blocks, while two blocks may be executed with the
same `time` (even though this is rather improbable).

Since many transactions may be executed in a single block, if we need a unique id for the execution of
a particular message then we shall need something else.
What we can use is the
[`transaction`](https://docs.rs/cosmwasm-std/1.2.4/cosmwasm_std/struct.TransactionInfo.html)
field of the `Env` type:

```rust
pub struct TransactionInfo {
    pub index: u32,
}
```

The `index` here contains a unique index of the transaction
in the block. That means that to get a unique identifier
of a transaction within the entire blockchain, we can use the
`(height, transaction_index)` pair.

## Join time

We want to use the time in our system to keep track of the
join time of admins. We won't yet solve this for adding new members to the
group, but we can already set the join time of initial
admins. Let's start updating our state:

```rust
use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::Map;
# use cw_storage_plus::Item;

pub const ADMINS: Map<&Addr, Timestamp> = Map::new("admins");
# pub const DONATION_DENOM: Item<String> = Item::new("donation_denom");
```

As you can see, our admins set has became a proper map - we will
assign the join time to every admin.

Now we need to update how we initialize a map - we stored the `Empty` data previously, but it no longer matches our value type. Let's consider an updated instantiation function:

You might argue for creating a separate structure for the value
of this map, so that in future we can add things if necessary.  In the author's opinion, this would be premature as
it would be the same breaking change, and in any case we can still change the entire value type in the futur if we need to.

Let's take a look at an updated instantiation function:

```rust
use crate::state::{ADMINS, DONATION_DENOM};
use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Response, StdResult,
};

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    for addr in msg.admins {
        let admin = deps.api.addr_validate(&addr)?;
        ADMINS.save(deps.storage, &admin, &env.block.time)?;
    }
    DONATION_DENOM.save(deps.storage, &msg.donation_denom)?;

    Ok(Response::new())
}
```

Instead of storing `&Empty {}` as an admin value, we store
the join time, which we read from `&env.block.time`. Also,
note that we have removed the underscore from the name of the
`env` block (it was there only to assure the Rust compiler that
the variable was deliberately unused and not some kind of bug).

Finally, remember to remove any obsolete `Empty` imports
through the project - the compiler should help you point out
unused imports.

## Query and tests

The last thing to add regarding join time is the new query
asking for the join time of a particular admin. Everything
you need to do that was already discussed, so it is left as an exercise. The query variant should look like this:

```rust
#[returns(JoinTimeResp)]
JoinTime { admin: String },
```

And the example response type:

```rust
#[cw_serde]
pub struct JoinTimeResp {
    pub joined: Timestamp,
}
```

You may question the fact that the suggested response type always returns a `joined`
value, while there is always the possibility that no such admin exists in the group. Well, in such a case, we
rely on the fact that the `load` function returns a descriptive error of
`missing value in storage`.  However, feel free to define your own error for such
a case or even to make the `joined` field optional, to be returned only if the requested
admin exists.

Finally, it would be a good idea to make a test for this new functionality - call
a new query right after instantiation to verify that the initial admins all have a proper join
time (possibly by extending the existing instantiation test).

One thing you might need help with in the tests is how exactly to get the time of
execution. Using any OS-time will be doomed to fail - instead, you can call
the
[`block_info`](https://docs.rs/cw-multi-test/0.16.4/cw_multi_test/struct.App.html#method.block_infohttps://docs.rs/cw-multi-test/0.16.4/cw_multi_test/struct.App.html#method.block_info)
function to reach the
[`BlockInfo`](https://docs.rs/cosmwasm-std/latest/cosmwasm_std/struct.BlockInfo.html)
structure containing the block state at a particular moment in the app - calling
it just before instantiation ensures that you are working with the same state
which would be simulated on the call.
