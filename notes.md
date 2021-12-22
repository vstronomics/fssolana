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
