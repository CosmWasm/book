# Working with time

The concept of time in the blockchain is tricky - as in
every distributed system, it is not easy to synchronize the
clocks of all the nodes.

However, there is the notion of a time that is even
monotonic - which means that it should never go "backward"
between executions. Also, what is important is - time is
always unique throughout the whole transaction - and even
the entire block, which is built of multiple transactions.

The time is encoded in the
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
mentioning - it contains a sequence number of the processed
block. It is sometimes more useful than time, as it is
guaranteed that the `height` field is guaranteed to increase
between blocks, while two blocks may be executed with the
same `time` (even though it is rather not probable).

Also, many transactions might be executed in a single block.
That means that if we need a unique id for the execution of
a particular message, we should look for something more.
This thing is a
[`transaction`](https://docs.rs/cosmwasm-std/1.2.4/cosmwasm_std/struct.TransactionInfo.html)
field of the `Env` type:

```rust
pub struct TransactionInfo {
    pub index: u32,
}
```

The `index` here contains a unique index of the transaction
in the block. That means that to get the unique identifier
of a transaction through the whole block, we can use the
`(height, transaction_index)` pair.

## Join time

We want to use the time in our system to keep track of the
join time of admins. We don't yet add new members to the
group, but we can already set the join time of initial
admins. Let's start updating our state:

```rust
use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::Map;
# use cw_storage_plus::Item;

pub const ADMINS: Map<&Addr, Timestamp> = Map::new("admins");
# pub const DONATION_DENOM: Item<String> = Item::new("donation_denom");
```

As you can see, our admins set became a proper map - we will
assign the join time to every admin.

Now we need to update how we initialize a map - we stored the Empty data previously, but it nevermore matches our value type. Let's check an updated instantiation function:

You might argue to create a separate structure for the value
of this map, so in the future, if we would need to add
something there, but in my opinion, it would be premature -
we can also change the entire value type in the future, as
it would be the same breaking change.

Now we need to update how we initialize a map - we stored
the `Empty` data previously, but it nevermore matches our
value type. Let's check an updated instantiation function:

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
note that I removed the underscore from the name of the
`env` block - it was there only to ensure the Rust compiler
the variable is purposely unused and not some kind of a bug.

Finally, remember to remove any obsolete `Empty` imports
through the project - the compiler should help you point out
unused imports.

## Query and tests

The last thing to add regarding join time is the new query
asking for the join time of a particular admin. Everything
you need to do that was already discussed, I'll leave it for
you as an exercise. The query variant should look like:

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

You may question that in response type, I suggest always returning a `joined`
value, but what to do when no such admin is added? Well, in such a case, I
would rely on the fact that `load` function returns a descriptive error of
missing value in storage - however, feel free to define your own error for such
a case or even make the `joined` field optional, and be returned if requested
admin exists.

Finally, there would be a good idea to make a test for new functionality - call
a new query right after instantiation to verify initial admins has proper join
time (possibly by extending the existing instantiation test).

One thing you might need help with in tests might be how to get the time of
execution. Using any OS-time would be doomed to fail - instead, you can call
the
[`block_info`](https://docs.rs/cw-multi-test/0.16.4/cw_multi_test/struct.App.html#method.block_infohttps://docs.rs/cw-multi-test/0.16.4/cw_multi_test/struct.App.html#method.block_info)
function to reach the
[`BlockInfo`](https://docs.rs/cosmwasm-std/latest/cosmwasm_std/struct.BlockInfo.html)
structure containing the block state at a particular moment in the app - calling
it just before instantiation would make you sure you are working with the same state
which would be simulated on the call.
