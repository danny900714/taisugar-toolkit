use crate::delivery_record::DeliveryRecordView;
use crate::purchase_order::PurchaseOrderView;
use gpui::prelude::*;
use gpui::{App, ClickEvent, Entity, Window, div, img, relative};
use gpui_component::sidebar::{Sidebar, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem};
use gpui_component::{ActiveTheme, Root, Side, h_flex, v_flex};

pub struct ToolkitView {
    active_item: MenuItem,
    purchase_order_view: Entity<PurchaseOrderView>,
}

impl ToolkitView {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let purchase_order_view = PurchaseOrderView::view(window, cx);

        ToolkitView {
            active_item: MenuItem::PurchaseOrderNotice,
            purchase_order_view,
        }
    }

    fn render_content(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .when(self.active_item == MenuItem::PurchaseOrderNotice, |this| {
                this.child(self.purchase_order_view.clone())
            })
            .when(self.active_item == MenuItem::DeliveryRecordSheet, |this| {
                this.child(cx.new(|_| DeliveryRecordView::new()))
            })
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum MenuItem {
    PurchaseOrderNotice,
    DeliveryRecordSheet,
}

impl MenuItem {
    fn all() -> [Self; 2] {
        [MenuItem::PurchaseOrderNotice, MenuItem::DeliveryRecordSheet]
    }

    fn label(&self) -> &'static str {
        match self {
            MenuItem::PurchaseOrderNotice => "訂貨通知單",
            MenuItem::DeliveryRecordSheet => "交貨統計表",
        }
    }

    fn handler(
        &self,
    ) -> impl Fn(&mut ToolkitView, &ClickEvent, &mut Window, &mut Context<ToolkitView>) + 'static
    {
        let item = *self;
        move |this, _, _, cx| {
            this.active_item = item;
            cx.notify();
        }
    }
}

impl Render for ToolkitView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notification_layer = Root::render_notification_layer(window, cx);

        div().size_full().children(notification_layer).child(
            h_flex()
                .rounded(cx.theme().radius)
                .border_1()
                .border_color(cx.theme().border)
                .h_full()
                .child(
                    Sidebar::new(Side::Left)
                        .header(
                            SidebarHeader::new()
                                .w_full()
                                .child(img("images/taisugar.svg").size_8())
                                .child(
                                    v_flex()
                                        .gap_0()
                                        .text_sm()
                                        .flex_1()
                                        .line_height(relative(1.25))
                                        .overflow_hidden()
                                        .text_ellipsis()
                                        .child("台灣糖業股份有限公司")
                                        .child(div().child("油品事業部").text_xs()),
                                ),
                        )
                        .child(SidebarGroup::new("贈品").child(SidebarMenu::new().children(
                            MenuItem::all().iter().map(|item| {
                                SidebarMenuItem::new(item.label())
                                    .active(item == &self.active_item)
                                    .on_click(cx.listener(item.handler()))
                            }),
                        ))),
                )
                .child(
                    v_flex()
                        .size_full()
                        .gap_4()
                        .p_4()
                        .child(self.render_content(window, cx)),
                ),
        )
    }
}
