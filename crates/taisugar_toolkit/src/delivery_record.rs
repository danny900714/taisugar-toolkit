use chrono::{Datelike, Local};
use freebie::Freebie;
use gpui::prelude::*;
use gpui::{App, Entity, SharedString, Window, div};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::calendar::Date;
use gpui_component::date_picker::{DatePicker, DatePickerState};
use gpui_component::form::{form_field, v_form};
use gpui_component::input::{InputState, TextInput};
use gpui_component::tab::{Tab, TabBar};
use gpui_component::{Sizable, v_flex};

pub struct DeliveryRecordView {
    selected_tab_index: usize,
    query_month_input: Entity<InputState>,
    query_month_description: String,
    report_date_picker: Entity<DatePickerState>,
    report_date_description: String,
    submit_button_loading: bool,
}

impl DeliveryRecordView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let now = Local::now().naive_local().date();
        let query_month_input = cx.new(|cx| {
            InputState::new(window, cx)
                .validate(|value, _| {
                    let value = value.parse::<u32>().unwrap_or_default();
                    (1..=12).contains(&value)
                })
                .default_value(SharedString::from(now.month().to_string()))
        });
        let report_date_picker = cx.new(|cx| {
            let mut state = DatePickerState::new(window, cx);
            state.set_date(now, window, cx);
            state
        });

        DeliveryRecordView {
            selected_tab_index: 0,
            query_month_input,
            query_month_description: String::new(),
            report_date_picker,
            report_date_description: String::new(),
            submit_button_loading: false,
        }
    }

    fn validate(&mut self, cx: &mut Context<Self>) -> bool {
        let query_month = self.query_month_input.read(cx).value();
        let report_date = self.report_date_picker.read(cx).date();

        let mut is_valid = true;

        if query_month.is_empty() {
            self.query_month_description = "請輸入統計月份".to_string();
            is_valid = false;
        } else {
            self.query_month_description = String::new();
        }

        if self.selected_tab_index == 2
            && let Date::Single(report_date) = report_date
            && report_date.is_none()
        {
            self.report_date_description = "請選擇報告日期".to_string();
            is_valid = false;
        } else {
            self.report_date_description = String::new();
        }

        is_valid
    }

    fn submit(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.submit_button_loading = true;
        cx.notify();

        if !self.validate(cx) {
            self.submit_button_loading = false;
            return;
        }

        self.submit_button_loading = false;
    }

    fn render_tab_content(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div().py_2().child(
            v_form()
                .column(2)
                .child(
                    form_field()
                        .label("統計月份")
                        .required(true)
                        .when(self.selected_tab_index != 2, |this| this.col_span(2))
                        .when(!self.query_month_description.is_empty(), |this| {
                            this.description(SharedString::from(&self.query_month_description))
                        })
                        .child(TextInput::new(&self.query_month_input).suffix("月")),
                )
                .when(self.selected_tab_index == 2, |this| {
                    this.child(
                        form_field()
                            .label("報告日期")
                            .required(true)
                            .when(!self.report_date_description.is_empty(), |this| {
                                this.description(SharedString::from(&self.report_date_description))
                            })
                            .child(DatePicker::new(&self.report_date_picker).number_of_months(1)),
                    )
                })
                .child(
                    form_field().no_label_indent().col_span(2).child(
                        Button::new("generate-report")
                            .primary()
                            .label("產生交貨統計表")
                            .loading(self.submit_button_loading)
                            .on_click(cx.listener(|this, _, window, cx| this.submit(window, cx))),
                    ),
                ),
        )
    }
}

impl Render for DeliveryRecordView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_2()
            .child(div().text_2xl().child("交貨統計表"))
            .child(
                TabBar::new("delivery-record-tab-bar")
                    .underline()
                    .large()
                    .selected_index(self.selected_tab_index)
                    .on_click(cx.listener(|this, i, _window, cx| {
                        this.selected_tab_index = *i;
                        cx.notify();
                    }))
                    .children(
                        Freebie::all()
                            .iter()
                            .map(|freebie| Tab::new(freebie.name())),
                    ),
            )
            .child(div().flex_1().child(self.render_tab_content(window, cx)))
    }
}
