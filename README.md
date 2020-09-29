Advanced Fungible Token for NEAR blockchain.

Part of Rainbow hackathon: https://gitcoin.co/hackathon/hack-the-rainbow/projects/1497/Advanced-Fungible-Token-Standard-NEP-9000

# Run

```sh
brew install rustup
rustup update
cd contracts
cargo build
cargo test
```

## Visual Studio Code

Install Rust extension. Choose *Start Rust Server* from the command palette.

# Walkthrough

1. User calls send()
2. Update token balance
3. Notify contract
4. Contract update internal balance

5. Reduce internal balance
6. Contract call send()
7. Notify another contract

# Test cases

Tests are written to be run on the local NEAR node because simulation tests did not support [cross contract calls](https://github.com/near-examples/simulation-testing/blob/master/tests/simulation_tests.rs#L189).

## Running tests

You need neacore, nearup.

```sh
git clone git@github.com:nearprotocol/nearcore.git
cd nearcore
make release
```

Then nearup:

```sh
python3 -m venv venv
source venv/bin/activate
pip install nearup
```

Create a local net:

```sh
# Use your nearcore build destination
nearup run localnet --binary-path ~/code/nearcore/target/release
```

Test that baseline tests work for the JS core package:

```sh
git clone git@github.com:near/near-api-js.git
export NEAR_ENV=ci
yarn test
```

Build contracts

```sh
cd contract
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

Run our own tests:

```sh
yarn install
yarn build:contract

```

##

We use `Escrow` as the name on the contract that receives token transfer in the step one.

## Two parallel transfer

- The user initiates two transactions that both trigger `on_token_transfer()` to escrow contract

-> Not parallerised because in-shard transactions cannot go parallel.

## Escrow forwards tokens immediately

- The user initiates transactions that triggers `on_token_transfer()` on the escrow contract triggers another `on_token_transfer()` to a third contract

## Escrow forward to another contract that immediately wants to move tokens to


1. User moves tokens to Escrow
-> Call token.send()
-> trigges on_token_received
2. Escrow calls fee contract
-> Receive Escrow.on_token_received
-> Call token.send()
-> trigger Fee.on_token_received
3. Transaction finalized

User balance: 20
Escrow balance: 7
Fee balance: 3

1. User moves tokens to Escrow
-> Call token.send()
-> trigges on_token_received
2. Escrow calls fee contract
-> Receive Escrow.on_token_received
-> Call token.send()
-> trigger Fee.on_token_received
3. Fee contract wants to move tokens from Escrow contract to another user account

User balance: 30
Escrow balance: user/7 (pending)
Fee balance: user/3 (pending)

-> Call escrow.pay_dividend(user)
-> User
-> Call token.send()

Hostile vector

- The user initiates transactions that triggers `on_token_transfer()` on the escrow contract triggers another `on_token_transfer()` to a hostile contract that tries to steal the balacne

## Complex cases


# Notes

- A specialised `AccountLookupMap` type is needed that would allow easily to use blockchain explorer to see token account balances.
  Currently accounts are compressed to sha256 and it makes hard to analyse the data.

- Simulation tests way too hard to write at the moment - 200 lines of utils.rs needed for a simple test.
  Also unit tests do not work for any real system as any non-trivial system has more than two contracts,
  making unit tests next to useless if they cannot test cross-contract calls.

- Simulation tests cannot cover all NEAR blockchain use cases, so they are also not useful.

- create-near-app knows no ABI files, but all contract methods need to be written by hand https://github.com/near/create-near-app/blob/master/templates/react/src/main.test.js#L5

- Near JS API does not document how to interact with contracts https://near.github.io/near-api-js/modules/_contract_.html

- Manually specifying gas amounts in contract calls is cumbersome https://github.com/near/core-contracts/blob/master/lockup/src/owner_callbacks.rs#L330

- Missing cross contract interfaces and importing contracts in another source code (Rust)

- ```error: the `#[global_allocator]` in this crate conflicts with global allocator in: nep9000_token```

# Further reading

https://github.com/near/near-sdk-rs/blob/master/examples/fungible-token/src/lib.rs

https://github.com/near/core-contracts/tree/master/lockup/src

https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html

https://github.com/near-examples/simulation-testing

https://github.com/near-examples/guest-book/tree/master

https://github.com/smartcontractkit/near-protocol-contracts