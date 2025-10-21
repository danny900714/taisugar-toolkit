mod view;
mod purchase_order;
mod delivery_record;

use gpui::prelude::*;
use gpui::{
    Application, AsyncApp, Bounds, KeyBinding, TitlebarOptions, WindowBounds, WindowOptions,
    actions, px, size,
};
use gpui_component::Root;
use tscred::Client;
use view::ToolkitView;
use crate::purchase_order::PurchaseOrderView;

actions!(window, [Quit]);

fn main() {
    let application = Application::new();
    application.run(|cx| {
        // Initialize GPUI component
        gpui_component::init(cx);

        // Configure window options
        let bounds = Bounds::centered(None, size(px(1280.), px(720.)), cx);
        let titlebar_options = TitlebarOptions {
            title: Some("台糖工具包".into()),
            ..Default::default()
        };
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(titlebar_options),
            ..Default::default()
        };

        cx.spawn(async move |cx: &mut AsyncApp| {
            cx.open_window(window_options, |window, cx| {
                let purchase_order_view = cx.new(|_| PurchaseOrderView::new(Client::new()));
                let toolkit_view = cx.new(|_| ToolkitView::new(purchase_order_view));
                cx.new(|cx| Root::new(toolkit_view.into(), window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();

        cx.bind_keys([
            #[cfg(target_os = "macos")]
            KeyBinding::new("cmd-q", Quit, None),
            #[cfg(not(target_os = "macos"))]
            KeyBinding::new("alt-f4", Quit, None),
        ]);

        cx.on_action(|_: &Quit, cx| {
            cx.quit();
        });

        cx.activate(true);
    });
}
