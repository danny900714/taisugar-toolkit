use gpui::prelude::*;
use gpui::{ClickEvent, Window, div, relative};
use gpui_component::sidebar::{Sidebar, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem};
use gpui_component::{ActiveTheme, Side, h_flex, v_flex};

pub struct ToolkitView {
    active_item: MenuItem,
}

impl ToolkitView {
    pub fn new() -> Self {
        ToolkitView {
            active_item: MenuItem::PurchaseOrderNotice,
        }
    }

    fn render_content(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().child(match self.active_item {
            MenuItem::PurchaseOrderNotice => self.active_item.label(),
            MenuItem::DeliveryRecordSheet => self.active_item.label(),
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
        h_flex()
            .rounded(cx.theme().radius)
            .border_1()
            .border_color(cx.theme().border)
            .h_full()
            .child(
                Sidebar::new(Side::Left)
                    .header(
                        SidebarHeader::new().w_full().child(
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
            )
    }
}
