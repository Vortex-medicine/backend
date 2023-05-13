use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ProductRepository {
    type Product;
    async fn get_all_products(&self) -> Result<Vec<Self::Product>>;
    async fn get_product_by_id(&self, product_id: &str) -> Result<Option<Self::Product>>;
}
