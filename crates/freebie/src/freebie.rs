use umya_spreadsheet::helper::coordinate::CellCoordinates;

pub enum Freebie {
    Tissue60,
    Tissue110,
    MineralWater,
}

impl Freebie {
    pub fn name(&self) -> &'static str {
        match self {
            Freebie::Tissue60 => "60抽盒裝面紙",
            Freebie::Tissue110 => "110抽盒裝面紙",
            Freebie::MineralWater => "台糖礦泉水/箱",
        }
    }

    pub fn notification_date_coord(&self) -> impl Into<CellCoordinates> {
        match self {
            Freebie::Tissue60 => "E2",
            Freebie::Tissue110 => "E2",
            Freebie::MineralWater => "F3",
        }
    }

    pub fn order_number_coord(&self) -> impl Into<CellCoordinates> {
        match self {
            Freebie::Tissue60 => "C40",
            Freebie::Tissue110 => "D38",
            Freebie::MineralWater => "F2",
        }
    }

    pub fn order_number_cell_value<R: AsRef<str>>(&self, order_number: R) -> String {
        match self {
            Freebie::Tissue60 => format!("訂單編號：{}", order_number.as_ref()),
            Freebie::Tissue110 => format!("訂單編號：{}", order_number.as_ref()),
            Freebie::MineralWater => format!("南訂{}", order_number.as_ref()),
        }
    }
}
