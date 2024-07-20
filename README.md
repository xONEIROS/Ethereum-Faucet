# Ethereum Faucet

This is an Ethereum Faucet for the Ropsten Testnet. It allows users to request and donate Ether. The faucet ensures that users can receive a small amount of Ether for testing purposes on the Ropsten network, and it also accepts donations to keep the faucet funded for future users.

## Smart Contract in Rust (using ink!)

The smart contract has been converted from Solidity to Rust using the `ink!` framework.

### Rust Contract

```rust
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod faucet {
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::collections::BTreeMap;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Faucet {
        owner: AccountId,
        timeouts: BTreeMap<AccountId, u64>,
    }

    #[ink(event)]
    pub struct Withdrawal {
        #[ink(topic)]
        to: AccountId,
    }

    #[ink(event)]
    pub struct Deposit {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        amount: Balance,
    }

    impl Faucet {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn withdraw(&mut self) {
            let caller = self.env().caller();
            let balance = self.env().balance();
            let timestamp = self.env().block_timestamp();

            assert!(balance >= 100_000_000_000_000_000, "This faucet is empty. Please check back later.");
            assert!(self.timeouts.get(&caller).unwrap_or(&0) <= &(timestamp - 1800_000), "You can only withdraw once every 30 minutes. Please check back later.");

            self.env().transfer(caller, 100_000_000_000_000_000).expect("Transfer failed");
            self.timeouts.insert(caller, timestamp);

            self.env().emit_event(Withdrawal { to: caller });
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) {
            let caller = self.env().caller();
            let amount = self.env().transferred_balance();

            self.env().emit_event(Deposit { from: caller, amount });
        }

        #[ink(message)]
        pub fn destroy(&mut self) {
            let caller = self.env().caller();
            assert_eq!(caller, self.owner, "Only the owner of this faucet can destroy it.");

            self.env().terminate_contract(self.owner);
        }
    }

    impl Default for Faucet {
        fn default() -> Self {
            Self::allocate_spread()
        }
    }
}

```

### Usage Guide

1. **Install the `ink!` toolchain:**
   Follow the instructions from the [ink! documentation](https://paritytech.github.io/ink-docs/getting-started).

2. **Create a new `ink!` project:**
   ```sh
   cargo contract new faucet
   ```

3. **Replace the generated contract code with the provided Rust code.**

4. **Compile and deploy the contract:**
   ```sh
   cargo contract build
   cargo contract deploy
   ```

## HTML Frontend

The HTML frontend has been updated to improve its appearance and functionality.

### Features

- Users can request Ether from the faucet.
- Users can donate Ether to the faucet.
- Displays the current balance of the faucet.
- Provides feedback on transaction status.

### How to Use

1. Open `index.html` in a web browser.
2. Ensure Metamask or another Web3 provider is configured and connected to the Ropsten Testnet.
3. Use the "Request Ether" and "Donate Ether" buttons to interact with the smart contract.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
