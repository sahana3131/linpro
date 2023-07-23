use async_trait::async_trait;
use linera_sdk::{
    base::{Amount, Owner},
    contract::system_api,
    ApplicationCallResult, CalleeContext, Contract, ExecutionResult, MessageContext,
    OperationContext, SessionCallResult, ViewStateStorage,
};
use thiserror::Error;

linera_sdk::contract!(CryptoFinance);

impl WithContractAbi for CryptoFinance {
    type Abi = crypto_finance::CryptoFinanceAbi;
}

#[async_trait]
impl Contract for CryptoFinance {
    type Error = ContractError;
    type Storage = ViewStateStorage<Self>;

    async fn initialize(
        &mut self,
        _context: &OperationContext,
        // Add any initialization arguments here, if needed
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        // Add initialization logic here, if needed
        Ok(ExecutionResult::default())
    }

    async fn execute_operation(
        &mut self,
        context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        // Add contract operation logic here based on the provided operation variant
        match operation {
            // Implement operation handling here, e.g., deposit, withdraw, transfer, etc.
            // Example:
            Operation::Deposit { owner, amount } => {
                self.deposit(owner, amount).await?;
            }
            Operation::Withdraw { owner, amount } => {
                self.withdraw(owner, amount).await?;
            }
        }

        // Return the default execution result, as we don't use messages in this example
        Ok(ExecutionResult::default())
    }

    async fn execute_message(
        &mut self,
        _context: &MessageContext,
        _message: Self::Message,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        // Messages are not used in this example, so just return the default execution result
        Ok(ExecutionResult::default())
    }

    async fn handle_application_call(
        &mut self,
        _context: &CalleeContext,
        _call: Self::ApplicationCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<ApplicationCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {
        // Application calls are not used in this example, so just return the default application call result
        Ok(ApplicationCallResult::default())
    }

    async fn handle_session_call(
        &mut self,
        _context: &CalleeContext,
        _session: Self::SessionState,
        _call: Self::SessionCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<SessionCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {
        // Session calls are not supported in this example, so return an error indicating that
        Err(ContractError::SessionsNotSupported)
    }
}

/// An error that can occur during the contract execution.
#[derive(Debug, Error)]
pub enum ContractError {
    /// Failed to deserialize BCS bytes
    #[error("Failed to deserialize BCS bytes")]
    BcsError(#[from] bcs::Error),

    /// Failed to deserialize JSON string
    #[error("Failed to deserialize JSON string")]
    JsonError(#[from] serde_json::Error),

    // Add more error variants here as needed
    // Example:
    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Sessions not supported")]
    SessionsNotSupported,
}

impl CryptoFinance {
    // Implement contract functions here, e.g., deposit, withdraw, transfer, etc.
    // Add your application-specific logic based on the contract operations.
    // Example:

    async fn deposit(&mut self, owner: Owner, amount: Amount) -> Result<(), ContractError> {
        // Implement your deposit logic here, e.g., update balances, emit events, etc.
        // Example:
        let current_balance = self.balances.get(&owner).unwrap_or_default();
        self.balances.insert(&owner, current_balance + amount);

        Ok(())
    }

    async fn withdraw(&mut self, owner: Owner, amount: Amount) -> Result<(), ContractError> {
        // Implement your withdraw logic here, e.g., check balance, update balances, emit events, etc.
        // Example:
        let current_balance = self.balances.get(&owner).unwrap_or_default();
        if current_balance >= amount {
            self.balances.insert(&owner, current_balance - amount);
            Ok(())
        } else {
            Err(ContractError::InsufficientBalance)
        }
    }

}