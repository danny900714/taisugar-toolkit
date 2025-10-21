use gpui::prelude::*;
use gpui::Window;

pub struct DeliveryRecordView {
}

impl DeliveryRecordView {
    pub fn new() -> Self {
        DeliveryRecordView {
        }
    }
}

impl Render for DeliveryRecordView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ""
    }
}
