use crate::{
    advert::Advert,
    constants::{API_DOMAIN, API_ENDPOINT_STOCK, VERSION_EXPORT},
};
use quickxml_to_serde::{NullValue, xml_string_to_json};
use serde::Deserialize;

use crate::client::RakutenClient;

pub const HEADERS_CSV: [&str; 27] = [
    "advertid",
    "adverttype",
    "comment",
    "isnegotiable",
    "isoriginal",
    "isrefurbished",
    "isrsl",
    "amount",
    "currency",
    "alias",
    "barcode",
    "caption",
    "digest",
    "headline",
    "productid",
    "topic",
    "url",
    "isbn",
    "quality",
    "shipping_amount",
    "shipping_currency",
    "shipping_type",
    "sku",
    "stock",
    "unlimitedquantity",
    "importtag",
    "privatecomment",
];

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Export {
    pub inventoryresult: Inventoryresult,
}

impl Export {
    fn nexttoken(&self) -> &Option<String> {
        &self.inventoryresult.response.nexttoken
    }
    fn products(self) -> Vec<Advert> {
        self.inventoryresult.response.advertlist.advert
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventoryresult {
    pub request: Request,
    pub response: ResponseRakuten,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub login: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseRakuten {
    pub advertlist: Advertlist,
    pub nbresults: i64,
    pub nexttoken: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Advertlist {
    pub advert: Vec<Advert>,
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("error while requesting")]
    Request(#[from] reqwest::Error),
    #[error("could not convert xml to json")]
    QuickXml(#[from] roxmltree::Error),
    #[error("could not deserialize json")]
    Json(#[from] serde_json::Error),
}

impl RakutenClient {
    /// export all the products of the seller
    ///
    /// <https://outils.fr.shopping.rakuten.com/dev-pro/fr/documentation/Gestion_de_stock/Export_d_inventaire_Export_Rakuten_France_Webservices.html>
    pub fn export(&self) -> Result<Vec<Advert>, ExportError> {
        // uri of the export action including authentication
        let url = format!(
            "https://{API_DOMAIN}/{API_ENDPOINT_STOCK}?action=export&login={}&pwd={}&version={VERSION_EXPORT}",
            self.username, self.token
        );
        // get the first page
        // about 4 seconds per full page, thanks Rakuten ?
        let first_page = self.request_export(&url)?;
        let mut nexttoken = first_page.nexttoken().clone();
        let mut products = first_page.products();
        while let Some(token) = nexttoken {
            let new_url = format!("{url}&nexttoken={token}");
            let page = self.request_export(&new_url)?;
            nexttoken = page.nexttoken().clone();
            products.extend_from_slice(&page.products());
        }
        Ok(products)
    }

    fn request_export(&self, url: &str) -> Result<Export, ExportError> {
        let mut rep_body = self.client.get(url).send()?.text()?;
        let conf_quickxml =
            quickxml_to_serde::Config::new_with_custom_values(true, "", "txt", NullValue::Null);
        rep_body = rep_body
            .chars()
            .filter(|c| !c.is_control())
            .collect::<String>();
        let export =
            serde_json::from_value::<Export>(xml_string_to_json(rep_body, &conf_quickxml)?)?;
        Ok(export)
    }
}
