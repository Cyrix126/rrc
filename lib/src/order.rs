use quickxml_to_serde::{NullValue, xml_string_to_json};

use crate::constants::{API_DOMAIN, API_ENDPOINT_SALES, VERSION_EXPORT};
use crate::export::ExportError;
use crate::sales::SaleFetch;
use crate::{client::RakutenClient, customer::Customer};

pub struct Order {
    // reference
    pub sku: String,
    pub purchaseid: String,
    pub stock: u32,
    pub customer: Customer,
}

impl RakutenClient {
    // Sales in the last 48 hours
    pub fn export_new_sales(&self) -> Result<Vec<Order>, ExportError> {
        // uri of the get new sales action including authentication
        let url = format!(
            "https://{API_DOMAIN}/{API_ENDPOINT_SALES}?action=getnewsales&login={}&pwd={}&version={VERSION_EXPORT}",
            self.username, self.token
        );
        let mut rep_body = self.client.get(url).send()?.text()?;
        let conf_quickxml =
            quickxml_to_serde::Config::new_with_custom_values(true, "", "txt", NullValue::Null);
        rep_body = rep_body
            .chars()
            .filter(|c| !c.is_control())
            .collect::<String>();
        let result =
            serde_json::from_value::<SaleFetch>(xml_string_to_json(rep_body, &conf_quickxml)?)?;
        let mut orders = vec![];
        for sale in result.getnewsalesresult.response.sales.sale {
            orders.push(sale.order());
        }
        Ok(orders)
    }
}
