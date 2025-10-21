use gpui::prelude::*;
use gpui::{App, Entity, Window, div};
use gpui_component::date_picker::{DatePicker, DatePickerState};
use gpui_component::form::{form_field, v_form};
use gpui_component::tab::{Tab, TabBar};
use gpui_component::v_flex;

pub struct PurchaseOrderView {
    active_tab: usize,
    start_date_picker: Entity<DatePickerState>,
}

impl PurchaseOrderView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let start_date_picker = cx.new(|cx| DatePickerState::new(window, cx));

        PurchaseOrderView {
            active_tab: 0,
            start_date_picker,
        }
    }

    fn render_tab_content(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.active_tab {
            0 => div().child(
                v_form().child(
                    form_field()
                        .label("開始日期")
                        .child(DatePicker::new(&self.start_date_picker)),
                ),
            ),
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
                    .child(Tab::new("礦泉水")),
            )
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .child(self.render_tab_content(window, cx)),
            )
    }
}
