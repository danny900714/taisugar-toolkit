use crate::{Error, Freebie};
use jiff::civil::Date;
use std::collections::HashMap;
use tscred::ItemNeeds;
use umya_spreadsheet::Spreadsheet;

pub fn generate_purchase_order_report<R: AsRef<str>>(
    template: &Spreadsheet,
    item_needs_slice: &[ItemNeeds],
    freebie: &Freebie,
    notification_date: &Date,
    order_number: R,
) -> Result<Spreadsheet, Error> {
    let mut spreadsheet = umya_spreadsheet::new_file_empty_worksheet();
    let worksheet = template
        .get_sheet_by_name("template")
        .ok_or(Error::MissingTemplateWorksheet)?;
    spreadsheet
        .add_sheet(worksheet.clone())
        .expect("unable to add sheet");

    let worksheet = spreadsheet.get_sheet_mut(&0).unwrap();

    // Set the notification date
    let notification_date_string = format!(
        "{}/{:02}/{:02}",
        notification_date.year() - 1911,
        notification_date.month(),
        notification_date.day()
    );
    worksheet
        .get_cell_mut(freebie.notification_date_coord())
        .set_value(notification_date_string);

    // Set the order number
    worksheet
        .get_cell_mut(freebie.order_number_coord())
        .set_value(freebie.order_number_cell_value(order_number));

    // Get the freebie ID
    let id = item_needs_slice
        .iter()
        .find_map(|item_needs| {
            item_needs
                .get_all_items()
                .into_iter()
                .find(|item| item.title == freebie.name())
                .map(|item| item.id)
        })
        .ok_or(Error::FreebieNotFound)?;

    // Set the item needs
    let stations = worksheet
        .get_cell_value_by_range("A5:A25")
        .into_iter()
        .enumerate()
        .map(|(i, value)| (value.get_value(), i + 5))
        .collect::<HashMap<_, _>>();
    for item_needs in item_needs_slice {
        for item_need in item_needs.iter() {
            if let Some(count) = item_need.items_count.get(id.as_str())
                && let Some(cord) = stations.get(item_need.station_name)
            {
                worksheet
                    .get_cell_mut(format!("C{}", cord))
                    .set_value_number(*count as f64);
            }
        }
    }

    Ok(spreadsheet)
}

#[cfg(test)]
mod tests {
    use crate::freebie::Freebie;
    use crate::purchase_order::generate_purchase_order_report;
    use jiff::civil::Date;
    use std::io::Cursor;
    use tscred::ItemNeeds;
    use umya_spreadsheet::reader;

    fn deserialize_item_needs() -> ItemNeeds {
        let json = include_bytes!("../../../testdata/generate-purchase-order-report.json");
        serde_json::from_slice(json)
            .expect("Failed to deserialize generate-purchase-order-report.json")
    }

    #[test]
    fn test_generate_tissue_60_purchase_order_report() {
        let bytes = include_bytes!("../../../assets/templates/60抽面紙每週訂購單.xlsx");
        let template = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
        let item_needs = deserialize_item_needs();
        let notification_date = Date::new(2025, 10, 21).unwrap();
        let order_number = "10-3";

        let sheet = generate_purchase_order_report(
            &template,
            &[item_needs],
            &Freebie::Tissue60,
            &notification_date,
            order_number,
        )
        .unwrap();

        let worksheet = sheet.get_sheet(&0).unwrap();
        let counts: Vec<f64> = worksheet
            .get_cell_value_by_range("C5:C25")
            .iter()
            .map(|cell| cell.get_value_number().unwrap())
            .collect();
        assert_eq!(
            counts,
            vec![
                20., 0., 50., 0., 0., 30., 990., 0., 0., 0., 0., 0., 0., 0., 0., 50., 0., 0., 100.,
                0., 0.
            ]
        );
        assert_eq!(worksheet.get_value("E2"), "114/10/21");
        assert_eq!(worksheet.get_value("C40"), "訂單編號：10-3");
    }

    #[test]
    fn test_generate_tissue_110_purchase_order_report() {
        let bytes = include_bytes!("../../../assets/templates/110抽面紙每週訂購單.xlsx");
        let template = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
        let item_needs = deserialize_item_needs();
        let notification_date = Date::new(2025, 10, 28).unwrap();
        let order_number = "10-4";

        let sheet = generate_purchase_order_report(
            &template,
            &[item_needs],
            &Freebie::Tissue110,
            &notification_date,
            order_number,
        )
        .unwrap();

        let worksheet = sheet.get_sheet(&0).unwrap();
        let counts: Vec<f64> = worksheet
            .get_cell_value_by_range("C5:C25")
            .iter()
            .map(|cell| cell.get_value_number().unwrap())
            .collect();
        assert_eq!(
            counts,
            vec![
                0., 0., 30., 0., 0., 30., 0., 0., 0., 0., 0., 0., 0., 0., 0., 50., 0., 0., 30., 0.,
                0.
            ]
        );
        assert_eq!(worksheet.get_value("E2"), "114/10/28");
        assert_eq!(worksheet.get_value("D38"), "訂單編號：10-4");
    }

    #[test]
    fn test_generate_mineral_water_purchase_order_report() {
        let bytes = include_bytes!("../../../assets/templates/礦泉水每週訂購單.xlsx");
        let template = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
        let item_needs = deserialize_item_needs();
        let notification_date = Date::new(2025, 10, 14).unwrap();
        let order_number = "10-2";

        let sheet = generate_purchase_order_report(
            &template,
            &[item_needs],
            &Freebie::MineralWater,
            &notification_date,
            order_number,
        )
        .unwrap();

        let worksheet = sheet.get_sheet(&0).unwrap();
        let counts: Vec<f64> = worksheet
            .get_cell_value_by_range("C5:C25")
            .iter()
            .map(|cell| cell.get_value_number().unwrap())
            .collect();
        assert_eq!(
            counts,
            vec![
                60., 0., 60., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 60., 0., 0., 0., 0.,
                0.
            ]
        );
        assert_eq!(worksheet.get_value("F3"), "114/10/14");
        assert_eq!(worksheet.get_value("F2"), "南訂10-2");
    }
}
