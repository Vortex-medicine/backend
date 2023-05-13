use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub type ProductName = String;
pub type ProductPrice = f64;

pub type LocalizedProductDescr = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductDescr {
    pub uk: LocalizedProductDescr,
    pub en: LocalizedProductDescr,
    pub ru: LocalizedProductDescr,
}

pub type ImgPath = String;
pub type ImgWidth = u64;
pub type ImgHeight = u64;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductImg {
    pub path: ImgPath,
    pub width: ImgWidth,
    pub height: ImgHeight,
}

pub type LocalizedDiscountInfo = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscountInfo {
    pub uk: LocalizedDiscountInfo,
    pub en: LocalizedDiscountInfo,
    pub ru: LocalizedDiscountInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: ProductName,
    pub descr: ProductDescr,
    pub img: ProductImg,
    pub price: ProductPrice,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_info: Option<DiscountInfo>,
}

pub type ProductId = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShortenedProduct {
    pub id: ProductId,
    pub name: ProductName,
    pub price: ProductPrice,
}
