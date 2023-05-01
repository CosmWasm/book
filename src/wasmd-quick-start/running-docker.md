# Running Docker

If you are using a Windows machine, or have an M1 chip then it's best to use
the Docker images that are provided. This section is a quick guide on working with
`wasmd` through docker to test your smart contracts on the blockchain. If you are
running `wasmd` locally or only want to develop contracts and test them with the
UT environment, feel free to skip this section.

If you have never heard of [docker](https://www.docker.com/), it is a way to
ensure applications that have been prepared for specific systems and environments
can be used easily by others no matter what system they might have themselves.
After installing docker you can grab the latest image of `wasmd`:

```bash
$ docker image pull cosmwasm/wasmd:latest
```

Now you have the docker image you can initialize your own local testnet by setting
up a validator:

```
$ docker run --rm -it -e PASSWORD=xxxxxxxx \
  --mount type=volume,source=wasmd_client,target=/root \
  cosmwasm/wasmd:latest /opt/setup_wasmd.sh
```

The `PASSWORD` parameter is setting an env variable for later use that you'll 
need to enter every time a tx needs signing. The output of the above command 
will give you the address that you'll add as a validator to our testnet, along 
with its mnemonic. Save the details of these so you can use them for querying 
later. The output will look something like the following:

```
- name: validator
  type: local
  address: wasm1u0grxl65reu6spujnf20ngcpz3jvjfsp5rs7lkavud3rhppnyhmqqnkcx6
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A8pamTZH8x8+8UAFjndrvU4x7foJbCvcz78buyQ8q7+k"}'
```

The command also created a new volume called `wasmd_client`. This is the saved 
state of our container that can be run at any time so that you can interact with 
the local testnet. Now let's create a new container that will set our above 
address as a genesis validator:

```
$ docker run --rm -it \
  --mount type=volume,source=wasmd_data,target=/root \
  cosmwasm/wasmd:latest /opt/setup_wasmd.sh wasm1u0grxl65reu6spujnf20ngcpz3jvjfsp5rs7lkavud3rhppnyhmqqnkcx6
```

A new volume that contains the blockchain state was created called `wasmd_data`.
To start up the blockchain and servers, invoke:

```
$ docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 \
  --mount type=volume,source=wasmd_data,target=/root \
  cosmwasm/wasmd:latest /opt/run_wasmd.sh
```

From now on, if you ever need to access this blockchain and carry on from where
you left off, run the above command. After executing the command, the terminal 
will output the logs of all new blocks being produced. In a separate terminal 
window you can interact with this testnet.

```
$ docker run --rm -it \
  --mount type=volume,source=wasmd_client,target=/root \
  --network=host \
  cosmwasm/wasmd:latest wasmd \
  query bank balances wasm1u0grxl65reu6spujnf20ngcpz3jvjfsp5rs7lkavud3rhppnyhmqqnkcx6

...
balances:
- amount: "1000000000"
  denom: ucosm
- amount: "750000000"
  denom: ustake
pagination:
  next_key: null
  total: "0"
...
```

To create a new wallet, simply invoke:

```
$ docker run --rm -it \
  --mount type=volume,source=wasmd_client,target=/root \
  --network=host \
  cosmwasm/wasmd:latest wasmd \
  keys add wallet

...
- name: wallet
  type: local
  address: wasm1wukxp2kldxae36rgjz28umqtq792twtxdfe6ux
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A8pamTZH8x8+8UAFjndrvU4x7foJbCvcz78buyQ8q7+k"}'
  mnemonic: ""
...
```

If you want to transfer some funds to the new wallet you can send some from your 
validator address:

```
$ docker run --rm -it \
  --mount type=volume,source=wasmd_client,target=/root \
  --network=host cosmwasm/wasmd:latest wasmd \
  tx bank send validator wasm1wukxp2kldxae36rgjz28umqtq792twtxdfe6ux 1000000ucosm
```

This will send 1,000,000 ucosm to the new wallet you created.

In the next section when you will upload a smart contract and interact with it, 
ensure that all your `wasmd` commands are prepended with your docker container
command:

```
$ docker run --rm -it \
  --mount type=bind,src="$(pwd)",target=/code \ 
  --mount type=volume,source=wasmd_client,target=/root \
  --network=host cosmwasm/wasmd:latest wasmd
```
