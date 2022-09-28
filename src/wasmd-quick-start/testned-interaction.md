# Interaction with testnet

Blockchain interaction is performed using the
[wasmd](https://github.com/CosmWasm/wasmd) command-line tool. To start working
with the testnet, we need to upload some smart contract code. For now, we would
use an example `cw4-group` from the `cw-plus` repository. Start with cloning
it:

```
$ git clone git@github.com:CosmWasm/cw-plus.git
```

Now go to cloned repo and run Rust optimizer on it:

```
$ docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.6
```

After a couple of minutes - it can take some for the first time - you should
have an `artifact` directory in your repo, and there should be a
`cw4-group.wasm` file being the contract we want to upload. To do so, run -
note that `wallet` is name of the key you created in the previous chapter:

```
$ wasmd tx wasm store ./artifacts/cw4_group.wasm --from wallet $TXFLAG -y -b block

...
logs:
- events:
  - attributes:
    - key: action
      value: /cosmwasm.wasm.v1.MsgStoreCode
    - key: module
      value: wasm
    - key: sender
      value: wasm1wukxp2kldxae36rgjz28umqtq792twtxdfe6ux
    type: message
  - attributes:
    - key: code_id
      value: "12"
    type: store_code
...
```

As a result of execution, you should get a pretty long output with information
about what happened. Most of this is an ancient cipher (aka base64) with
execution metadata, but what we are looking for is the `logs` section. There
should be an event called `store_code,` with a single attribute `code_id` - its
`value` field is the code id of our uploaded contract - 12 in my case.

Now, when we have our code uploaded, we can go forward and instantiate a
contract to create its new instance:

```
$ wasmd tx wasm instantiate 12 \
  '{ "admin": "wasm1wukxp2kldxae36rgjz28umqtq792twtxdfe6ux", "members": [] }' \
  --from wallet --label "Group" --no-admin $TXFLAG -y

...
logs:
- events:
  - attributes:
    - key: _contract_address
      value: wasm18yn206ypuxay79gjqv6msvd9t2y49w4fz8q7fyenx5aggj0ua37q3h7kwz
    - key: code_id
      value: "12"
    type: instantiate
  - attributes:
    - key: action
      value: /cosmwasm.wasm.v1.MsgInstantiateContract
    - key: module
      value: wasm
    - key: sender
      value: wasm1wukxp2kldxae36rgjz28umqtq792twtxdfe6ux
    type: message
...

```

In this command, the `12` is the code id - the result of uploading the code.
After that, a JSON is an instantiation message - I will talk about this later.
Just think about it as a message requiring fields to create a new contract.
Every contract has its instantiation message format. For `cw4-group`, there are
two fields: `admin` is an address that would be eligible to execute messages on
this contract. It is crucial to set it to your address, as we will want to
learn how to execute contracts. `members` is an array of addresses that are
initial members of the group. We leave it empty for now, but you can put any
addresses you want there. Here, I put one hint about messages inline into the
command line, but I often put messages to be sent to the file and embed them
via `$(cat msg.json)`. It is fish syntax, but every shell provides a syntax for
this.

Then after the message, you need to add a couple of additional flags. The
`--from wallet` is the same as before - the name of the key you created
earlier. `--label "Group"` is just an arbitrary name for your contract. An
important one is a `--no-admin` flag - keep in mind that it is a different
"admin" that we set in the instantiation message. This flag is relevant only
for contract migrations, but we won't cover them right now, so leave this flag
as it is.

Now, look at the result of the execution. It is very similar to before - much
data about the execution process. And again, we need to take a closer look into
the `logs` section of the response. This time we are looking at an event with
type `instantiate`, and the `_contract_address` attribute - its value is newly
created contract address - `wasm1wukxp2kldxae36rgjz28umqtq792twtxdfe6ux` in an
example.

Now let's go forward with querying our contract:

```
$ wasmd query wasm contract-state smart \
  wasm18yn206ypuxay79gjqv6msvd9t2y49w4fz8q7fyenx5aggj0ua37q3h7kwz \
  '{ "list_members": {} }'

data:
  members: []
```

Remember to change the address (right after `smart`) with your contract
address. After that, there is another message - this time the query message -
which is sent to the contract. This query should return a list of group
members. And in fact, it does - response is a single `data` object with a
single field - empty members list. That was easy, now let's try the last thing:
the execution:

```
$ wasmd tx wasm execute \
  wasm18yn206ypuxay79gjqv6msvd9t2y49w4fz8q7fyenx5aggj0ua37q3h7kwz \
  '{ "update_members": { "add": [{ "addr": "wasm1wukxp2kldxae36rgjz28umqtq792twtxdfe6ux", "weight": 1 }], "remove": [] } }' \
  --from wallet $TXFLAG
```

As you can see, execution is very similar to instantiation. The differences
are, that instantiation is called just once, and execution needs a contract
address. It is fair to say that instantiation is a particular case for first
execution, which returns the contract address. Just like before we can see that
we got some log output - you can analyze it to see that something probably
happened. But to ensure that there is an effect on blockchain, the best way
would be to query it once again:

```
$ wasmd query wasm contract-state smart \
  wasm18yn206ypuxay79gjqv6msvd9t2y49w4fz8q7fyenx5aggj0ua37q3h7kwz \
  '{ "list_members": {} }'

data:
  members:
  - addr: wasm1wukxp2kldxae36rgjz28umqtq792twtxdfe6ux
    weight: 1
```

For the time being, this is all you need to know about `wasmd` basics in order
to be able to play with your simple contracts. We would focus on testing them
locally, but if you want to check in real life, you have some basics now.
We will take a closer look at `wasmd` later when we would talk about the
architecture of the actor model defining communication between smart contracts.
