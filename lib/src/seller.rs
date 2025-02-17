use derive_builder::Builder;

#[derive(Default, Builder, Clone)]
pub struct SellerInfo {
    // can only include 0123456789
    // must ne a string since it can began with 0
    pub phone: String,
    // can only include 5 numbers
    pub postal_code: String,
    pub country: String,
}

impl SellerInfo {
    pub fn valid_phone(&self) -> bool {
        self.phone.parse::<u64>().is_ok()
    }
    pub fn valid_postalcode(&self) -> bool {
        self.phone.parse::<u32>().is_ok()
    }
}
