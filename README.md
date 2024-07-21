# Ethereum Faucet

This is an Ethereum Faucet for the Ropsten Testnet. It allows users to request and donate Ether. The faucet ensures that users can receive a small amount of Ether for testing purposes on the Ropsten network, and it also accepts donations to keep the faucet funded for future users.

## Smart Contract in Rust (using ink!)

### Installing ink! Tools

To install the `ink!` tools, follow these steps. This includes installing Rust and the necessary tools for developing smart contracts using `ink!`.

1. **Install Rust**:
   First, install Rust using `rustup`:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Add the `wasm32-unknown-unknown` Target**:
   Add the WebAssembly target required for building `ink!` smart contracts:
   ```sh
   rustup target add wasm32-unknown-unknown --toolchain nightly
   ```

3. **Install the `cargo-contract` Tool**:
   Install the `cargo-contract` tool, which is used for building and deploying smart contracts:
   ```sh
   cargo install cargo-contract --force
   ```

4. **Install Substrate**:
   If you haven't already installed Substrate, you can do so using the following command:
   ```sh
   curl https://getsubstrate.io -sSf | bash
   ```

### Creating an ink! Project

After installing the `ink!` tools, follow these steps to create and deploy a smart contract:

1. **Create a New `ink!` Project**:
   ```sh
   cargo contract new faucet
   ```

2. **Replace the Generated Contract Code**:
   Replace the generated code with the provided Rust code from the previous responses.

3. **Build and Deploy the Contract**:
   Use the following commands to build and deploy the contract:
   ```sh
   cargo contract build
   cargo contract deploy --constructor new 100000000000000000 1000000000000000000
   ```

These steps will help you install the `ink!` tools, create a new project, and deploy your smart contract. If you need further assistance, feel free to ask!

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

# How to Use

1. Open `index.html` in a web browser.
2. Ensure Metamask or another Web3 provider is configured and connected to the Ropsten Testnet.
3. Use the "Request Ether" and "Donate Ether" buttons to interact with the smart contract.


# version 0.002
Here's an enhanced version of the given Rust code for the Ethereum Faucet using the `ink!` framework. The improvements include additional functionality such as setting withdrawal limits, checking the contract balance, and adding an emergency stop mechanism.

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
        withdrawal_limit: Balance,
        emergency_stop: bool,
    }

    #[ink(event)]
    pub struct Withdrawal {
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        amount: Balance,
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
        pub fn new(withdrawal_limit: Balance) -> Self {
            Self {
                owner: Self::env().caller(),
                timeouts: BTreeMap::new(),
                withdrawal_limit,
                emergency_stop: false,
            }
        }

        #[ink(message)]
        pub fn withdraw(&mut self) {
            let caller = self.env().caller();
            let balance = self.env().balance();
            let timestamp = self.env().block_timestamp();

            assert!(!self.emergency_stop, "Withdrawals are currently disabled.");
            assert!(balance >= self.withdrawal_limit, "This faucet is empty. Please check back later.");
            assert!(self.timeouts.get(&caller).unwrap_or(&0) <= &(timestamp - 1800_000), "You can only withdraw once every 30 minutes. Please check back later.");

            self.env().transfer(caller, self.withdrawal_limit).expect("Transfer failed");
            self.timeouts.insert(caller, timestamp);

            self.env().emit_event(Withdrawal { to: caller, amount: self.withdrawal_limit });
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) {
            let caller = self.env().caller();
            let amount = self.env().transferred_balance();

            self.env().emit_event(Deposit { from: caller, amount });
        }

        #[ink(message)]
        pub fn set_withdrawal_limit(&mut self, new_limit: Balance) {
            assert_eq!(self.env().caller(), self.owner, "Only the owner can set the withdrawal limit.");
            self.withdrawal_limit = new_limit;
        }

        #[ink(message)]
        pub fn toggle_emergency_stop(&mut self) {
            assert_eq!(self.env().caller(), self.owner, "Only the owner can toggle the emergency stop.");
            self.emergency_stop = !self.emergency_stop;
        }

        #[ink(message)]
        pub fn destroy(&mut self) {
            let caller = self.env().caller();
            assert_eq!(caller, self.owner, "Only the owner of this faucet can destroy it.");

            self.env().terminate_contract(self.owner);
        }

        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            self.env().balance()
        }
    }

    impl Default for Faucet {
        fn default() -> Self {
            Self::new(100_000_000_000_000_000) // Default withdrawal limit set to 0.1 ETH
        }
    }
}
```

### Enhancements and New Features

1. **Withdrawal Limit**: Added a constructor parameter and a function to set a withdrawal limit.
2. **Emergency Stop**: Added an emergency stop mechanism to enable/disable withdrawals.
3. **Get Balance**: Added a function to get the current balance of the contract.
4. **Event Enhancement**: Enhanced the `Withdrawal` event to include the withdrawn amount.
5. **Constructor Initialization**: The constructor now initializes the withdrawal limit and the emergency stop flag.

### Usage Guide

1. **Install the `ink!` toolchain**:
   Follow the instructions from the [ink! documentation](https://paritytech.github.io/ink-docs/getting-started).

2. **Create a new `ink!` project**:
   ```sh
   cargo contract new faucet
   ```

3. **Replace the generated contract code with the provided Rust code**.

4. **Compile and deploy the contract**:
   ```sh
   cargo contract build
   cargo contract deploy --constructor new 100000000000000000
   ```

5. **Interact with the contract**:
   - **Withdraw**: Call the `withdraw` function to withdraw Ether.
   - **Deposit**: Send Ether to the contract to deposit.
   - **Set Withdrawal Limit**: Call the `set_withdrawal_limit` function to change the limit.
   - **Toggle Emergency Stop**: Call the `toggle_emergency_stop` function to enable/disable withdrawals.
   - **Get Balance**: Call the `get_balance` function to check the contract's balance.
   - **Destroy**: Call the `destroy` function to destroy the contract and send remaining funds to the owner.

Feel free to ask if you need any further assistance!
