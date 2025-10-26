#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod assets;
mod delivery_record;
mod http;
mod purchase_order;
mod view;

use crate::assets::Assets;
use crate::http::HttpClient;
use gpui::prelude::*;
use gpui::{
    Application, AsyncApp, Bounds, KeyBinding, TitlebarOptions, WindowBounds, WindowOptions,
    actions, px, size,
};
use gpui_component::Root;
use std::time::Duration;
use ureq::Agent;
use view::ToolkitView;

actions!(window, [Quit]);

fn main() {
    let application = Application::new().with_assets(Assets);
    application.run(|cx| {
        // Initialize GPUI component
        gpui_component::init(cx);

        // Set locale
        gpui_component::set_locale("zh-HK");

        // Set global states
        let config = Agent::config_builder()
            .user_agent("")
            .timeout_global(Some(Duration::from_secs(5)))
            .build();
        cx.set_global(HttpClient(Agent::new_with_config(config)));

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
                cx.new(|cx| Root::new(ToolkitView::view(window, cx).into(), window, cx))
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
