// new advert to import
pub mod advert;
// product that already exist on Rakuten
pub mod product;
// HTTP client to interact with Rakuten
pub mod client;
// defined value by Rakuten
pub(crate) mod constants;
// method to retrieve products
pub mod export;
// method to import/update/delete products
pub mod import;
// data related to the seller
pub mod seller;
// configuration file
pub mod config;
// new orders
pub mod order;
// sales as exported
pub mod sales;
// definition of a customer
pub mod customer;
