use gpui::prelude::*;
use gpui::{Window, div};
use gpui_component::v_flex;

pub struct PurchaseOrderView {}

impl PurchaseOrderView {
    pub fn new() -> Self {
        PurchaseOrderView {}
    }
}

impl Render for PurchaseOrderView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_2()
            .child(div().text_lg().child("訂貨通知單"))
    }
}
