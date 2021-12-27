# Notes

[Blog Post](https://dev.to/dabit3/the-complete-guide-to-full-stack-solana-development-with-react-anchor-rust-and-phantom-3291) | [GitHub Repo](https://github.com/dabit3/complete-guide-to-full-stack-solana-development)

## The Tech Stack
1. React
* A client-facing framework which offers quick/easy development for creating interactive UIs.
* [Documentation](https://reactjs.org/docs/getting-started.html)
2. Anchor
* A framework for developing on Solana's [Sealevel](https://medium.com/solana-labs/sealevel-parallel-processing-thousands-of-smart-contracts-d814b378192) runtime and provides several developer tools for writing smart contracts.
* [Documentation](https://project-serum.github.io/anchor/getting-started/introduction.html)
3. Rust
* Statically-typed, general-purpose programming language desgined for safety and performance; most especially safe concurrency and memory management.
* [Documentation](https://doc.rust-lang.org/book/title-page.html).
4. Phantom
* A crypto wallet built for the Solana blockchain.
* [Documentation](https://docs.phantom.app/)
5. Solana Tool Suite
* A CLI developed for interacting with the Solana Network/Blockchain
* [Documentation](https://docs.solana.com/cli/install-solana-cli-tools)
6. Solana/Web3.js
* A collection of libraries which enable developers to interact with local or remote Solana nodes using HTTP, IPC, or WebSockets. Based off the [ETH Web3.js library](https://web3js.readthedocs.io/en/v1.5.2/).
* [Documentation](https://solana-labs.github.io/solana-web3.js/)

## Installation Notes (Ubuntu)
1. Install `nvm`

```
$ sudo apt install curl
```

```
$ curl https://raw.githubusercontent.com/creationix/nvm/master/install.sh | bash
```

```
$ source ~/.profile  // loads environment entry the nvm install script created
```

2. Install `node`

```
$ nvm install node
```

3. Install `Solana Tool Suite`

```
$ sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

```
$ export PATH="/home/pathfinder/.local/share/solana/install/active_release/bin:$PATH"  // this was prompted by the script
```

```
$ solana --version  // verify install was successful
```

4. Install `Rust`

```
$ curl https://sh.rustup.rs -sSf | sh
```

```
$ source $HOME/.cargo/env  // this was prompted by the script - Cargo is the package manager and crate host for Rust
```

```
$ rustc --version  // verify install was successful
```

```
$ sudo apt-get install build-essential -y  // dependencies required by rust cmd
```

5. Install `Yarn`

```
$ npm install -g yarn
```

6. Install `Anchor`

```
$ npm i -g @project-serum/anchor-cli
```

```
$ anchor --version  // verify install was successful
```

## Build Notes

1. How to use Phantom Wallet Address with Solana CLI

```
// First make sure your CLI and Wallet are both configured to run on the `devnet`
$ solana config set --url devnet  // for shell, wallet is configured in Phantom UI Settings > Change Network
$ solana-keygen recover 'prompt:?key=0/0' --outfile ~/.config/solana/id.json
> this will prompt you to input your seed phrase and passphrase (if configured)

// cmds to test connection bw shell and wallet
$ solana balance 
$ solana-keygen pubkey  // your wallet address
$ solana-keygen verify <wallet_address>
```

2. Solana Tool Suite - Localhost Setup

```
// In Phantom UI, Settings > Change Network > Localhost
$ solana config set --url localhost

// Next, start local Solana Node for testing
// This will create a dir in your project called `test-ledger`
$ solana-test-validator

// Test by doing an airdrop to your wallet
$ solana airdrop 100
```

3. Anchor Concepts and Framework Structure
* `app` dir > this is where frontend code lives
* `programs` dir > this is where Rust code lives
* `test` dir > frontend tests go here
* `migrations` dir > basic deploy script
* `Provider` > refers to an abstraction which represents a client's connection to the Solana Network
    * A valid connection will consist of: a `Connection` to a fullnode JSON RPC endpoint, a `Wallet`, and a `preflight commitment`.
* `program` > an abstraction made up of a `Provider`, `IDL`, and `programID` combination
    * enables us to make RPC calls against the Anchor program
* `account` > There is no persisted state within a Solana program. All state-related data is attached to an `account`. This means all data is referenced from external sources.

4. Anchor's eDSL (Embedded Domain Specific Language)
* a *domain specific language* can be attributed as a tiny programming language used for a specific task. Things like Python, Golang, and C are considered to be *general-purpose languages*.
* Examples of DSLs
    * CSS > DSL used for styling-related rules HTML properties on websites
    * SQL > DSL for querying databases
    * Regular Expressions > used to specify patterns to match strings

5. Anchor Commands

```
// Compile your project and create and executable
// This will create a new dir called `target`
$ anchor build
```

```
// run tests
$ anchor test
```

6. `target` directory
* contains an IDL so other programs/tests can run your program via RPC
* contains a `deploy` dir; this dir contains the binary you'll use to deploy your program on the Solana network via `$ solana program deploy <abs_path_to_sol_binary>`
