use gpui::prelude::*;
use gpui::{Window, div};
use gpui_component::tab::{Tab, TabBar};
use gpui_component::v_flex;
use tscred::Client;

pub struct PurchaseOrderView {
    tscred: Client,
    active_tab: usize,
}

impl PurchaseOrderView {
    pub fn new(tscred: Client) -> Self {
        PurchaseOrderView {
            tscred,
            active_tab: 0,
        }
    }

    fn render_tab_content(&self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.active_tab {
            0 => div().child("60"),
            _ => div().child("Unknown content"),
        }
    }
}

impl Render for PurchaseOrderView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_1()
            .child(div().text_lg().child("訂貨通知單"))
            .child(
                TabBar::new("test")
                    .pill()
                    .selected_index(self.active_tab)
                    .on_click(cx.listener(|this, i, _, cx| {
                        this.active_tab = *i;
                        cx.notify();
                    }))
                    .child(Tab::new("60抽面紙"))
                    .child(Tab::new("110抽面紙"))
                    .child(Tab::new("礦泉水"))
            )
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .child(self.render_tab_content(cx))
            )
    }
}
