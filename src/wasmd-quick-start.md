# Quick start with `wasmd`

In the past, we suggested playing with contracts on the `malaga` testnet using `wasmd`.
Now `malaga` is no longer operative, and the best way to test the contract in the
real environment is to use one of the big CosmWasm chains testnets - Osmosis, Juno,
Terra, or other ones.
[Here](https://docs.osmosis.zone/cosmwasm/testnet/cosmwasm-deployment/) is the
recommended introduction on how to start with the Osmosis testnet.

# Getting Started with wasmd

Welcome to the world of CosmWasm development! If you want to dive into smart contract development on the Cosmos network, starting with wasmd is a fantastic choice. wasmd is the backbone for deploying and managing CosmWasm smart contracts on various Cosmos SDK-based blockchains. This guide will walk you through the initial steps of getting started with wasmd, focusing on using the Osmosis testnet as your playground for deploying and testing your contracts.

### Step 1: Setting Up Your Development Environment

Before you begin, ensure your development environment is ready. You'll need:

- **Go:** Install the latest version of Go (at least version 1.17).
- **Rust:** CosmWasm contracts are written in Rust, so you'll need the Rust toolchain.
- **Node.js and Yarn:** For managing front-end applications and script tooling.

After setting up the necessary tools, install wasmd by cloning the repository and following the build instructions in the README file.

### Step 2: Joining the Osmosis Testnet

The Osmosis testnet offers a vibrant environment for testing your CosmWasm contracts. To join the testnet:[Here](https://docs.osmosis.zone/cosmwasm/testnet/cosmwasm-deployment/)

1. **Get Testnet Tokens:** Visit the Osmosis faucet to obtain testnet tokens. These tokens are used for deploying contracts and executing transactions on the testnet.
2. **Configure wasmd:** Follow the Osmosis documentation to configure your wasmd instance for the Osmosis testnet. This typically involves setting the correct endpoints and configuring your wallet.

### Step 3: Writing Your First Contract

With your environment set up and connected to the Osmosis testnet, it's time to write your first Contract. Start with a simple contract, such as a "Hello, World!" or a basic counter. The CosmWasm documentation provides templates and tutorials to get you started.

### Step 4: Compiling and Deploying

Once your Contract is ready:

1. **Compile Your Contract:** Use cargo wasm to compile your Rust contract into WebAssembly (Wasm).
2. **Deploy to the Testnet:** Use wasmd to deploy your compiled Contract to the Osmosis testnet. You must specify your Contract's Wasm file and provide an instantiation message.

### Step 5: Interacting with Your Contract

After deployment, interact with your Contract through wasmd by sending executed transactions. You can also query your Contract's state to see the results of your interactions.

### Next Steps

Congratulations! You've just deployed your first CosmWasm contract to the Osmosis testnet. From here, you can:

- Explore more complex contract logic.
- Participate in the CosmWasm community for support and collaboration.
- Test your contracts on other CosmWasm chain testnets like Juno and Terra for broader exposure.

This quick start guide is just the beginning of your journey with wasmd and CosmWasm. As you become more comfortable with contract development, you'll discover the power and flexibility of the CosmWasm ecosystem. Happy coding!