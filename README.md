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

# SOLANA version


### Enhancing the Smart Contract for Solana

To transition this smart contract to the Solana network and add more features, we can use the `Anchor` framework. Below are some suggested features and the updated code for Solana.

### Suggested Features:
1. **Incentive System for Depositors**:
   - Implement a mechanism to reward users who deposit Ether into the contract.
2. **Whitelist for Authorized Withdrawals**:
   - Add a whitelist for addresses that are allowed to withdraw from the contract.
3. **Graphical Transaction Reporting**:
   - Use stored data to display graphical reports of transactions in the UI.
4. **Integration with Solana Wallets**:
   - Ensure compatibility with popular Solana wallets like Phantom.

### Transition to Solana:
To transition this contract to the Solana network, we will use the `Anchor` framework, a framework for developing smart contracts on Solana using Rust. Below is the code for a similar contract on Solana:

```rust
use anchor_lang::prelude::*;

declare_id!("YourProgramID");

#[program]
pub mod faucet {
    use super::*;
    use std::collections::HashMap;

    #[state]
    pub struct Faucet {
        owner: Pubkey,
        timeouts: HashMap<Pubkey, i64>,
        withdrawal_limit: u64,
        emergency_stop: bool,
        max_balance: u64,
        transactions: Vec<Transaction>,
    }

    #[account]
    pub struct Transaction {
        from: Pubkey,
        to: Pubkey,
        amount: u64,
        timestamp: i64,
    }

    impl Faucet {
        pub fn new(ctx: Context<Initialize>, withdrawal_limit: u64, max_balance: u64) -> Result<Self> {
            let owner = *ctx.accounts.owner.key;
            Ok(Self {
                owner,
                timeouts: HashMap::new(),
                withdrawal_limit,
                emergency_stop: false,
                max_balance,
                transactions: Vec::new(),
            })
        }

        pub fn withdraw(&mut self, ctx: Context<Withdraw>) -> Result<()> {
            let caller = *ctx.accounts.caller.key;
            let clock = Clock::get()?;
            let balance = ctx.accounts.faucet.to_account_info().lamports();

            require!(!self.emergency_stop, FaucetError::WithdrawalsDisabled);
            require!(balance >= self.withdrawal_limit, FaucetError::InsufficientFunds);
            require!(self.timeouts.get(&caller).unwrap_or(&0) <= &(clock.unix_timestamp - 1800), FaucetError::TimeoutNotReached);

            **ctx.accounts.caller.try_borrow_mut_lamports()? += self.withdrawal_limit;
            **ctx.accounts.faucet.to_account_info().try_borrow_mut_lamports()? -= self.withdrawal_limit;

            self.timeouts.insert(caller, clock.unix_timestamp);
            self.transactions.push(Transaction {
                from: *ctx.accounts.faucet.key,
                to: caller,
                amount: self.withdrawal_limit,
                timestamp: clock.unix_timestamp,
            });

            Ok(())
        }

        pub fn deposit(&mut self, ctx: Context<Deposit>, amount: u64) -> Result<()> {
            let caller = *ctx.accounts.caller.key;
            let balance = ctx.accounts.faucet.to_account_info().lamports() + amount;

            require!(balance <= self.max_balance, FaucetError::MaxBalanceExceeded);

            **ctx.accounts.faucet.to_account_info().try_borrow_mut_lamports()? += amount;

            self.transactions.push(Transaction {
                from: caller,
                to: *ctx.accounts.faucet.key,
                amount,
                timestamp: Clock::get()?.unix_timestamp,
            });

            Ok(())
        }

        pub fn set_withdrawal_limit(&mut self, ctx: Context<SetLimit>, new_limit: u64) -> Result<()> {
            require!(ctx.accounts.owner.key == self.owner, FaucetError::Unauthorized);
            self.withdrawal_limit = new_limit;
            Ok(())
        }

        pub fn set_max_balance(&mut self, ctx: Context<SetLimit>, new_max_balance: u64) -> Result<()> {
            require!(ctx.accounts.owner.key == self.owner, FaucetError::Unauthorized);
            self.max_balance = new_max_balance;
            Ok(())
        }

        pub fn toggle_emergency_stop(&mut self, ctx: Context<ToggleEmergency>) -> Result<()> {
            require!(ctx.accounts.owner.key == self.owner, FaucetError::Unauthorized);
            self.emergency_stop = !self.emergency_stop;
            Ok(())
        }

        pub fn transfer_ownership(&mut self, ctx: Context<TransferOwnership>, new_owner: Pubkey) -> Result<()> {
            require!(ctx.accounts.owner.key == self.owner, FaucetError::Unauthorized);
            self.owner = new_owner;
            Ok(())
        }

        pub fn get_balance(&self, ctx: Context<GetBalance>) -> Result<u64> {
            Ok(ctx.accounts.faucet.to_account_info().lamports())
        }

        pub fn get_time_until_next_withdrawal(&self, ctx: Context<GetTimeUntilNextWithdrawal>, account: Pubkey) -> Result<i64> {
            let current_time = Clock::get()?.unix_timestamp;
            let last_withdrawal = self.timeouts.get(&account).unwrap_or(&0);
            if *last_withdrawal == 0 || current_time >= last_withdrawal + 1800 {
                Ok(0)
            } else {
                Ok((last_withdrawal + 1800) - current_time)
            }
        }

        pub fn get_transactions(&self, _ctx: Context<GetTransactions>) -> Result<Vec<Transaction>> {
            Ok(self.transactions.clone())
        }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub caller: AccountInfo<'info>,
    #[account(mut)]
    pub faucet: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(signer)]
    pub caller: AccountInfo<'info>,
    #[account(mut)]
    pub faucet: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetLimit<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ToggleEmergency<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(signer)]
    pub owner: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GetBalance<'info> {
    pub faucet: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GetTimeUntilNextWithdrawal<'info> {}

#[derive(Accounts)]
pub struct GetTransactions<'info> {}

#[error]
pub enum FaucetError {
    #[msg("Withdrawals are currently disabled.")]
    WithdrawalsDisabled,
    #[msg("Insufficient funds in the faucet.")]
    InsufficientFunds,
    #[msg("Timeout for next withdrawal not reached.")]
    TimeoutNotReached,
    #[msg("Unauthorized action.")]
    Unauthorized,
    #[msg("Maximum balance exceeded.")]
    MaxBalanceExceeded,
}
```

### Description of the Enhancements:

1. **Incentive System for Depositors**:
   Implement a reward system for users who deposit funds into the contract.
2. **Whitelist for Authorized Withdrawals**:
   Add a whitelist to allow only specific addresses to withdraw from the contract.
3. **Graphical Transaction Reporting**:
   Use stored transaction data to display graphical reports in the UI.
4. **Integration with Solana Wallets**:
   Ensure compatibility with popular Solana wallets like Phantom.

### Installing Solana and Anchor Tools:

1. **Install Anchor**:
   ```sh
   cargo install --git https://github.com/project-serum/anchor --tag v0.19.0 anchor-cli --locked
   ```

2. **Create a New Anchor Project**:
   ```sh
   anchor init faucet
   ```

3. **Replace the Generated Contract Code**:
   Replace the generated code with the provided code above.

4. **Build and Deploy the Contract**:
   ```sh
   anchor build
   anchor deploy
   ```

These steps will help you install the `Anchor` tools, create a new project, and deploy your smart contract on the Solana network. 
