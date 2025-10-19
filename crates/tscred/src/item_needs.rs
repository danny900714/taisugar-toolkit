use crate::date::parse_date_from_roc_calendar;
use jiff::civil::Date;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;

pub struct GetItemNeedsOptions<'a> {
    pub operation_center_id: &'a str,
    pub start_date: &'a Date,
    pub end_date: &'a Date,
    pub display_mode: &'a DisplayMode,
    pub department_id: &'a str,
}

pub enum DisplayMode {
    ByStation,
    ByDate,
    Details,
}

impl Display for DisplayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DisplayMode::ByStation => "1".to_string(),
            DisplayMode::ByDate => "2".to_string(),
            DisplayMode::Details => "3".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Deserialize, Debug)]
struct DynamicColumn {
    field: String,
    title: String,

    #[allow(dead_code)]
    width: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct DataKV {
    key: String,
    value: Value,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Value {
    String(String),
    Number(u64),
}

const STATION_NAME_KEY: &str = "NAME";
const ORDER_DATE_KEY: &str = "ORDNO";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ItemNeeds {
    dynamic_columns: Vec<DynamicColumn>,
    data: Vec<Vec<DataKV>>,
}

impl ItemNeeds {
    pub fn get_all_items(&self) -> Vec<Item> {
        let mut items = Vec::with_capacity(self.dynamic_columns.len() - 3);
        for column in self.dynamic_columns.iter() {
            // Find all columns whose field starts with "A_", which are columns for items
            if column.field.starts_with("A_") {
                items.push(Item {
                    id: column.field.clone(),
                    title: column.title.clone(),
                })
            }
        }
        items
    }

    pub fn get_station_item_need_count(&self, station_name: &str, item_id: &str) -> Option<u64> {
        match &self
            .data
            .iter()
            // Find the station vector with the given name
            .find(|station_need| {
                station_need.iter().any(|kv| {
                    kv.key == STATION_NAME_KEY
                        && match &kv.value {
                            Value::String(value) => value == station_name,
                            Value::Number(_) => false,
                        }
                })
            })?
            .iter()
            // Find the key-value pair with the given item id
            .find(|kv| kv.key == item_id)?
            .value
        {
            Value::String(_) => None,
            Value::Number(value) => Some(*value),
        }
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            target: self,
            index: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Item {
    pub id: String,
    pub title: String,
}

pub struct ItemNeed<'a> {
    station_name: &'a str,
    order_date: Date,
    items_count: HashMap<&'a str, u64>,
}

pub struct Iter<'a> {
    target: &'a ItemNeeds,
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = ItemNeed<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let raw_item_need = self.target.data.get(self.index)?;
        self.index += 1;

        let mut station_name = "";
        let mut order_date = Date::default();
        let mut items_count = HashMap::new();
        for kv in raw_item_need.iter() {
            match &kv.value {
                Value::String(value) => match kv.key.as_str() {
                    STATION_NAME_KEY => station_name = value,
                    ORDER_DATE_KEY => {
                        order_date = parse_date_from_roc_calendar(value)
                            .expect("unable to parse date from roc calendar")
                    }
                    _ => {}
                },
                Value::Number(value) => {
                    items_count.insert(kv.key.as_str(), *value);
                }
            }
        }

        Some(ItemNeed {
            station_name,
            order_date,
            items_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deserialize_item_needs() -> ItemNeeds {
        let json = include_bytes!("../testdata/GetItemNeedCount.json");
        serde_json::from_slice(json).expect("Failed to deserialize GetItemNeedCount.json")
    }

    #[test]
    fn test_item_needs_deserialize() {
        deserialize_item_needs();
    }

    #[test]
    fn test_get_all_items() {
        let item_needs = deserialize_item_needs();
        let items = item_needs.get_all_items();
        assert_eq!(
            items,
            vec![
                Item {
                    id: "A_G960".to_string(),
                    title: "[民]台糖詩夢絲環保洗衣精".to_string(),
                },
                Item {
                    id: "A_G001".to_string(),
                    title: "60抽盒裝面紙".to_string(),
                },
                Item {
                    id: "A_G002".to_string(),
                    title: "台糖礦泉水/箱".to_string(),
                },
                Item {
                    id: "A_G277".to_string(),
                    title: "原味蜆精62cc".to_string(),
                },
                Item {
                    id: "A_G281".to_string(),
                    title: "寡醣乳酸菌(正常包)".to_string(),
                },
                Item {
                    id: "A_G298".to_string(),
                    title: "妙管家強效洗衣粉4.5KG".to_string(),
                },
                Item {
                    id: "A_G316".to_string(),
                    title: "泡舒洗潔精1000ml".to_string(),
                },
                Item {
                    id: "A_G330".to_string(),
                    title: "五月花110抽連續抽取式衛生紙".to_string(),
                },
                Item {
                    id: "A_G363".to_string(),
                    title: "妙管家抗菌洗衣精4000gm".to_string(),
                },
                Item {
                    id: "A_GP01".to_string(),
                    title: "洗手間環保大捲筒衛生紙".to_string(),
                },
                Item {
                    id: "A_G411".to_string(),
                    title: "妙管家-衣物柔軟精補充包2L".to_string(),
                },
                Item {
                    id: "A_G412".to_string(),
                    title: "妙管家-濃縮洗衣精補充包2L".to_string(),
                },
                Item {
                    id: "A_G432".to_string(),
                    title: "110抽盒裝面紙".to_string(),
                }
            ]
        )
    }

    #[test]
    fn test_items_need_iter() {
        let item_needs = deserialize_item_needs();
        let item_needs = item_needs.iter().collect::<Vec<_>>();
        assert_eq!(item_needs.len(), 45);

        let first_item_need = &item_needs[0];
        assert_eq!(first_item_need.station_name, "成功嶺站");
        assert_eq!(first_item_need.order_date, Date::new(2025, 9, 30).unwrap());
        assert_eq!(first_item_need.items_count.len(), 13);
        assert_eq!(first_item_need.items_count.get("A_G960"), Some(&0));
        assert_eq!(first_item_need.items_count.get("A_G001"), Some(&0));
        assert_eq!(first_item_need.items_count.get("A_G002"), Some(&60));
        assert_eq!(first_item_need.items_count.get("A_G277"), Some(&0));
        assert_eq!(first_item_need.items_count.get("A_G281"), Some(&0));
        assert_eq!(first_item_need.items_count.get("A_G298"), Some(&0));
        assert_eq!(first_item_need.items_count.get("A_G316"), Some(&0));
        assert_eq!(first_item_need.items_count.get("A_G330"), Some(&5));
        assert_eq!(first_item_need.items_count.get("A_G363"), Some(&0));
        assert_eq!(first_item_need.items_count.get("A_GP01"), Some(&0));
        assert_eq!(first_item_need.items_count.get("A_G411"), Some(&0));
        assert_eq!(first_item_need.items_count.get("A_G412"), Some(&0));
        assert_eq!(first_item_need.items_count.get("A_G432"), Some(&0));

        let last_item_need = &item_needs[44];
        assert_eq!(last_item_need.station_name, "嘉保站");
        assert_eq!(last_item_need.order_date, Date::new(2025, 9, 20).unwrap());
        assert_eq!(last_item_need.items_count.len(), 13);
        assert_eq!(last_item_need.items_count.get("A_G960"), Some(&0));
        assert_eq!(last_item_need.items_count.get("A_G001"), Some(&30));
        assert_eq!(last_item_need.items_count.get("A_G002"), Some(&0));
        assert_eq!(last_item_need.items_count.get("A_G277"), Some(&0));
        assert_eq!(last_item_need.items_count.get("A_G281"), Some(&0));
        assert_eq!(last_item_need.items_count.get("A_G298"), Some(&0));
        assert_eq!(last_item_need.items_count.get("A_G316"), Some(&0));
        assert_eq!(last_item_need.items_count.get("A_G330"), Some(&0));
        assert_eq!(last_item_need.items_count.get("A_G363"), Some(&0));
        assert_eq!(last_item_need.items_count.get("A_GP01"), Some(&0));
        assert_eq!(last_item_need.items_count.get("A_G411"), Some(&0));
        assert_eq!(last_item_need.items_count.get("A_G412"), Some(&0));
        assert_eq!(last_item_need.items_count.get("A_G432"), Some(&0));
    }
}
