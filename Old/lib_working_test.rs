#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod meme_token {
    use ink::storage::Mapping;
    use scale::{Encode, Decode};
    use scale_info::TypeInfo;

    #[ink(storage)]
    pub struct MemeToken {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        owner: AccountId,
    }

    #[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
    pub enum TransferError {
        InsufficientBalance,
    }

    #[derive(Debug, PartialEq, Eq, Encode, Decode, TypeInfo)]
    pub enum MintError {
        NotOwner,
    }

    impl MemeToken {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = Mapping::default();
            balances.insert(caller, &initial_supply);
            Self {
                total_supply: initial_supply,
                balances,
                owner: caller,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(0)
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> Balance {
            self.balances.get(account).unwrap_or(0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), TransferError> {
            let from = self.env().caller();
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(TransferError::InsufficientBalance);
            }
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            Ok(())
        }

        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, amount: Balance) -> Result<(), MintError> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(MintError::NotOwner);
            }
            self.total_supply += amount;
            let current_balance = self.balance_of(to);
            self.balances.insert(to, &(current_balance + amount));
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;

        #[ink::test]
        fn total_supply_works() {
            let contract = MemeToken::new(100);
            assert_eq!(contract.total_supply(), 100);
        }

        #[ink::test]
        fn balance_of_works() {
            let contract = MemeToken::new(100);
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(contract.balance_of(accounts.alice), 100);
            assert_eq!(contract.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = MemeToken::new(100);
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(contract.transfer(accounts.bob, 10), Ok(()));
            assert_eq!(contract.balance_of(accounts.bob), 10);
            assert_eq!(contract.balance_of(accounts.alice), 90);
        }

        #[ink::test]
        fn transfer_insufficient_balance_fails() {
            let mut contract = MemeToken::new(50);
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            let result = contract.transfer(accounts.bob, 100);
            assert_eq!(result, Err(TransferError::InsufficientBalance));
            assert_eq!(contract.balance_of(accounts.bob), 0);
            assert_eq!(contract.balance_of(accounts.alice), 50);
        }

        #[ink::test]
        fn only_owner_can_mint() {
            let mut contract = MemeToken::new(0);
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(contract.mint(accounts.bob, 42), Ok(()));
            assert_eq!(contract.total_supply(), 42);
            assert_eq!(contract.balance_of(accounts.bob), 42);

            test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            let result = contract.mint(accounts.bob, 10);
            assert_eq!(result, Err(MintError::NotOwner));
        }
    }
}
