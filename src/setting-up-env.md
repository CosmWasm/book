# Setting up the environment

To work with CosmWasm smart contract, you will need rust installed on your machine. If you don't have
one, you can find installation instructions on [Rust website](https://www.rust-lang.org/tools/install).

I assume you are working with a stable Rust channel in this book.

Additionally, you will need the Wasm rust compiler backend installed to build Wasm binaries.
To install it, run:

```
rustup target add wasm32-unknown-unknown
```

Optionally if you want to try out your contracts on a testnet, you will need a
[wasmd](https://github.com/CosmWasm/wasmd) binary. We would focus on testing
contracts with Rust unit testing utility throughout the book, so it is not required
to follow. However, seeing the job working in a real-world environment may be nice.

To install `wasmd`, first install the [golang](https://github.com/golang/go/wiki#working-with-go). Then
checkout the `wasmd` and install it:

```
$ git clone git@github.com:CosmWasm/wasmd.git
$ cd ./wasmd
$ make install
```

Also, to be able to upload Rust Wasm Contracts to the blockchain, you will need to install
[docker](https://www.docker.com/). It will be required to run CosmWasm Rust Optimizer to minimize your
contract sizes - without that, more complex contracts might exceed a size limit.

## Verifying the installation

To make sure you are ready to build your smart contracts, you need to make sure you can build examples.
Checkout the [cw-plus](https://github.com/CosmWasm/cw-plus) repository, and run the testing command in
its folder:

```
$ git clone git@github.com:CosmWasm/cw-plus.git
$ cd ./cw-plus
$ cargo test
```

You should see that everything in the repository gets compiled, and all tests pass. 

`cw-plus` is a great place to find example contracts - look for them in `contracts` directory. The
repository is maintained by CosmWasm creators, so contracts in there should follow good practices.
