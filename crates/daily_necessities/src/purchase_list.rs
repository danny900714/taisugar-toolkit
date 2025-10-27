use jiff::civil::Date;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::Formatter;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Purchase {
    #[serde(rename = "step_id")]
    pub station_id: String,
    #[serde(rename = "stepname")]
    pub station_name: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: Date,
    pub product_id: String,
    #[serde(rename = "prname")]
    pub product_name: String,
    pub class: String,
    pub sup_id: Option<String>,
    pub sup_name: Option<String>,
    pub rcpt: String,
    pub price: String,
    #[serde(rename = "qty")]
    pub quantity: String,
    #[serde(rename = "notax_amt")]
    pub amount_before_tax: String,
    #[serde(rename = "GROUP")]
    pub group: String,
    #[serde(rename = "AREA")]
    pub area: String,
    pub dep: String,
    pub sep: String,
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    struct JiffDateVisitor;

    impl<'de> Visitor<'de> for JiffDateVisitor {
        type Value = Date;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing a date in YYYYMMDD format")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let date = Date::strptime("%Y%m%d", v)
                .map_err(|e| E::custom(format!("Failed to parse date: {}", e)))?;
            Ok(date)
        }
    }

    deserializer.deserialize_str(JiffDateVisitor)
}

#[derive(Debug, Deserialize)]
pub struct PurchaseList {
    data: Vec<Purchase>,
}

impl PurchaseList {
    pub fn iter(&self) -> impl Iterator<Item = &Purchase> + '_ {
        self.data.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deserialize_purchase_list() -> PurchaseList {
        let json = include_bytes!("../../../testdata/purchase-list.json");
        serde_json::from_slice(json).unwrap()
    }

    #[test]
    fn test_purchase_list_iter() {
        let purchase_list = deserialize_purchase_list();
        let mut iter = purchase_list.iter();
        assert_eq!(
            iter.next(),
            Some(&Purchase {
                station_id: "IIB11".to_string(),
                station_name: "柳林".to_string(),
                date: Date::new(2025, 9, 1).unwrap(),
                product_id: "A815".to_string(),
                product_name: "散裝尿素水--諾瓦".to_string(),
                class: "102".to_string(),
                sup_id: None,
                sup_name: None,
                rcpt: "1140821".to_string(),
                price: "9.0000".to_string(),
                quantity: "1000.00".to_string(),
                amount_before_tax: "9000.0000".to_string(),
                group: "中".to_string(),
                area: "2".to_string(),
                dep: "11".to_string(),
                sep: "62".to_string(),
            })
        );
        assert_eq!(
            iter.last(),
            Some(&Purchase {
                station_id: "SSC84".to_string(),
                station_name: "博學".to_string(),
                date: Date::new(2025, 9, 30).unwrap(),
                product_id: "A017".to_string(),
                product_name: "[民] 冰棒 (博學站)".to_string(),
                class: "102".to_string(),
                sup_id: None,
                sup_name: None,
                rcpt: "1".to_string(),
                price: "0.0000".to_string(),
                quantity: "2.00".to_string(),
                amount_before_tax: "0.0000".to_string(),
                group: "南".to_string(),
                area: "5".to_string(),
                dep: "13".to_string(),
                sep: "47".to_string(),
            })
        )
    }
}
