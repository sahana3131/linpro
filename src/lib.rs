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
        _initial_state: InitialState,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {

        Ok(ExecutionResult::default())
    }

    async fn execute_operation(
        &mut self,
        context: &OperationContext,
        operation: Self::Operation,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        match operation {
            Operation::Transfer {
                owner,
                amount,
                target_account,
            } => {
                self.transfer(context, owner, amount, target_account)
                    .await?;
                Ok(ExecutionResult::default())
            }
        }
    }

    async fn execute_message(
        &mut self,
        _context: &MessageContext,
        _message: Self::Message,
    ) -> Result<ExecutionResult<Self::Message>, Self::Error> {
        Ok(ExecutionResult::default())
    }

    async fn handle_application_call(
        &mut self,
        _context: &CalleeContext,
        _call: Self::ApplicationCall,
        _forwarded_sessions: Vec<SessionId>,
    ) -> Result<ApplicationCallResult<Self::Message, Self::Response, Self::SessionState>, Self::Error>
    {

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

        Err(ContractError::SessionsNotSupported)
    }
}

impl CryptoFinance {
    async fn transfer(
        &mut self,
        context: &OperationContext,
        from: Owner,
        amount: Amount,
        target_account: Account,
    ) -> Result<(), ContractError> {
        Self::check_account_authentication(context.authenticated_signer, from)?;

        let target_account_owner = target_account.owner;
        let target_account_chain_id = target_account.chain_id;

        // If the target account is on the same chain, directly credit the amount
        if target_account_chain_id == system_api::current_chain_id() {
            self.credit(target_account_owner, amount).await;
        } else {
            // If the target account is on a different chain, send a message to credit the amount
            let message = Message::Credit {
                owner: target_account_owner,
                amount,
            };
            self.send_message(target_account_chain_id, message).await?;
        }

        Ok(())
    }

    fn check_account_authentication(
        authenticated_signed: Option<Owner>,
        owner: Owner,
    ) -> Result<(), ContractError> {
        if authenticated_signed == Some(owner) {
            Ok(())
        } else {
            Err(ContractError::IncorrectAuthentication)
        }
    }
}

#[derive(Debug, Error)]
pub enum ContractError {
    /// Failed to deserialize BCS bytes
    #[error("Failed to deserialize BCS bytes")]
    BcsError(#[from] bcs::Error),

    /// Failed to deserialize JSON string
    #[error("Failed to deserialize JSON string")]
    JsonError(#[from] serde_json::Error),

    #[error("Incorrect Authentication")]
    IncorrectAuthentication,

    #[error("Sessions not supported")]
    SessionsNotSupported,
}

#[derive(Debug)]
pub struct InitialState {

}

#[derive(Debug)]
pub struct Account {
    owner: Owner,
    chain_id: ChainId,
}

