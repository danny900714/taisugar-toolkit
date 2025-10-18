use std::fmt::Display;
use jiff::civil::Date;
use serde::Deserialize;

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

#[derive(Deserialize)]
struct DynamicColumn {
    field: String,
    title: String,

    #[allow(dead_code)]
    width: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DataKV {
    key: String,
    value: Value,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Value {
    String(String),
    Number(u64),
}

pub struct Item {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize)]
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
