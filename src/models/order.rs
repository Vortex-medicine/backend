use crate::models::product::{Product, ShortenedProduct};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub type Country = String;
pub type City = String;
pub type Address = String;
pub type Zip = String;
pub type Warehouse = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NovaposhtaDeliveryKind {
    pub city: City,
    pub warehouse: Warehouse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CourierDeliveryKing {
    pub city: City,
    pub address: Address,
    pub zip: Zip,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldwideDeliveryKing {
    pub country: Country,
    pub city: City,
    pub address: Address,
    pub zip: Zip,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "kind", content = "data")]
pub enum Delivery {
    Novaposhta(NovaposhtaDeliveryKind),
    Courier(CourierDeliveryKing),
    Worldwide(WorldwideDeliveryKing),
}

pub type OrderItemQuantity = u64;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderItem {
    pub product_id: String,
    pub quantity: OrderItemQuantity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullOrderItem {
    pub product: ShortenedProduct,
    pub quantity: OrderItemQuantity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<DateTime<Utc>>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub delivery: Delivery,
    pub ordered_products: Vec<OrderItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FullOrder {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<DateTime<Utc>>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub delivery: Delivery,
    pub ordered_products: Vec<FullOrderItem>,
}
