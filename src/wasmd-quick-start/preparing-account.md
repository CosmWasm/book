# Preparing account

The first thing you need to interact with the testnet is a valid account. Start with adding a new key to the `wasmd` configuration: 

```
$ wasmd keys add wallet
- name: wallet
  type: local
  address: wasm1wukxp2kldxae36rgjz28umqtq792twtxdfe6ux
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A8pamTZH8x8+8UAFjndrvU4x7foJbCvcz78buyQ8q7+k"}'
  mnemonic: ""
...
```

As a result of this command, you get information about just the prepared account. Two things are relevant here:
* address is your identity in the blockchain
* mnemonic (omitted by myself in the example) is 12 words that allow you to recreate an account so you can use it, for
example, from a different machine

For testing purposes, storing the mnemonic is probably never necessary, but it is critical information to keep safe in the real world.

Now, when you create an account, you have to initialize it with some tokens - you will need them to pay for any interaction with
blockchain - we call this the "gas cost" of an operation. Usually, you would need to buy those tokens somehow, but in testnets,
you can typically create as many tokens as you want on your accounts. To do so on malaga network, invoke:

```
$ curl -X POST --header "Content-Type: application/json" \
  --data '{ "denom": "umlg", "address": "wasm1wukxp2kldxae36rgjz28umqtq792twtxdfe6ux" }' \
  https://faucet.malaga-420.cosmwasm.com/credit
```

It is a simple HTTP POST request to the `https://faucet.malaga-420.cosmwasm.com/credit` endpoint. The data of this request is a JSON
containing the name of a token to mint and the address which should receive new tokens. Here we are minting `umlg` tokens, which are
tokens used to pay gas fees in the malaga testnet.

You can now verify your account tokens balance by invoking (substituting my address with yours):

```
$ wasmd query bank balances wasm1wukxp2kldxae36rgjz28umqtq792twtxdfe6ux
balances:
- amount: "100000000"
  denom: umlg
pagination:
  next_key: null
  total: "0"
```

100M tokens should be plenty for playing around, and if you need more, you can always mint another batch.
