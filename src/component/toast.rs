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
                .bg(rgb(0x0fc5fc))
                .text_color(rgb(0x07151a))
                .font_weight(FontWeight::SEMIBOLD)
                .text_sm()
                .px_4()
                .py_2()
                .rounded_lg()
                .shadow_lg()
                .child(toast_msg),
        )
}
