use derive_builder::Builder;
use get_pass::get_password;
use reqwest::{
    Error,
    blocking::{Client, ClientBuilder},
};

use crate::{config::Config, seller::SellerInfo};

/// The Client to use to interact with Rakuten API or directly with the website.
///
/// To use imports with the API, you must set profiles number attributed by Rakuten.
/// <https://fr.shopping.rakuten.com/file?action=tplselect>
#[derive(Builder, Default)]
pub struct RakutenClient {
    // reqwest client
    #[builder(setter(custom))]
    pub(crate) client: Client,
    // Token used for the API,
    pub(crate) token: String,
    // Username of your account
    pub(crate) username: String,
    // profile number for overwriting the inventory with csv
    #[builder(setter(into, strip_option), default)]
    pub(crate) _overwrite_pf_nb: Option<u32>,
    // profile number for updating price of products
    #[builder(setter(into, strip_option), default)]
    pub(crate) update_price_pf_nb: Option<u32>,
    // profile number for updating shipment type of products
    #[builder(setter(into, strip_option), default)]
    pub(crate) update_shipping_pf_nb: Option<u32>,
    // profile number for fast creation/update with csv (with existent barcode on Rakuten side)
    #[builder(setter(into, strip_option), default)]
    pub(crate) fast_update_pf_nb: Option<u32>,
    // seller info, used for import
    #[builder(setter(into, strip_option), default)]
    pub(crate) seller_info: Option<SellerInfo>,
}

impl RakutenClientBuilder {
    /// Give a ClientBuilder so that you can provide your custom client while adding the required defaults for Rakuten
    pub fn client(mut self, value: ClientBuilder) -> Result<Self, Error> {
        let client = value.redirect(reqwest::redirect::Policy::none()).build()?;
        self.client = Some(client);
        Ok(self)
    }
}

impl TryFrom<Config> for RakutenClient {
    type Error = Box<dyn std::error::Error>;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        let mut builder = RakutenClientBuilder::default().client(ClientBuilder::new())?;

        builder.token(get_password(&config.token)?);
        builder.username(config.username);

        if let Some(update_price_pf_nb) = config.update_price_pf_nb {
            builder.update_price_pf_nb(update_price_pf_nb);
        }
        if let Some(fast_update_pf_nb) = config.fast_update_pf_nb {
            builder.fast_update_pf_nb(fast_update_pf_nb);
        }
        if let Some(update_shipping_pf_nb) = config.update_shipping_pf_nb {
            builder.update_shipping_pf_nb(update_shipping_pf_nb);
        }
        if let Some(seller_info) = config.seller_info {
            builder.seller_info(seller_info);
        }
        Ok(builder.build()?)
    }
}
