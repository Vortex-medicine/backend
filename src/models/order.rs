use serde::{Deserialize, Serialize};

pub type Country = String;
pub type City = String;
pub type Address = String;
pub type Zip = String;
pub type Warehouse = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct NovaposhtaDeliveryKind {
    city: City,
    warehouse: Warehouse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CourierDeliveryKing {
    city: City,
    address: Address,
    zip: Zip,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldwideDeliveryKing {
    country: Country,
    city: City,
    address: Address,
    zip: Zip,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "kind", content = "data")]
pub enum Delivery {
    Novaposhta(NovaposhtaDeliveryKind),
    Courier(CourierDeliveryKing),
    Worldwide(WorldwideDeliveryKing),
}

pub type OrderItemId = String;
pub type OrderItemName = String;
pub type OrderItemDescr = String;
pub type OrderItemPrice = f64;
pub type OrderItemQuantity = u64;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderItem {
    id: OrderItemId,
    name: OrderItemName,
    descr: OrderItemDescr,
    price: OrderItemPrice,
    quantity: OrderItemQuantity,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    first_name: String,
    last_name: String,
    email: String,
    phone_number: String,
    delivery: Delivery,
    ordered_products: Vec<OrderItem>,
}
