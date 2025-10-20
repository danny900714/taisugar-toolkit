use gpui::prelude::*;
use gpui::{
    Application, Bounds, KeyBinding, TitlebarOptions, Window, WindowBounds, WindowOptions, actions,
    div, px, size,
};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::{Root, StyledExt};

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Hello, world!")
            .child(
                Button::new("ok")
                    .primary()
                    .label("Let's Go!")
                    .on_click(|_, _, _| println!("clicked")),
            )
    }
}

actions!(window, [Quit]);

fn main() {
    let application = Application::new();
    application.run(|cx| {
        // Initialize GPUI component
        gpui_component::init(cx);

        // Configure window options
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        let titlebar_options = TitlebarOptions {
            title: Some("台糖工具包".into()),
            ..Default::default()
        };

        // Open a window
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(titlebar_options),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|_| HelloWorld);
                cx.new(|cx| Root::new(view.into(), window, cx))
            },
        )
        .expect("Failed to open window");

        // Add actions
        cx.on_action(|_: &Quit, cx| {
            cx.quit();
        });
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        cx.activate(true);
    });
}
