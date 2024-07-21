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
