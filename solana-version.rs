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
