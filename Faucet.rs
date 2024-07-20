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
