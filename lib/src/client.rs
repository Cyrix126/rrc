use derive_builder::Builder;
use reqwest::{Client, ClientBuilder, Error};

use crate::seller::SellerInfo;

/// The Client to use to interact with Rakuten API or directly with the website.
///
/// To use imports with the API, you must set profiles number attributed by Rakuten.
/// <https://fr.shopping.rakuten.com/file?action=tplselect>
#[derive(Builder)]
pub struct RakutenClient {
    // reqwest client
    #[builder(setter(custom))]
    pub(crate) client: Client,
    // Token used for the API,
    pub(crate) token: String,
    // Username of your account
    pub(crate) username: String,
    // profile number for overwriting the inventory with csv
    pub(crate) _overwrite_pf_nb: u32,
    // profile number for updating price of products
    pub(crate) update_price_pf_nb: u32,
    // profile number for updating shipment type of products
    pub(crate) update_shipping_pf_nb: u32,
    // profile number for fast creation/update with csv (with existent barcode on Rakuten side)
    pub(crate) fast_update_pf_nb: u32,
    // seller info, used for import
    pub(crate) seller_info: SellerInfo,
}

impl RakutenClientBuilder {
    /// Give a ClientBuilder so that you can provide your custom client while adding the required defaults for Rakuten
    pub fn client(&mut self, value: ClientBuilder) -> Result<(), Error> {
        let client = value.redirect(reqwest::redirect::Policy::none()).build()?;
        self.client = Some(client);
        Ok(())
    }
}
