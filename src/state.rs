use linera_sdk::{
    views::{MapView, RegisterView, ViewStorageContext},
    base::{Amount, Owner},
};
use linera_views::views::{GraphQLView, RootView};
use thiserror::Error;

#[derive(RootView, GraphQLView)]
#[view(context = "ViewStorageContext")]
pub struct CryptoFinance {
    // Implement views for application state here (e.g., balances, transactions, user wallets, etc.)
    pub balances: MapView<Owner, Amount>,
}

impl CryptoFinance {
    // Implement functions to interact with the application state (e.g., deposit, withdraw, transfer, etc.)

    // Function to deposit funds to a user's balance
    pub async fn deposit(&mut self, owner: Owner, amount: Amount) {
        let current_balance = self.balances.get(owner).await.unwrap_or_default();
        self.balances.insert(owner, current_balance + amount).await;
    }

    // Function to withdraw funds from a user's balance
    pub async fn withdraw(&mut self, owner: Owner, amount: Amount) -> Result<(), WithdrawError> {
        let current_balance = self.balances.get(owner).await.unwrap_or_default();

        if current_balance < amount {
            return Err(WithdrawError::InsufficientBalance);
        }

        self.balances.insert(owner, current_balance - amount).await;
        Ok(())
    }

    // Function to transfer funds from one user to another
    pub async fn transfer(
        &mut self,
        from: Owner,
        to: Owner,
        amount: Amount,
    ) -> Result<(), TransferError> {
        let from_balance = self.balances.get(from).await.unwrap_or_default();

        if from_balance < amount {
            return Err(TransferError::InsufficientBalance);
        }

        self.balances.insert(from, from_balance - amount).await;
        let to_balance = self.balances.get(to).await.unwrap_or_default();
        self.balances.insert(to, to_balance + amount).await;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum WithdrawError {
    #[error("Insufficient balance")]
    InsufficientBalance,
}

#[derive(Debug, Error)]
pub enum TransferError {
    #[error("Insufficient balance")]
    InsufficientBalance,
}


