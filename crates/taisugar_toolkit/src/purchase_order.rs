use chrono::{Datelike, Days, Local};
use gpui::prelude::*;
use gpui::{App, AsyncApp, Entity, SharedString, WeakEntity, Window, div};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::calendar::{Date, Matcher};
use gpui_component::date_picker::{DatePicker, DatePickerState};
use gpui_component::form::{form_field, v_form};
use gpui_component::input::{InputState, TextInput};
use gpui_component::tab::{Tab, TabBar};
use gpui_component::{Sizable, v_flex};
use std::sync::Arc;
use tscred::Client;

pub struct PurchaseOrderView {
    active_tab: usize,
    report_date_picker: Entity<DatePickerState>,
    notification_date_picker: Entity<DatePickerState>,
    order_number_input: Entity<InputState>,
    order_number_description: String,
    submit_button_loading: bool,
    tscred: Arc<Client>,
}

impl PurchaseOrderView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let now = Local::now().naive_local().date();
        let report_date_picker = cx.new(|cx| {
            let start = now.checked_sub_days(Days::new(7)).unwrap();

            // Disable the dates after today
            let mut state = DatePickerState::range(window, cx).disabled_matcher(Matcher::range(
                Some(now.checked_add_days(Days::new(1)).unwrap()),
                None,
            ));

            // Set the default date range to the last 7 days
            state.set_date((start, now), window, cx);

            state
        });
        let notification_date_picker = cx.new(|cx| {
            let mut state = DatePickerState::new(window, cx);

            // Set the default date to today
            state.set_date(now, window, cx);

            state
        });
        let order_number_input = cx.new(|cx| {
            let n_week_in_month = (now.day() - 1) / 7 + 1;
            InputState::new(window, cx).default_value(format!(
                "{}-{}",
                now.month(),
                n_week_in_month
            ))
        });

        PurchaseOrderView {
            active_tab: 0,
            report_date_picker,
            notification_date_picker,
            order_number_input,
            order_number_description: String::new(),
            submit_button_loading: false,
            tscred: Arc::new(Client::new()),
        }
    }

    fn submit(&mut self, cx: &mut Context<Self>) {
        let report_date = self.report_date_picker.read(cx).date();
        let notification_date = self.notification_date_picker.read(cx).date();
        let order_number = self.order_number_input.read(cx).value();

        // Validate order number
        if order_number.is_empty() {
            self.order_number_description = "請輸入訂單編號".to_string();
            return;
        } else {
            self.order_number_description = String::new();
        }

        let start_date: jiff::civil::Date;
        let end_date: jiff::civil::Date;
        match report_date {
            Date::Range(start, end) => {
                start_date = start.unwrap().to_string().parse().unwrap();
                end_date = end.unwrap().to_string().parse().unwrap();
            }
            Date::Single(date) => {
                // TODO: Handle error
                return;
            }
        }

        let client = Arc::clone(&cx.http_client());
        cx.spawn(|this: WeakEntity<Self>, cx: &mut AsyncApp| async move {

        }).detach();
    }

    fn render_tab_content(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().py_2().child(
            v_form()
                .column(2)
                .child(
                    form_field()
                        .label("統計日期區間")
                        .required(true)
                        .child(DatePicker::new(&self.report_date_picker)),
                )
                .child(
                    form_field()
                        .label("通知日期")
                        .required(true)
                        .child(DatePicker::new(&self.notification_date_picker).number_of_months(1)),
                )
                .child(
                    form_field()
                        .label("訂單編號")
                        .required(true)
                        .col_span(2)
                        .when(!self.order_number_description.is_empty(), |this| {
                            this.description(SharedString::from(&self.order_number_description))
                        })
                        .child(TextInput::new(&self.order_number_input)),
                )
                .child(
                    form_field().no_label_indent().col_span(3).child(
                        Button::new("generate-report")
                            .primary()
                            .label("產生訂購單")
                            .loading(self.submit_button_loading)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.submit_button_loading = true;
                                this.submit(cx);
                                this.submit_button_loading = false;
                            })),
                    ),
                ),
        )
    }
}

impl Render for PurchaseOrderView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_2()
            .child(div().text_2xl().child("訂貨通知單"))
            .child(
                TabBar::new("test")
                    .underline()
                    .large()
                    .selected_index(self.active_tab)
                    .on_click(cx.listener(|this, i, _, cx| {
                        this.active_tab = *i;
                        cx.notify();
                    }))
                    .child(Tab::new("60抽面紙"))
                    .child(Tab::new("110抽面紙"))
                    .child(Tab::new("礦泉水")),
            )
            .child(div().flex_1().child(self.render_tab_content(window, cx)))
    }
}
