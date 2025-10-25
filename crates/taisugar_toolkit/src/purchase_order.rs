use crate::assets::Assets;
use crate::http::HttpClient;
use chrono::{Datelike, Days, Local};
use freebie::Freebie;
use gpui::prelude::*;
use gpui::{App, AsyncApp, Entity, SharedString, WeakEntity, Window, div};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::calendar::{Date, Matcher};
use gpui_component::date_picker::{DatePicker, DatePickerState};
use gpui_component::dropdown::{Dropdown, DropdownItem, DropdownState};
use gpui_component::form::{form_field, v_form};
use gpui_component::input::{InputState, TextInput};
use gpui_component::notification::{Notification, NotificationType};
use gpui_component::tab::{Tab, TabBar};
use gpui_component::{ContextModal, Sizable, v_flex};
use std::env;
use std::io::Cursor;
use std::sync::Arc;
use tscred::{Client, DisplayMode, GetItemNeedsOptions, OperationCenter};
use umya_spreadsheet::{reader, writer};

#[derive(Debug, Clone)]
struct OperationCenterDropdownItem(OperationCenter);

impl DropdownItem for OperationCenterDropdownItem {
    type Value = String;

    fn title(&self) -> SharedString {
        SharedString::from(&self.0.name)
    }

    fn value(&self) -> &Self::Value {
        &self.0.id
    }
}

pub struct PurchaseOrderView {
    active_tab: usize,
    report_date_picker: Entity<DatePickerState>,
    notification_date_picker: Entity<DatePickerState>,
    operation_center_dropdown: Entity<DropdownState<Vec<OperationCenterDropdownItem>>>,
    operation_center_dropdown_disabled: bool,
    operation_center_description: String,
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
        let operation_center_dropdown = cx.new(|cx| DropdownState::new(vec![], None, window, cx));
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

        // Getting operation centers list
        let agent = cx.global::<HttpClient>().0.clone();
        let tscred = Arc::new(Client::new(agent));
        let tscred_clone = Arc::clone(&tscred);
        let window_handle = window.window_handle();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let operation_centers = cx
                .background_spawn(async move { tscred_clone.get_operation_centers() })
                .await;

            match operation_centers {
                Ok(operation_centers) => {
                    this.update(cx, |this, cx| {
                        cx.update_window(window_handle, |_, window, cx| {
                            this.operation_center_dropdown.update(cx, |dropdown, cx| {
                                let items = operation_centers
                                    .into_iter()
                                    .map(OperationCenterDropdownItem)
                                    .collect::<Vec<_>>();
                                let tainan = items
                                    .iter()
                                    .find(|item| item.0.name == "台南區業務組")
                                    .map(|item| item.0.id.clone());

                                dropdown.set_items(items, window, cx);
                                if let Some(tainan) = tainan {
                                    dropdown.set_selected_value(&tainan, window, cx);
                                }
                            });
                        })
                        .unwrap();

                        this.operation_center_dropdown_disabled = false;
                    })
                    .unwrap();
                }
                Err(err) => {
                    cx.update_window(window_handle, |_, window, cx| {
                        window.push_notification(
                            Notification::new()
                                .with_type(NotificationType::Error)
                                .message(SharedString::from(format!(
                                    "無法從紅網取得營運中心清單\n{:?}",
                                    err
                                ))),
                            cx,
                        );
                    })
                    .unwrap();
                }
            }
        })
        .detach();

        PurchaseOrderView {
            active_tab: 0,
            report_date_picker,
            notification_date_picker,
            operation_center_dropdown,
            operation_center_dropdown_disabled: true,
            operation_center_description: String::new(),
            order_number_input,
            order_number_description: String::new(),
            submit_button_loading: false,
            tscred,
        }
    }

    fn get_active_freebie(&self) -> Option<Freebie> {
        match self.active_tab {
            0 => Some(Freebie::Tissue60),
            1 => Some(Freebie::Tissue110),
            2 => Some(Freebie::MineralWater),
            _ => None,
        }
    }

    fn get_template_path(freebie: &Freebie) -> &'static str {
        match freebie {
            Freebie::Tissue60 => "templates/60抽面紙每週訂購單.xlsx",
            Freebie::Tissue110 => "templates/110抽面紙每週訂購單.xlsx",
            Freebie::MineralWater => "templates/礦泉水每週訂購單.xlsx",
        }
    }

    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Show the loading state of submit button
        self.submit_button_loading = true;
        cx.notify();

        let report_date = self.report_date_picker.read(cx).date();
        let notification_date = self.notification_date_picker.read(cx).date();
        let operation_center_id = self.operation_center_dropdown.read(cx).selected_value();
        let order_number = self.order_number_input.read(cx).value();

        // Validate report date
        let start_date: jiff::civil::Date;
        let end_date: jiff::civil::Date;
        match report_date {
            Date::Range(start, end) => {
                if start.is_none() || end.is_none() {
                    self.submit_button_loading = false;
                    cx.notify();
                    return;
                }

                start_date = start.unwrap().to_string().parse().unwrap();
                end_date = end.unwrap().to_string().parse().unwrap();
            }
            _ => {
                self.submit_button_loading = false;
                cx.notify();
                return;
            }
        }

        // Validate operation center picker
        if operation_center_id.is_none() || operation_center_id.unwrap().is_empty() {
            self.operation_center_description = "請選擇營運中心".to_string();
            self.submit_button_loading = false;
            cx.notify();
            return;
        }

        // Validate order number
        if order_number.is_empty() {
            self.order_number_description = "請輸入訂單編號".to_string();
            self.submit_button_loading = false;
            cx.notify();
            return;
        } else {
            self.order_number_description = String::new();
        }

        // Create variables for the async tasks
        let window_handle = window.window_handle();
        let tscred = self.tscred.clone();
        let operation_center_id = operation_center_id.unwrap().clone();
        let active_freebie = self.get_active_freebie().unwrap();
        let active_freebie_name = active_freebie.name();
        let template_path = Self::get_template_path(&active_freebie);
        let notification_date = notification_date.to_string().parse().unwrap();

        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let item_needs_result = cx
                .background_spawn(async move {
                    tscred.get_item_needs(GetItemNeedsOptions {
                        operation_center_id: &operation_center_id,
                        start_date: &start_date,
                        end_date: &end_date,
                        display_mode: &DisplayMode::Details,
                        department_id: "2",
                    })
                })
                .await;

            // Handle errors when retrieving item needs
            if item_needs_result.is_err() {
                let _ = cx.update_window(window_handle, |_this, window, cx| {
                    window.push_notification(
                        Notification::new()
                            .with_type(NotificationType::Error)
                            .message(SharedString::from(format!(
                                "無法從紅網取得贈品需求資料\n{:?}",
                                item_needs_result.err().unwrap()
                            ))),
                        cx,
                    );
                });
                this.update(cx, |this, cx| {
                    this.submit_button_loading = false;
                    cx.notify();
                })
                .unwrap();
                return;
            }
            let item_needs = item_needs_result.unwrap();

            // Generate the purchase order report
            let spreadsheet = cx
                .background_spawn(async move {
                    let template_file = Assets::get(template_path).unwrap();
                    let template =
                        reader::xlsx::read_reader(Cursor::new(template_file.data), true).unwrap();
                    freebie::generate_purchase_order_report(
                        &template,
                        &item_needs,
                        &active_freebie,
                        &notification_date,
                        &order_number,
                    )
                    .unwrap()
                })
                .await;

            // Retrieve the path to save the report
            let paths_receiver = cx
                .update(|cx| {
                    cx.prompt_for_new_path(env::home_dir().unwrap().as_path(), Some("活頁簿.xlsx"))
                })
                .unwrap();
            let path_buf_option = cx.background_spawn(paths_receiver).await.unwrap().unwrap();
            if let Some(path_buf) = path_buf_option {
                let path_string = path_buf.to_string_lossy().to_string();

                // Save the generated repor to the specified path
                let write_result = cx
                    .background_spawn(async move { writer::xlsx::write(&spreadsheet, path_buf) })
                    .await;

                // Show the notification to the user about the result of the save operation
                let _ = cx.update_window(window_handle, |_this, window, cx| {
                    let notification = match write_result {
                        Ok(_) => Notification::new()
                            .with_type(NotificationType::Success)
                            .message(format!(
                                "已將{}每週訂購單儲存到 {}",
                                active_freebie_name, path_string
                            )),
                        Err(error) => Notification::new()
                            .with_type(NotificationType::Error)
                            .message(format!(
                                "無法將{}每週訂購單儲存到 {}\nError: {}",
                                active_freebie_name, path_string, error
                            )),
                    };
                    window.push_notification(notification, cx);
                });
            }

            // Reset the submit button loading state
            this.update(cx, |this, cx| {
                this.submit_button_loading = false;
                cx.notify();
            })
            .unwrap();
        })
        .detach();
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
                        .label("營運中心")
                        .required(true)
                        .when(!self.operation_center_description.is_empty(), |this| {
                            this.description(SharedString::from(&self.operation_center_description))
                        })
                        .child(
                            Dropdown::new(&self.operation_center_dropdown)
                                .disabled(self.operation_center_dropdown_disabled),
                        ),
                )
                .child(
                    form_field()
                        .label("訂單編號")
                        .required(true)
                        .when(!self.order_number_description.is_empty(), |this| {
                            this.description(SharedString::from(&self.order_number_description))
                        })
                        .child(TextInput::new(&self.order_number_input)),
                )
                .child(
                    form_field().no_label_indent().col_span(2).child(
                        Button::new("generate-report")
                            .primary()
                            .label("產生訂購單")
                            .loading(self.submit_button_loading)
                            .on_click(cx.listener(|this, _, window, cx| this.submit(window, cx))),
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
