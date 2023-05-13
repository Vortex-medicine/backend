use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait OrderRepository {
    type Order;
    type CreateOrderResult;

    async fn create_order(&self, order: Self::Order) -> Result<Self::CreateOrderResult>;
}
