use derive_more::Display;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Advert {
    pub advertid: i64,
    pub adverttype: String,
    pub comment: Option<String>,
    #[serde(skip)]
    pub rich_description: Option<String>,
    // https://fr.shopping.rakuten.com/restpublic/sel-web/products/categories
    #[serde(skip)]
    pub category: u32,
    pub isnegotiable: bool,
    pub isoriginal: bool,
    pub isrefurbished: bool,
    pub isrsl: String,
    pub price: Price,
    pub productsummary: Productsummary,
    pub quality: Quality,
    pub shippingcost: Option<Shippingcost>,
    #[serde(default)]
    pub shippingtype: ShippingType,
    pub sku: Option<String>,
    pub stock: i64,
    pub unlimitedquantity: bool,
    pub importtag: Option<String>,
    pub privatecomment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Productsummary {
    pub alias: String,
    pub barcode: Option<String>,
    pub caption: Option<String>,
    pub digest: Option<String>,
    pub headline: Option<String>,
    pub productid: i64,
    pub topic: Option<String>,
    pub url: String,
    pub isbn: Option<String>,
    #[serde(skip)]
    pub weight_g: Option<u32>,
    #[serde(skip)]
    pub brand: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Shippingcost {
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Display, Default)]
#[serde(rename_all = "camelCase")]
pub enum ShippingType {
    #[default]
    #[display("RET")]
    Ret,
    #[display("EXP")]
    Exp,
    #[serde(rename(deserialize = "Point relais Mondial Relay"))]
    #[display("EXP / RET")]
    ExpAndRet,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Display)]
pub enum Quality {
    #[display("N")]
    #[serde(rename(serialize = "N"))]
    #[serde(rename(deserialize = "NEW"))]
    New,
    #[display("CN")]
    #[serde(rename(serialize = "CN"))]
    #[serde(rename(deserialize = "LIKE_NEW"))]
    LikeNew,
    #[display("TBE")]
    #[serde(rename(serialize = "TBE"))]
    #[serde(rename(deserialize = "VERY_GOOD"))]
    VeryGood,
    #[display("BE")]
    #[serde(rename(serialize = "BE"))]
    #[serde(rename(deserialize = "GOOD"))]
    Good,
    #[display("EC")]
    #[serde(rename(serialize = "EC"))]
    #[serde(rename(deserialize = "CORRECT"))]
    Correct,
}

impl Quality {
    /// Use if importing products by using the website directly
    fn _direct_web_str(&self) -> &str {
        match self {
            Self::New => "USED_LIKE_NEW",
            Self::LikeNew => "USED_LIKE_NEW",
            Self::VeryGood => "USED_VERY_GOOD",
            Self::Good => "USED_GOOD",
            Self::Correct => "USED_CORRECT",
        }
    }
}

#[derive(Error, Debug)]
pub enum AdvertError {
    #[error("Advert data is malformated")]
    DataMalformated,
}
impl Advert {
    pub fn images(&self) -> Result<Vec<Url>, AdvertError> {
        self.productsummary
            .url
            .split("#")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|url| Url::parse(url).map_err(|_| AdvertError::DataMalformated))
            .collect()
    }
}
