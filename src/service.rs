use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use async_trait::async_trait;
use linera_sdk::base::{Amount, Owner};
use linera_sdk::{base::WithServiceAbi, QueryContext, Service, ViewStateStorage};
use std::sync::Arc;
use thiserror::Error;

linera_sdk::service!(CryptoFinanceService);

impl WithServiceAbi for CryptoFinanceService {
    type Abi = crypto_finance::CryptoFinanceAbi;
}

#[async_trait]
impl Service for CryptoFinanceService {
    type Error = ServiceError;
    type Storage = ViewStateStorage<Self>;

    async fn query_application(
        self: Arc<Self>,
        _context: &QueryContext,
        request: Request,
    ) -> Result<Response, Self::Error> {
        let schema = Schema::build(self.clone(), MutationRoot {}, EmptySubscription).finish();
        let response = schema.execute(request).await;
        Ok(response)
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn deposit(&self, owner: Owner, amount: Amount) -> DepositResult {
        // You can implement your deposit logic here
        // For example, call the deposit function of the CryptoFinance struct
        // and handle any errors that may occur during the deposit operation
        // Return a DepositResult indicating whether the deposit was successful or not.
        // For simplicity, I'm returning a boolean indicating success or failure in this example.

        let success = true; // Replace this with your actual deposit logic

        DepositResult { success }
    }

    async fn withdraw(&self, owner: Owner, amount: Amount) -> WithdrawResult {
        // You can implement your withdraw logic here
        // For example, call the withdraw function of the CryptoFinance struct
        // and handle any errors that may occur during the withdraw operation
        // Return a WithdrawResult indicating whether the withdraw was successful or not.
        // For simplicity, I'm returning a boolean indicating success or failure in this example.

        let success = true; // Replace this with your actual withdraw logic

        WithdrawResult { success }
    }

    // You can define other mutation operations for your crypto finance application here...
}

#[derive(Debug, Clone)]
struct DepositResult {
    success: bool,
}

#[derive(Debug, Clone)]
struct WithdrawResult {
    success: bool,
}

/// An error that can occur while querying the service.
#[derive(Debug, Error)]
pub enum ServiceError {
    /// Invalid query argument; could not deserialize request.
    #[error("Invalid query argument; could not deserialize request")]
    InvalidQuery(#[from] serde_json::Error),
    // Add more error variants here.
}