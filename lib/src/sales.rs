use serde::{Deserialize, Serialize};

use crate::{customer::Customer, order::Order};

#[derive(Serialize, Deserialize)]
pub struct SaleFetch {
    pub getnewsalesresult: Getnewsalesresult,
}

#[derive(Serialize, Deserialize)]
pub struct Getnewsalesresult {
    pub response: Response,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub sales: Sales,
}

#[derive(Serialize, Deserialize)]
pub struct Sales {
    pub sale: Vec<Sale>,
}

#[derive(Serialize, Deserialize)]
pub struct Sale {
    pub purchaseid: String,
    pub purchasedate: String,
    pub deliveryinformation: Deliveryinformation,
    pub items: Items,
}

#[derive(Serialize, Deserialize)]
pub struct Deliveryinformation {
    pub shippingtype: String,
    pub isfullrsl: String,
    pub purchasebuyerlogin: String,
    pub purchasebuyeremail: String,
    pub deliveryaddress: Deliveryaddress,
}

#[derive(Serialize, Deserialize)]
pub struct Deliveryaddress {
    pub civility: String,
    pub lastname: String,
    pub firstname: String,
    pub address1: String,
    pub address2: String,
    pub zipcode: String,
    pub city: String,
    pub country: String,
    pub countryalpha2: String,
    pub phonenumber1: String,
    pub phonenumber2: String,
}

#[derive(Serialize, Deserialize)]
pub struct Items {
    pub item: Item,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub sku: String,
    pub advertid: String,
    pub advertpricelisted: Advertpricelisted,
    pub itemid: String,
    pub headline: String,
    pub itemstatus: String,
    pub ispreorder: String,
    pub is_fulfilment: String,
    pub isnego: String,
    pub negotiationcomment: String,
    pub price: Advertpricelisted,
    pub isrsl: String,
    pub isbn: String,
    pub ean: String,
    pub paymentstatus: String,
    pub sellerscore: String,
}

#[derive(Serialize, Deserialize)]
pub struct Advertpricelisted {
    pub amount: String,
    pub currency: String,
}

impl Sale {
    pub fn customer(&self) -> Customer {
        Customer {
            login: self.deliveryinformation.purchasebuyerlogin.to_string(),
            email: self.deliveryinformation.purchasebuyeremail.to_string(),
            civility: self
                .deliveryinformation
                .deliveryaddress
                .civility
                .to_string(),
            lastname: self
                .deliveryinformation
                .deliveryaddress
                .lastname
                .to_string(),
            firstname: self
                .deliveryinformation
                .deliveryaddress
                .firstname
                .to_string(),
            address1: self
                .deliveryinformation
                .deliveryaddress
                .address1
                .to_string(),
            zipcode: self.deliveryinformation.deliveryaddress.zipcode.to_string(),
            city: self.deliveryinformation.deliveryaddress.city.to_string(),
            country: self.deliveryinformation.deliveryaddress.country.to_string(),
            countryalpha2: self
                .deliveryinformation
                .deliveryaddress
                .countryalpha2
                .to_string(),
            phonenumber: self
                .deliveryinformation
                .deliveryaddress
                .phonenumber1
                .to_string(),
        }
    }
    pub fn order(&self) -> Order {
        Order {
            sku: self.items.item.sku.clone(),
            purchaseid: self.purchaseid.clone(),
            stock: 1,
            customer: self.customer(),
        }
    }
}
