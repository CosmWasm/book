# Smart contract as an actor

In previous chapters we talked about the Actor Model and how it is implemented
on the blockchain. Now it is time to take a closer look at the typical contract
structure to understand how different features of the Actor Model map to
it.

This will not be a step-by-step guide on contract creation, as that is a topic
for a series in itself. We shall instead go through various contract elements roughly to
visualize how to handle architecture in the Actor Model.

## The state

As before we will start with the state. Previously we worked with
the `cw4-group` contract, so let's start by looking at its code. Go to
`cw-plus/contracts/cw4-group/src`. The folder structure should look like
this:

```bash
  src
├──  contract.rs
├──  error.rs
├──  helpers.rs
├──  lib.rs
├──  msg.rs
└──  state.rs
```

As you may already have figured out, we want to check `state.rs` first.

The most important thing to note here are is the use of a few constants: `ADMIN`, `HOOKS`,
`TOTAL`, and `MEMBERS`. Every one of these constants represents a single portion
of the contract state - just as tables in databases. The types of those constants
represent what kind of table it is. The most basic ones are of type `Item<T>`, which
keeps zero or one element of a given type, and `Map<K, T>` which is a key-value
map.

You can see that `Item` is used to keep an admin and some other data: `HOOKS`, and
`TOTAL`. `HOOKS` is used by the `cw4-group` to allow subscription to any
changes to a group.  Thus a contract can be added as a hook, so that a message is sent to it whenever a group changes. The `TOTAL` is just a sum of all members'
weights.

The `MEMBERS` in the group contract is of type `SnapshotMap` - as you can imagine,
it is a `Map` - but on steroids.  This particular one gives us access to the
state of the map at any point in history, accessed by the blockchain
`height`. `height` is the count of blocks created since the beginning of
the blockchain itself, and it is the most atomic time representation available to smart contracts.
There is also a way to access the clock time in them, but everything that happens in a
single block is considered to have happened at the same moment.

Other types of storage objects that are not used in group contracts are:

* `IndexedMap` - another map type, that allows accessing values
  by a variety of keys
* `IndexedSnapshotMap` - `IndexedMap` and `SnapshotMap` married together

It is very important to note that every state type in the contract is accessed using
some name. None of these types are containers, just accessors to the state.
Do you remember that we stated that the blockchain is our database? It's exactly so!
All these types are just [ORM](https://en.wikipedia.org/wiki/ORM) for this database - we use them by passing a special `State` object to them, enabling them to retrieve data items from the blockchain.

You may be wondering why all that contract data is not just auto-fetched by
whatever is running it. It's a good question. The answer is that we actually want our
contracts to be lazy with fetching. Copying data is a very expensive operation,
and someone has to pay, as realized by the gas cost. It was mentioned before that as a contract developer you don't need to worry about gas at all. Well, that was only partially true. While you don't need to know
exactly how gas is calculated, lowering your contract's gas cost makes execution cheaper, which is a good thing. One good practice to aim for is simply to avoid fetching any data that you will not use in a
particular call.

## Messages

On the blockchain, contracts communicate with each other via JSON
messages. They are defined in most contracts in the `msg.rs` file. Let's take
a look at it.

There are three types defined in it, so let's go through them one by one.
The first one is an `InstantiateMsg`. This message is sent
on contract instantiation and typically contains data that
is needed to properly initialize it. In most cases, this just has a
simple structure.

Then there are two enums: `ExecuteMsg`, and `QueryMsg`. They are
enums because every single one of their variants represents a different
message which can be sent. For example, the `ExecuteMsg::UpdateAdmin`
corresponds to the `update_admin` message we were sending previously.

Note, that all the messages are attributed with
`#[derive(Serialize, Deserialize)]`, and
`#[serde(rename_all="snake_case")]`. These attributes come from
the [serde](https://serde.rs/) crate, and they help us with
deserialization (and serialization in case of sending
messages to other contracts). The second one is not strictly required,
but it allows us to keep camel-case style in our Rust code while encoding the JSON in snake-case as is more 
usual in this format.

You are encouraged to take a closer look at the `serde` documentation as everything there can be used with the messages.

One important thing to notice - empty variants of those enums,
tend to use empty brackets, e.g. `Admin {}` instead of
the more Rusty `Admin`. This is on purpose, to make the encoded JSON cleaner,
and it is related to how `serde` serializes enum.

Also worth noting is that these message types are not set in stone, in fact
we can use anything. This is just the convention, and you do sometimes
see things like `ExecuteCw4Msg` or similar. Just keep
in mind that it can be helpful to make sure your message names are obvious in terms of their
purpose - sticking to `ExecuteMsg`/`QueryMsg` is generally a good
idea.

## Entry points

So now that we have our contract messages, we need a way to handle
them. They are sent to our contract via entry points. There are
three entry points in the `cw4-group` contract:

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // ...
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // ..
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // ..
}
```

Those functions are called by the CosmWasm virtual machine when
a message is to be handled by the contract. You can think of them
as something like the `main` function of normal programs, except they have signatures
that better suit interction with the blockchain.

It is very important that the names of these entry points are fixed (similarly to
the `main` function).  This ensures the virtual machine knows
exactly what to call.

So, let's start with the first line. Every entry point is attributed with
`#[cfg_attr(not(feature = "library"), entry_point)]`. It may look a bit
scary, but it is just a conditional version of `#[entry_point]` -
the attribute is applied if and only if the "library" feature is not set.
We do this so that we are able to use our contracts as dependencies for other
contracts. Since the final binary can contain only one copy of each entry point,
this makes sure that only the top-level one is compiled with this
feature (entry point).

The `entry_point` attribute is a macro that generates some boilerplate.
As the binary is run by WASM virtual machine, it doesn't actually know much about
Rust types.  The actual entry point signatures are very inconvenient to
use. To overcome this issue this macro is used to generate the appropriate
entry points that will call our functions for us.

Now take a look at the arguments of the functions. Every single entry point takes as
the last argument the message which triggered its execution (except for
`reply` which will be explained later). In addition to this, the following arguments are provided by the blockchain:

* `Deps` or `DepsMut` object is the gateway to the world outside the smart contract context. It allows
  access to the contract state, as well as querying other contracts. It 
  also delivers an `Api` object with a couple of useful utility functions.
  The difference between them is that `DepsMut` allows the state to be updated, while `Deps`
  only allows it to be looked at.
* `Env` object delivers information about the blockchain state at the
  moment of execution - its height, the timestamp of execution and information
  about the executing contract itself.
* `MessageInfo` object is information about the contract call - it
  contains the address of the sender of the message and any funds sent with the
  message.

Keep in mind that the signatures of these functions are fixed (except for
the messages type), so for example you cannot interchange `Deps` with `DepsMut` to
update the contract state in the query call!

The last element of the entry points is the return type. Every entry point returns
a `Result` type which includes any error that can be turned into a string.  In case of
contract failure, the returned error is just logged. In most cases, the error
type is defined for the contract itself, typically using the
[thiserror](https://docs.rs/thiserror/latest/thiserror/) crate. `Thiserror` is
not strictly required here, but is strongly recommended - using it makes error
definitions very straightforward and improves the testability of the
contract.

Its important to understand the `Ok` part of `Result`. Let's start with the
`query` because this one is the simplest. The query always returns the `Binary`
object on the `Ok` case, which contains just a serialized response.
The most common way to create it is just to call the `to_binary` method
on an object implementing `serde::Serialize`.  Responses are typically
defined in `msg.rs` next to message types.

Slightly more complicated is the return type returned by the other entry
points - the `cosmwasm_std::Response` type. This one stores everything that is
needed to complete contract execution. This consists of three chunks of
information.

The first one is an `events` field. It contains all events that will
be emitted to the blockchain as a result of the execution. Events have
a really simple structure: they have a type which is just a string,
and a list of attributes which are just string-string key-value pairs.

You may notice that there is another `attributes` field on the `Response`.
This is just for convenience since most executions will return
only a single event: to make it a bit easier to operate one, a set of attributes are defined directly on the response. All of them will be converted
to a single `wasm` event which will be emitted. For this reason, we can consider
`events` and `attributes` to be the same chunk of data.

Finally we have the messages field, of `SubMsg` type. This one is the glue
of cross-contract communication. These messages will be sent to the
contracts after processing. It is important to remember that the whole execution is
not considered complete until or unless the processing of all sub-messages scheduled by the contract
completes. So, if the group contract sends some messages as a result of
`update_members` execution, the execution would be considered done only if
all the messages sent by it are handled (even if this handling results in failure).

When all the sub-messages sent by the contract are processed, then all the
attributes generated by all sub-calls and top-level calls are collected and
reported to the blockchain. There is one additional piece of information to consider-
the `data`. This is another `Binary` field, just like the result of a query
call and similarly it typically contains serialized JSON. Every contract
call can return some additional information in any format. You may be wondering why do we even bother returning attributes if this is the case? It is because they form a
completely different way of emitting events and data. Any attributes emitted by
the contract will be visible on the blockchain eventually unless the whole
message handling fails. So, if your contract emitted some event as a result of
being a sub-call of some bigger use case, the event will always be there visible
to everyone. This is not true for data. Every contract call will return only
a single `data` chunk, and it has to decide if it will just forward the `data`
field of one of the sub-calls "as is", or if it will construct something for itself.
This will be explained in more detail later.

## Sending submessages

We won't go into too many details on the `Response` API, since these can be read
directly from the documentation, but let's take a bit of a closer look at the part
about sending messages.

The first function to use here is `add_message`, which takes as an argument the
`CosmosMsg` (or rather anything that can be converted to it). A message added to a response
in this way will be sent and processed, and its execution will not affect the
result of the contract at all.

The other function to use is `add_submessage`, taking a `SubMsg` argument. This 
doesn't differ much from `add_message` since `SubMsg` just wraps the `CosmosMsg` while
adding some more info to it: the `id` field, and `reply_on`. There is also a
`gas_limit` property, but it is not so important (it causes sub-message
processing to fail early if the gas threshold is reached).

`reply_on` describes whether the `reply` message should be
sent on processing success, on failure, or both.

The `id` field is an equivalent of the "order id" in our KFC example from before. Without this field, if we sent multiple different sub-messages, it would be
impossible to distinguish between them. It would not even be
possible to figure out what the type of the original message handling the reply was! This is
why the `id` field exists - it can be set to any value when sending a sub-message and then on receiving the reply you can figure out what is happening based on
it.

Another important note here - you don't need to worry about setting up some sophisticated way
of generating ids. Remember that the whole process is atomic and only one
execution can be in progress at a time. In most cases, your contract sends a
fixed number of sub-messages on very concrete executions. This means that in practice you
can hardcode most of those ids while sending (preferably using some constant).

To easily create submessages, instead of setting all the fields separately
you would typically use the following helper constructors: `SubMsg::reply_on_success`,
`SubMsg::reply_on_error` and `SubMsg::reply_always`.

## CosmosMsg

If you take a look at the `CosmosMsg` type you may be very surprised - there
are so many variants, and it is not obvious how they relate to
communication with other contracts.

The message you are looking for is the `WasmMsg` (`CosmosMsg::Wasm` variant).
This one is very similar to what we already know. It has a few 
variants for operations to be performed by contracts: `Execute`, and
`Instantiate` (so we can create new contracts in contract executions).  It also has 
`Migrate`, `UpdateAdmin`, and `ClearAdmin` variants which are used to manage
migrations, which we'll discuss more at the end of this chapter.

Another interesting message is the `BankMsg` (`CosmosMsg::Bank`). This one
allows a contract to transfer native tokens to other contracts (or burn them -
equivalent to transferring them to some black hole contract). You may like to think
about this as being like sending a message to a very special contract responsible for handling
native tokens. Of course, in reality this is not a real contract, as it is in fact handled by the blockchain
itself, but this may be a way of simplifying things conceptually.

Other variants of `CosmosMsg` are not of much interest to us for now. The `Custom`
one is there to allow other CosmWasm-based blockchains to add some
blockchain-handled variant of the message. This is why most
message-related types in CosmWasm are generic over some `T` (to allow a
blockchain-specific type of message). We will never use it in the `wasmd`. All
other messages are related to advanced CosmWasm features and we shall not
describe them here.

## Reply handling

So now that we know how to send a sub-message, it is time to talk about
handling the reply. When sub-message processing is finished, and a
reply is requested, the contract is called with the following entry point:

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    // ...
}
```

The `DepsMut`, and `Env` arguments are already familiar, but there is a new
one which replaces the usual message argument: `cosmwasm_std::Reply`.

This is a type representing the execution status of the sub-message. It is a
slightly processed `cosmwasm_std::Response`. The first important thing it contains
is an `id` - the same one that you set when sending sub-message, so now you can
identify the response. The other one is the `ContractResult`, which is very
similar to the Rust `Result<T, String>` type, except that it exists for
serialization purposes. You can easily convert it into a `Result` by using the
`into_result` function.

In the error case of `ContractResult`, we get a string. As mentioned
before, errors are converted to strings right after execution. The `Ok` case
contains `SubMsgExecutionResponse` with two fields: `events` emitted by
the sub-call, and the `data` field embedded in the response.

Also mentioned before, you never need to worry about forwarding events - CosmWasm
does it automatically anyway. The `data` is another story, however. Every call will return only a single data object. When sending
sub-messages and not capturing a reply, this will always be whatever is returned
by the top-level message. However, when `reply` is called this does not have to be the case. If a
a reply is called, then it is a function which makes a decision about the final `data`. It can
decide to either forward the data from the sub-message (by returning `None`) or
to overwrite it. What it cannot do is choose to return data from the original execution
processing.  If the contract sends sub-messages waiting for replies, it is not
supposed to return any data, unless and until replies are called.

But what happens if multiple sub-messages are sent? What would the final
`data` contain? The rule is - the last non-None one. All sub-messages are always
called in the same order as they are added to the `Response`. As the order is
deterministic and well defined, it is always easy to predict which reply will
be used.

## Migrations

Migrations were mentioned earlier when describing `WasmMsg`. Migration
is another action that contracts can perform, and is somewhat similar to instantiate.
In software engineering, it is common to release updated versions of applications. This is also true in the case of blockchain - smart contracts can be updated with new features. In such cases, new
code is uploaded and the contract is migrated so that it knows that from
this point on, its messages are to be handled by another (updated) contract code.

However, it may be that the contract state used by the older version of the
contract differs from the new one. This is not a problem if some info was
added (for example some additional map - this would just be empty right
after migration). Complications arise when the state actually changes,
for example if a field is renamed. In this case, every contract execution
would fail because of (de)serialization problems. Even more subtle
cases can cause problems, such as adding a map which isn't empty but in fact needs to be synchronised with the whole contract state.

This is the purpose of the `migration` entry point. It looks like this:

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response<T>, ContracError> {
    // ..
}
```

`MigrateMsg` is the type defined by the contract in `msg.rs`.
The `migrate` entry point will be called at the moment of performing
the migration, and it is responsible for making sure the state is correct
after the migration. It is very similar to schema migrations in traditional
database applications. It is also rather challenging, because of the complicated version
management involved.  You can never assume that you are migrating a contract
from the previous version! It can be migrated from any version, released
anytime - even later than the version we are migrating to!

It is worth bringing back one issue from the past - the contract admin. Do you
remember the `--no-admin` flag we set previously on every contract
instantiation? It made our contract unmigrateable. Migrations can be performed
only by the contract admin. To be able to use it, you should pass a `--admin address`
flag instead, with the `address` being the address that will be able to
perform migrations.

## Sudo

Sudo is the last basic entry point in `CosmWasm`, and it is one we won't use in `wasmd`. It is equivalent to `CosmosMsg::Custom`, but instead of
being a special blockchain-specific message sent and handled by the
blockchain itself, it is now a special blockchain-specific message sent by the
blockchain to the contract in some conditions. There are many uses cases, but since they are not related to `CosmWasm` we will not cover them here. The signature of `sudo` looks like this:

```rust
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    // ..
}
```

The important difference is that since `sudo` messages are blockchain
specific, so the `SudoMsg` type is typically defined by some blockchain helper
crate, not the contract itself.
