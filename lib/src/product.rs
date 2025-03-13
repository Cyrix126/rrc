use crate::constants::API_DOMAIN;
use crate::constants::API_ENDPOINT_LISTING;
use crate::constants::VERSION_LISTING;
use quickxml_to_serde::{NullValue, xml_string_to_json};
use serde::{Deserialize, Serialize};

use crate::{client::RakutenClient, export::ExportError};

#[derive(Serialize, Deserialize)]
pub struct Listing {
    #[serde(rename = "listingresult")]
    listingresult: Listingresult,
}

#[derive(Serialize, Deserialize)]
pub struct Listingresult {
    #[serde(rename = "response")]
    response: Response,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "products")]
    products: Products,
}

#[derive(Serialize, Deserialize)]
pub struct Products {
    #[serde(rename = "product")]
    product: Vec<Product>,
}

#[derive(Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "productid")]
    productid: String,

    #[serde(rename = "alias")]
    alias: String,

    #[serde(rename = "headline")]
    headline: String,

    #[serde(rename = "caption")]
    caption: String,

    #[serde(rename = "topic")]
    topic: String,

    #[serde(rename = "offercounts")]
    offercounts: Offercounts,

    #[serde(rename = "bestprices")]
    bestprices: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "image")]
    image: String,

    #[serde(rename = "references")]
    references: References,
}

#[derive(Serialize, Deserialize)]
pub struct Offercounts {
    #[serde(rename = "total")]
    total: String,

    #[serde(rename = "new")]
    new: String,

    #[serde(rename = "used")]
    used: String,

    #[serde(rename = "collectible")]
    collectible: String,
}

#[derive(Serialize, Deserialize)]
pub struct References {
    #[serde(rename = "barcode")]
    barcode: String,

    #[serde(rename = "partnumber")]
    partnumber: Option<String>,
}

impl RakutenClient {
    /// https://outils.fr.shopping.rakuten.com/dev-pro/fr/documentation/Donnees_produits/Rechercher_des_produits_Listing_version_securisee_Rakuten_France_Webservices.html
    /// get data of a product from a barcode. The product may not exist. Multiple products can be reported ?
    /// TODO: figure out if this function is necessary since product_from_title could also be used with a barcode.
    pub async fn product_from_barcode(&self, barcode: &str) -> Result<Vec<Product>, ExportError> {
        let url = format!(
            "https://{API_DOMAIN}/{API_ENDPOINT_LISTING}?action=listing&login={}&pwd={}&version={VERSION_LISTING}&scope=PRICING&refs={barcode}&nbproductsperpage=10",
            self.username, self.token
        );
        let mut rep_body = self.client.get(url).send()?.text()?;
        let conf_quickxml =
            quickxml_to_serde::Config::new_with_custom_values(true, "", "txt", NullValue::Null);
        rep_body = rep_body
            .chars()
            .filter(|c| !c.is_control())
            .collect::<String>();
        let listing =
            serde_json::from_value::<Listing>(xml_string_to_json(rep_body, &conf_quickxml)?)?;
        let mut products = vec![];
        for product in listing.listingresult.response.products.product {
            products.push(product);
        }
        Ok(products)
    }
    /// https://outils.fr.shopping.rakuten.com/dev-pro/fr/documentation/Donnees_produits/Rechercher_des_produits_Listing_version_securisee_Rakuten_France_Webservices.html
    /// get data of a product from a title. The product may not exist. Multiple products can be reported.
    pub async fn product_from_title(&self, title: &str) -> Result<Vec<Product>, ExportError> {
        let url = format!(
            "https://{API_DOMAIN}/{API_ENDPOINT_LISTING}?action=listing&login={}&pwd={}&version={VERSION_LISTING}&scope=PRICING&key={title}&nbproductsperpage=10",
            self.username, self.token
        );
        let mut rep_body = self.client.get(url).send()?.text()?;
        let conf_quickxml =
            quickxml_to_serde::Config::new_with_custom_values(true, "", "txt", NullValue::Null);
        rep_body = rep_body
            .chars()
            .filter(|c| !c.is_control())
            .collect::<String>();
        let listing =
            serde_json::from_value::<Listing>(xml_string_to_json(rep_body, &conf_quickxml)?)?;
        let mut products = vec![];
        for product in listing.listingresult.response.products.product {
            products.push(product);
        }
        Ok(products)
    }
}
