use gpui_kit::*;

pub fn render_toast(toast_msg: String) -> impl IntoElement {
    div()
        .absolute()
        .bottom(px(24.))
        .w_full()
        .flex()
        .justify_center()
        .child(
            div()
                .bg(rgb(0x182430))
                .border_1()
                .border_color(rgb(0x5bbfe6))
                .text_color(rgb(0xe8edf4))
                .font_weight(FontWeight::SEMIBOLD)
                .text_xs()
                .px_4()
                .py_2p5()
                .rounded_xl()
                .shadow_2xl()
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .size(px(6.))
                        .rounded_full()
                        .bg(rgb(0x5bbfe6)),
                )
                .child(toast_msg),
        )
}
