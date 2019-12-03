# Dothereum Node

![Dothereum Banner](https://raw.githubusercontent.com/dothereum/logo/master/dothereum-head-reddit.jpg)

[![Badge](https://github.com/dothereum/dothereum/workflows/Nightly/badge.svg)](https://github.com/dothereum/dothereum/actions)
[![Discord](https://img.shields.io/discord/587923474471845898?label=Discord)](https://discord.gg/JcAQz58)
[![License](https://img.shields.io/github/license/dothereum/dothereum.svg)](LICENSE)
[![Standard-README compliant](https://img.shields.io/badge/readme-standard-brightgreen.svg)](https://github.com/RichardLitt/standard-readme)

The Dothereum node reference implementation; written in Rust, based on Parity's Substrate `v2.0` framework. The Dothereum node is an upstream-compliant Substrate node with a customly developed Dothereum runtime. The runtime features smart-contract functionality; both for Web-Assembly and Ethereum contracts.

_Note: Dothereum is currently undergoing fast development. Functionality and documentation can change quickly without any prior notice. The best orientation about development progress and features can be find [by following the milestones](https://github.com/dothereum/dothereum/milestones)._

## Table of Contents

- [Security](#security)
- [Background](#background)
- [Install](#install)
- [Usage](#usage)
- [API](#api)
- [Contributing](#contributing)
- [License](#license)

## Security

The Dothereum node is to be considered unstable and shall not be used in production without proper testing, Q-A-ing, and auditing. At this stage, security revelations are welcome [on our issue tracker](https://github.com/dothereum/dothereum/issues).

## Background

For developers new to Dothereum, the following background references might be valuable:

- [Parity _Substrate_](https://github.com/paritytech/substrate) is a framework that contains everything one needs to build a blockchain, including but not limited to database (`rocksdb`), networking (`libp2p`), swappable consensus, customizable transaction queue, and a diverse library of runtime modules. To get started with Dothereum development, the [Substrate Developer Hub](https://substrate.dev/) has some good resources to understand how substrate-based blockchains are designed.
- [Parity _Polkadot_](https://github.com/paritytech/polkadot) is a blockchain protocol that allows anyone to operate and connect so called _parachains_ to the Polkadot relay chain with the benefit of shared security and interoperability between all the connected chains. The ultimate goal of Dothereum is to become part of the Polkadot network. To understand the mechanics of Polkadot, it's recommended to check out the [Polkadot Wiki](https://wiki.polkadot.network/) for builders; especially the sections on parachains and smart contracts.
- [Parity _Cumulus_](https://github.com/paritytech/cumulus) is a set of tools that ease the task of building parachains for Polkadot. It's currently under heavy development and not fully feature-complete. Regardless, it's worth to keep an eye on.
- The Ethereum Virtual Machine is the smart contract execution environment of [_Ethereum_](https://ethereum.org/). Dothereum uses the Rust implementation of the EVM, formerly called _SputnikEVM_, now [Rust-EVM](https://github.com/sorpaas/rust-evm). Check out the [EVM Awesome List](https://github.com/ethereum/wiki/wiki/Ethereum-Virtual-Machine-%28EVM%29-Awesome-List#other-implementations), especially the sections on implementations and programming languages.

## Install

It's recommended to manage rust with [_Rustup_](https://rustup.rs) and other system dependencies through your favorite operating-system package manager.

1. Install Rust:
  ```bash
  curl https://sh.rustup.rs -sSf | sh
  ```
2. Install additional dependencies, i.e., for Ubuntu:
  ```bash
  sudo apt install build-essential cmake pkg-config libssl-dev openssl git clang libclang-dev
  ```
3. Install required toolchain:
  ```bash
  ./scripts/init.sh
  ```
4. Ensure Cargo is in your `$PATH`:
  ```bash
  export PATH=$PATH:$HOME/.cargo/bin
  ```
5. Build the node
  ```bash
  cargo build --release
  ```
6. The binary can be found in
  ```bash
  ./target/release/dothereum
  ```

The `--release` flag is turning off the debug mode and is required to run actual Dothereum chains, e.g., a local or public testnet.

For development purposes, it's recommended to not create `--release` builds for faster compile times by simply running `cargo build`.

## Usage

You can quickly start a local **development chain** with whatever runtime is currently implemented using the `--dev` mode:

```bash
dothereum --dev
```

This preconfigured network does not have any discovery, comes with a predefined authority, and starts authoring blocks right away. It can be used for runtime, API, and application development.

_Note: Updating the runtime usually requires an runtime upgrade or simply resetting the chain and starting with a new genesis:_

```bash
dothereum purge-chain --dev
```

You can run a multi-node **local testnet** by using the built-in chain specification `--chain local` and start validating blocks with the preset accounts for `--alice` and `--bob`:

```bash
dothereum --chain local \
  --base-path /tmp/local-xth/alice \
  --alice \
  --port 31337 \
  --validator \
  --node-key 00000000000000000000000000000000000000000000000000000000000a11c3
```

The `--bootnodes` flag ensures Bob connecting to Alice. The network starts producing blocks once Alice and Bob are connected.

```bash
dothereum --chain local \
  --base-path /tmp/local-xth/bob \
  --bob \
  --port 34242 \
  --validator \
  --node-key 0000000000000000000000000000000000000000000000000000000000000b0b \
  --bootnodes /ip4/127.0.0.1/tcp/31337/p2p/QmWboyUFLWqHnkYzGLq5fYFzviDJbvuYG3RNNK5r8xZkYG
```

To add more validators to your network, use the pre-configured accounts `--charlie`, `--dave`, `--eve`, and `--ferdie`. Make sure to connect them to Alice's and Bob's nodes using the `--bootnodes` flag.

To connect to a **public testnet**, please consult the [Dothereum Wiki](https://github.com/dothereum/dothereum/wiki) for a list of past, current, and future testnets. Due to the fast development cycle, the testnets are named after the Greek alphabet, e.g., `alpha`, `beta`, `gamma`, and the code required to connect to them can be found on the corresponding Github branches.

To contribute to chain specification development, the `build-spec` command can be helpful:

```bash
dothereum build-spec --chain local > cli/res/my-local-spec.json
```

To generate a Dothereum account, you can use `subkey` which can be found in the [Substrate Github repository](https://substrate.dev/docs/en/ecosystem/subkey).

```bash
subkey --network dothereum generate
```

## API

The easiest way to interact with a Dothereum node is the [Polkadot-JS API](https://polkadot.js.org/api/). The dynamic API allows chain interactions with transparent encoding and decoding. Most of the Dothereum APIs adhere to the Substrate defaults.

Custom Dothereum types will be documented on the [Dothereum Wiki](https://github.com/dothereum/dothereum/wiki/EVM-Types) and contributed upstream to the Polkadot-JS API once the interfaces stabilize.

Additionally, Dothereum will also [implement it's own RPC-API client](https://github.com/dothereum/dothereum/issues/64). This work, however has not yet been started yet.

## Contributing

For questions and bug reports, please use the [Github issue tracker](https://github.com/dothereum/dothereum/issues). See [the contributing file](CONTRIBUTING.md)!

Detailed logs may be shown by running the node with the following environment variables set:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 dothereum --dev
```

Additional [CLI usage options](https://github.com/dothereum/dothereum/wiki/CLI-Options) are available and may be shown by running:

```bash
dothereum --help
```

[Pull requests](https://github.com/dothereum/dothereum/pulls) are welcome and accepted.

## License

[GNU GENERAL PUBLIC LICENSE, Version 3, 29 June 2007](LICENSE)
