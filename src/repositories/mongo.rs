use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::Utc;
use futures::{future, stream, StreamExt};
use mongodb::results::InsertOneResult;
use mongodb::{bson::oid::ObjectId, Client, Collection};

use crate::constants::db::{DB_NAME, MONGOURI, ORDERS_COLLECTION, PRODUCTS_COLLECTION};
use crate::models::order::{FullOrder, FullOrderItem, Order};
use crate::models::product::{Product, ShortenedProduct};
use crate::repositories::order::OrderRepository;
use crate::repositories::product::ProductRepository;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;

#[derive(Debug, Clone)]
pub struct MongoRepository {
    orders: Collection<Order>,
    products: Collection<Product>,
}

impl MongoRepository {
    pub async fn init() -> Self {
        let uri = dotenv::var(MONGOURI).unwrap();

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database(DB_NAME);
        let orders = db.collection::<Order>(ORDERS_COLLECTION);
        let products = db.collection::<Product>(PRODUCTS_COLLECTION);

        Self { orders, products }
    }
}

#[async_trait]
impl OrderRepository for MongoRepository {
    type Order = Order;
    type CreateOrderResult = FullOrder;
    async fn create_order(&self, order: Self::Order) -> Result<Self::CreateOrderResult> {
        let new_order = Order {
            id: None,
            creation_time: Some(Utc::now()),
            ..order
        };

        let products = stream::iter(new_order.ordered_products.clone())
            .map(|product| async move {
                let db_product = self
                    .get_product_by_id(&product.product_id)
                    .await?
                    .ok_or(anyhow!("product not found"));

                match db_product {
                    Ok(db_product) => Ok(FullOrderItem {
                        product: ShortenedProduct {
                            id: db_product.id.unwrap().to_string(),
                            name: db_product.name,
                            price: db_product.price,
                        },
                        quantity: product.quantity,
                    }),
                    Err(_) => Err(anyhow!("product not found")),
                }
            })
            .buffered(10)
            .try_collect::<Vec<_>>()
            .await?;

        let created_order = self.orders.insert_one(new_order.clone(), None).await?;

        Ok(FullOrder {
            id: created_order.inserted_id.as_object_id(),
            creation_time: new_order.creation_time,
            first_name: new_order.first_name,
            last_name: new_order.last_name,
            email: new_order.email,
            phone_number: new_order.phone_number,
            delivery: new_order.delivery,
            ordered_products: products,
        })
    }
}

#[async_trait]
impl ProductRepository for MongoRepository {
    type Product = Product;

    async fn get_all_products(&self) -> Result<Vec<Self::Product>> {
        let cursor = self.products.find(None, None).await?;
        Ok(cursor.try_collect().await?)
    }

    async fn get_product_by_id(&self, product_id: &str) -> Result<Option<Self::Product>> {
        let obj_id = ObjectId::parse_str(product_id)?;
        let filter = doc! {"_id": obj_id};
        Ok(self.products.find_one(filter, None).await?)
    }
}
