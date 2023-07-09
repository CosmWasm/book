# Actors in blockchain

In the previius section we discussed actors mostly in the abstract, staying clear of any
blockchain-specific terms. However, we need to establish some common language before we dive into the code.  To do so we shall look at contracts from
the perspective of external users, rather than their implementation.

In this section, we shall use the `wasmd` binary to communicate with the 
testnet. To properly set it up, check out [Quick start with
`wasmd`](../wasmd-quick-start.md).

## Blockchain as a database

Although it is in some regards starting from the end, we'll start by looking at the state part of
the Actor Model. Insofar as it relates to traditional systems, there is one particular thing
we can directly compare blockchain with - a database!

In the previous section we learned that the most important part of
a contract is its state. Manipulating the state is the only way to persistently
manifest work performed to the world. But what else shares the purpose of maintaining a state? A database!

As a contract developer, the author's point of view is that we can think of it as a distributed
database with some magical mechanisms added to make it democratic. Those "magical
mechanisms", although crucial for a blockchain's existence (they are in fact the reasons why we even
use a blockchain), are not relevant from the contract creator's point of
view.  For us, all that matters is the state.

But you may protest: what about the financial part?! Isn't a blockchain (`wasmd` in particular)
a currency implementation? Indeed, with all of those gas costs, sending funds certainly can seem
very much more like a money transfer than a database update. And yes, you'd be somewhat right,
although there is an argument to this too. Imagine that for every native token (by
"native tokens" we meant tokens handled directly by the blockchain as opposed to cw20 contract tokens, for example) there is a special database bucket/table that maps addresses to token balances (ie how much of a token the address owns). You can query
this table (query for token balance), but you cannot modify it directly. To modify
it you must send a message to a special build-in bank contract. Looked at like this, everything
is still just a database.

But if a blockchain is a database, then where are smart contracts stored?
Well, obviously - in the database itself! So now imagine another special table - one containing a single table of code-ids mapped to blobs of wasm binaries. To operate on this table, you use a "special contract" which is not accessible
from another contract, but instead via the `wasmd` binary.

You may be asking: why do I even care about viewing a blockchain as a database? The reason
is that it makes reasoning about everything in blockchain very much more natural. Do you
recall that every message in the Actor Model is transactional? It perfectly
matches traditional database transactions i.e. every message starts a new
transaction! Also, when we talk about migrations later, it will turn out that
migrations in CosmWasm are very much equivalents of schema migrations in
traditional databases!

So, the thing to keep in mind - a blockchain is very similar to a database, having some
specially reserved tables (like native tokens, code repository, etc.), with a special
bucket created for every contract. A contract can look at every table in every
bucket on the whole blockchain, but it can only modify its own (the one it created).

## Compile the contract

We won't go directly into the code just yet, but to start we'll need at least a compiled
contract binary. The `cw4-group` contract from
[cw-plus](https://github.com/CosmWasm/cw-plus) is simple enough to work with for
now, let's start by compiling it. First clone the repository:

```bash
$ git clone git@github.com:CosmWasm/cw-plus.git
```

Then go to `cw4-group` contract and build it:

```bash
$ cd cw-plus/contracts/cw4-group
$ docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.6
```

Your final binary should be located in the
`cw-plus/artifacts` folder (`cw-plus` being where you cloned your repository).

## Contract code

When the contract binary is built, the first interaction with CosmWasm is uploading
it to the blockchain.  Assuming you have your wasm binary in the working directory:

```bash
$ wasmd tx wasm store ./cw4-group.wasm --from wallet $TXFLAG -y -b block
```

As a result of this operation you should get json output that looks like this:

```
..
logs:
..
- events:
  ..
  - attributes:
    - key: code_id
      value: "1069"
    type: store_code
```

We've ignored most of the fields as they are not relevant for now - what we care
about is the event emitted by the blockchain with information about the `code_id` of
stored contract. In this case the contract code was stored in blockchain under
the id of `1069`. We can now look at the code by querying for it:

```bash
$ wasmd query wasm code 1069 code.wasm
```

And now the important thing - the contract code is not an actor! So, what is exactly is 
contract code then? Perhaps the easiest way to think about it is as similar to a `class` or
a `type` in programming. It provides a blueprint for what can be done, but the
class itself is in most cases not very useful unless we create an instance
of a type on which we can call its class methods. So now let's move on to using
instances of such contract classes.

## Contract instance

Now we have our contract code, but what we want is an actual contract itself.
To create it, we need to instantiate it. Instantiation is analogous to calling a constructor in regular (object-oriented) programming. To do this, we need to send an instantiate message to the contract:

```bash
$ wasmd tx wasm instantiate 1069 '{"members": []}' --from wallet --label "Group 1" --no-admin $TXFLAG -y
```

What we've done here is to create a new contract and immediately call the `Instantiate`
message on it. The structure of such a message is different for every contract
code. In particular, the `cw4-group` Instantiate message contains two fields:

* `members` field, which is the list of initial group members 
* `admin` an optional field that defines an address, specifying who can add or remove
  group members

In this example, we've created an empty group with no admin - so it could never
change! It may seem not seem like a very useful contract, but it serves our needs as a first
example.

As the result of instantiating, we get this:

```
..
logs:
..
- events:
  ..
  - attributes:
    - key: _contract_address
      value: wasm1u0grxl65reu6spujnf20ngcpz3jvjfsp5rs7lkavud3rhppnyhmqqnkcx6
    - key: code_id
      value: "1069"
    type: instantiate
```

As you can see, we once more look at the `logs[].events[]` field, looking for an
interesting event and extracting information from it.  This is the common case.
We will talk more about events and their attributes later, but in general
they are ways to notify the world that something happened. Do you remember the
KFC example? If a waiter is serving our dish, they could put a tray on the bar and then 
yell the order number (or put it on a screen).  This would be
announcing an event, so that one can get some summary of the operation and possibly go and
do something useful with it.

So, what can we do with the contract? Well, obviously we can call it! But first
we need to discuss addresses.

## Addresses in CosmWasm

An address in CosmWasm is a way to refer to entities on the blockchain. There are
two types of addresses: contract addresses, and non-contract addresses ("non-contracts"). The difference
is that you can also send messages to contract addresses, as there is some smart
contract code associated with them, while non-contracts are just ordinary users of the
system. In an Actor Model, contract addresses represent actors, and
non-contracts represent clients of the system.

When operating with the blockchain using `wasmd`, you also have an address yourself - you
got one when you added the key to `wasmd`:

```bash
# add wallets for testing
$ wasmd keys add wallet3
- name: wallet3
  type: local
  address: wasm1dk6sq0786m6ayg9kd0ylgugykxe0n6h0ts7d8t
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"Ap5zuScYVRr5Clz7QLzu0CJNTg07+7GdAAh3uwgdig2X"}'
  mnemonic: ""
```

You can always check your address:

```bash
$ wasmd keys show wallet
- name: wallet
  type: local
  address: wasm1um59mldkdj8ayl5gknp9pnrdlw33v40sh5l4nx
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A5bBdhYS/4qouAfLUH9h9+ndRJKvK0co31w4lS4p5cTE"}'
  mnemonic: ""
```

Having an address is very important because it is a requirement for being able
to call anything. When we send a message to a contract it always needs to know the
address of the entity that sent it (the sender) - not to mention that
this sender is an address that will pay any gas fees.

## Querying the contract

So, now we have our contract, let's try to do something with it - a query would be the
easiest thing to do. Let's do it:

```bash
$ wasmd query wasm contract-state smart wasm1u0grxl65reu6spujnf20ngcpz3jvjfsp5rs7lkavud3rhppnyhmqqnkcx6 '{ "list_members": {} }'
data:
  members: []
```

The `wasm...` string is the contract address, and you should substitute it with
your own contract address. `{ "list_members": {} }` is the query message we send to the
contract. Typically, CosmWasm smart contract queries are given in the form of a single JSON
object, with one field: the query name (`list_members` in our case). The value
of this field is another object, holding query parameters (if there are any). The
`list_members` query handles two optional parameters: `limit`, and `start_after`, which support result pagination when used. However, in our case of
an empty group they aren't needed.

The query result we got is in human-readable text form. Iif we wanted to get the
JSON from - for example, to process it further with `jq` - we would just pass the
`-o json` flag. As you can see, the response contains just one field: `members` which is
an empty array.

So, can we do anything else with this contract? Not really. Let's try to do
something with a new one!

## Executions to perform some actions

The problem with our previous contract is that for the `cw4-group` contract,
the only one who can perform executions on it is an admin, but our contract
instance didn't have one. This is not a requirement for every smart contract, but it is the
nature of this one.

So, let's make a new group contract, but this time we will
make ourselves an admin. First, check our wallet address:

```bash
$ wasmd keys show wallet
```

And instantiate a new group contract - this time with proper admin:

```bash
$ wasmd tx wasm instantiate 1069 '{"members": [], "admin": "wasm1um59mldkdj8ayl5gknp9pnrdlw33v40sh5l4nx"}' --from wallet --label "Group 1" --no-admin $TXFLAG -y
..
logs:
- events:
  ..
  - attributes:
    - key: _contract_address
      value: wasm1n5x8hmstlzdzy5jxd70273tuptr4zsclrwx0nsqv7qns5gm4vraqeam24u
    - key: code_id
      value: "1069"
    type: instantiate
```

You may ask, why are we passing the `--no-admin` flag if we just said we
want to set an admin to the contract? The answer is sad and confusing, but...
it is a different admin. The admin we want to set is one checked and managed by the
contract itself. The admin which is declined with the
`--no-admin` flag is a wasmd-level admin, and is able to migrate the contract. You
don't need to worry about the latter at least until you learn about
contract migrations. Until then you can always pass the `--no-admin` flag to
the contract.

Now let's query our new contract for the member's list:

```bash
$ wasmd query wasm contract-state smart wasm1n5x8hmstlzdzy5jxd70273tuptr4zsclrwx0nsqv7qns5gm4vraqeam24u '{ "list_members": {} }'
data:
  members: []
```

Just like before - no members initially. Now check an admin:

```
$ wasmd query wasm contract-state smart wasm1n5x8hmstlzdzy5jxd70273tuptr4zsclrwx0nsqv7qns5gm4vraqeam24u '{ "admin": {} }'
data:
  admin: wasm1um59mldkdj8ayl5gknp9pnrdlw33v40sh5l4nx
```

So, there is an admin, it seems like the one we wanted to have there. So now we
can add someone to the group - maybe ourselves?

```bash
wasmd tx wasm execute wasm1n5x8hmstlzdzy5jxd70273tuptr4zsclrwx0nsqv7qns5gm4vraqeam24u '{ "update_members": { "add": [{ "addr": "wasm1um59mldkdj8ayl5gkn
p9pnrdlw33v40sh5l4nx", "weight": 1 }], "remove": [] } }' --from wallet $TXFLAG -y
```

The message for modifying the members is `update_members` and it has two
fields: one for members to `remove`, and another members to `add`. Members to remove are
just addresses. The structure of the `add` field has a slightly more complex structure: they
are records with two fields, address and weight. Weight is not relevant
for us now, it is just metadata stored with every group member - for
us, it will always be 1.

Let's query the contract again to check if our message changed anything:

```bash
$ wasmd query wasm contract-state smart wasm1n5x8hmstlzdzy5jxd70273tuptr4zsclrwx0nsqv7qns5gm4vraqeam24u '{ "list_members": {} }'
data:
  members:
  - addr: wasm1um59mldkdj8ayl5gknp9pnrdlw33v40sh5l4nx
    weight: 1
```

As you can see, the contract updated its state. This is basically how
it works - sending messages to contracts causes them to update the state,
and the state can be queried at any time. To keep things simple
we have until now just been interacting with the contract directly using `wasmd`, but as mentioned
before, contracts can communicate with each other. However, to investigate
this we need to understand how to write contracts. Next time we will look
at the contract structure and we will map it bit by bit to what we have learned
so far.
