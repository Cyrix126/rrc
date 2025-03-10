use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::seller::SellerInfo;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    // Token used for the API,
    pub token: PathBuf,
    // Username of your account
    pub username: String,
    // profile number for overwriting the inventory with csv
    pub _overwrite_pf_nb: Option<u32>,
    // profile number for updating price of products
    pub update_price_pf_nb: Option<u32>,
    // profile number for updating shipment type of products
    pub update_shipping_pf_nb: Option<u32>,
    // profile number for fast creation/update with csv (with existent barcode on Rakuten side)
    pub fast_update_pf_nb: Option<u32>,
    // seller info, used for import
    pub seller_info: Option<SellerInfo>,
}
