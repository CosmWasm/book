# Actor model

This section describes the fundaments of CosmWasm smart contracts architecture, which determines how do they communicate
with each other. I want to go through this before teaching step by step how to create multiple contracts relating to each
other, to give you a grasp of what to expect. Don't worry if it will not be clear after the first read - I suggest going
through this chapter once now and maybe giving it another take in the future when you know the practical part of this.

The whole thing described here is officially documented in the
[SEMANTICS.md](https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md), of the `cosmwasm` repository.
