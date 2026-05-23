use soroban_sdk::{contract, contractimpl, Address, Env, String};

use crate::{
    error::ContractError,
    events::{Approval, Burn, Transfer},
    storage::{AllowanceKey, DataKey},
};

#[contract]
pub struct SibToken;

#[contractimpl]
impl SibToken {
    pub fn initialize(
        env: Env,
        admin: Address,
        initial_supply: i128,
    ) -> Result<(), ContractError> {
        env.storage().persistent().set(&DataKey::Admin, &admin);
        env.storage()
            .persistent()
            .set(&DataKey::TotalSupply, &initial_supply);
        env.storage()
            .persistent()
            .set(&DataKey::Balance(admin), &initial_supply);
        Ok(())
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Allowance(AllowanceKey { from, spender }))
            .unwrap_or(0)
    }

    pub fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        live_until_ledger: u32,
    ) -> Result<(), ContractError> {
        from.require_auth();

        let key = DataKey::Allowance(AllowanceKey {
            from: from.clone(),
            spender: spender.clone(),
        });

        env.storage().persistent().set(&key, &amount);

        Approval {
            from,
            spender,
            amount,
            live_until_ledger,
        }
        .publish(&env);

        Ok(())
    }

    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        from.require_auth();

        let sender_balance = Self::balance(env.clone(), from.clone());
        let receiver_balance = Self::balance(env.clone(), to.clone());

        if sender_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(sender_balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(receiver_balance + amount));

        Transfer { from, to, amount }.publish(&env);

        Ok(())
    }

    pub fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        spender.require_auth();

        let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        if allowance < amount {
            return Err(ContractError::InsufficientAllowance);
        }

        let sender_balance = Self::balance(env.clone(), from.clone());
        if sender_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let allowance_key = DataKey::Allowance(AllowanceKey {
            from: from.clone(),
            spender,
        });
        env.storage()
            .persistent()
            .set(&allowance_key, &(allowance - amount));

        let receiver_balance = Self::balance(env.clone(), to.clone());
        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(sender_balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(receiver_balance + amount));

        Transfer { from, to, amount }.publish(&env);

        Ok(())
    }

    pub fn burn(env: Env, from: Address, amount: i128) -> Result<(), ContractError> {
        from.require_auth();

        let balance = Self::balance(env.clone(), from.clone());
        if balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(balance - amount));

        let total_supply: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::TotalSupply, &(total_supply - amount));

        Burn { from, amount }.publish(&env);

        Ok(())
    }

    pub fn burn_from(
        env: Env,
        spender: Address,
        from: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        spender.require_auth();

        let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        if allowance < amount {
            return Err(ContractError::InsufficientAllowance);
        }

        let balance = Self::balance(env.clone(), from.clone());
        if balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let allowance_key = DataKey::Allowance(AllowanceKey {
            from: from.clone(),
            spender,
        });
        env.storage()
            .persistent()
            .set(&allowance_key, &(allowance - amount));

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(balance - amount));

        let total_supply: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::TotalSupply, &(total_supply - amount));

        Burn { from, amount }.publish(&env);

        Ok(())
    }

    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), ContractError> {
        let admin: Option<Address> = env.storage().persistent().get(&DataKey::Admin);
        match admin {
            None => return Err(ContractError::Unauthorized),
            Some(admin_addr) => admin_addr.require_auth(),
        }

        let balance = Self::balance(env.clone(), to.clone());
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(balance + amount));

        let total_supply: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::TotalSupply, &(total_supply + amount));

        Transfer {
            from: env.current_contract_address(),
            to,
            amount,
        }
        .publish(&env);

        Ok(())
    }

    pub fn decimals(_env: Env) -> u32 {
        18
    }

    pub fn name(env: Env) -> String {
        String::from_str(&env, "SibToken")
    }

    pub fn symbol(env: Env) -> String {
        String::from_str(&env, "SIB")
    }
}
