use jiff::civil::Date;
use serde::Deserialize;
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

#[derive(Debug, PartialEq)]
pub struct Item {
    pub id: String,
    pub title: String,
}

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
                    kv.key == "NAME"
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
}
