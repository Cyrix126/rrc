use crate::constants::API_ENDPOINT_STOCK;
use reqwest::blocking::multipart::{Form, Part};

use thiserror::Error;

use crate::advert::{AdvertError, ShippingType};
use crate::constants::{API_DOMAIN, VERSION_IMPORT_CSV};
use crate::seller::SellerInfo;
use crate::{advert::Advert, client::RakutenClient};

impl RakutenClient {
    /// Create or update full products with barcode that exist on Rakuten
    pub fn fast_create_update_products(&self, products: Vec<Advert>) -> Result<(), ImportError> {
        let mut lines = vec![];
        for p in products {
            lines.push(
                p.fast_import_csv_line(
                    self.seller_info
                        .as_ref()
                        .ok_or(ImportError::ClientInfoMissing)?,
                )?,
            );
        }
        self.import(
            lines,
            self.fast_update_pf_nb
                .ok_or(ImportError::ClientInfoMissing)?,
        )?;
        Ok(())
    }

    /// Update the price of products
    pub fn update_price(&self, sku_prices: Vec<(String, f32)>) -> Result<(), ImportError> {
        let mut lines = vec![];
        sku_prices
            .iter()
            .for_each(|(sku, price)| lines.push(format!("{sku};{price}")));
        self.import(
            lines,
            self.update_price_pf_nb
                .ok_or(ImportError::ClientInfoMissing)?,
        )?;
        Ok(())
    }

    /// Update the shipping of products
    pub fn update_shipping(
        &self,
        sku_shipping: Vec<(String, ShippingType)>,
    ) -> Result<(), ImportError> {
        let mut lines = vec![];
        sku_shipping
            .iter()
            .for_each(|(sku, shipping)| lines.push(format!("{sku};{shipping}")));
        self.import(
            lines,
            self.update_shipping_pf_nb
                .ok_or(ImportError::ClientInfoMissing)?,
        )?;
        Ok(())
    }

    fn import(&self, lines: Vec<String>, profile_id: u32) -> Result<(), ImportError> {
        let url = format!("https://{API_DOMAIN}/{API_ENDPOINT_STOCK}?action=import");
        let csv = lines.join("\n");
        let file = Part::bytes(csv.into_bytes())
            .file_name("import.csv")
            .mime_str("csv/txt")
            .unwrap();
        let form = Form::new()
            .text("version", VERSION_IMPORT_CSV)
            .text("pwd", self.token.to_string())
            .text("profileid", profile_id.to_string())
            .text("login", self.username.to_string())
            .part("csv", file);
        self.client.post(url).multipart(form).send()?;
        Ok(())
    }
}
#[derive(Error, Debug)]
pub enum ImportError {
    #[error("data is missing from product")]
    DataMissing,
    #[error("data malformated from product")]
    Advert(#[from] AdvertError),
    #[error("Rakuten returned an error")]
    Reqwest(#[from] reqwest::Error),
    #[error("Information missing from client")]
    ClientInfoMissing,
}

impl Advert {
    /// line produced for import when a barcode exist
    pub fn fast_import_csv_line(&self, seller_info: &SellerInfo) -> Result<String, ImportError> {
        let barcode = self
            .productsummary
            .barcode
            .as_ref()
            .ok_or(ImportError::DataMissing)?
            .to_owned()
            .to_string();
        // check that barcode is 13 char
        // The price is always considered in €.
        let price = self.price.amount.to_string();
        let quality = self.quality.to_string();
        let quantity = self.stock.to_string();
        let public_note = self.comment.clone().unwrap_or_default();
        let title = self.productsummary.headline.clone().unwrap_or_default();
        let sku = self.sku.clone().unwrap().to_string();
        let private_note = self.privatecomment.clone().unwrap_or_default();
        let promo_code = "".to_string();
        let images = self
            .images()?
            .iter()
            .map(|u| u.as_str())
            .collect::<Vec<&str>>()
            .join("#");
        let shipping = self.shippingtype.clone().unwrap_or_default().to_string();
        let weight = self.productsummary.weight_g.unwrap_or_default().to_string();
        let brand = self.productsummary.brand.clone().unwrap_or_default();
        let rich_description = self.rich_description.clone().unwrap_or_default();
        let refurbished = self.isrefurbished.to_string();
        let category_code = self.category.to_string();

        // no support for different type of shipping for different places
        Ok(vec![
            barcode,
            price,
            quality,
            quantity,
            public_note,
            title,
            sku,
            private_note,
            promo_code,
            images,
            shipping,
            seller_info.phone.clone(),
            seller_info.postal_code.clone(),
            seller_info.country.clone(),
            weight,
            brand,
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            rich_description,
            refurbished,
            category_code,
        ]
        .join("\t"))
    }
}
