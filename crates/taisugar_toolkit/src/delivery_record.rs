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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        ""
    }
}
