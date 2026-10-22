use gpui_kit::*;

pub fn render_brand_mark() -> impl IntoElement {
    div()
        .size(px(36.))
        .rounded_xl()
        .bg(rgb(0x182735))
        .border_2()
        .border_color(rgb(0x5bbfe6))
        .p(px(6.))
        .flex()
        .flex_col()
        .justify_between()
        .items_center()
        .child(div().w(px(20.)).h(px(2.5)).bg(rgb(0x5bbfe6)).rounded_full())
        .child(div().w(px(14.)).h(px(2.5)).bg(rgb(0x5bbfe6)).rounded_full())
        .child(div().w(px(8.)).h(px(2.5)).bg(rgb(0x5bbfe6)).rounded_full())
}

pub fn render_rail() -> impl IntoElement {
    div()
        .w(px(68.))
        .h_full()
        .bg(rgb(0x101217))
        .flex()
        .flex_col()
        .items_center()
        .justify_between()
        .py_5()
        .border_r_1()
        .border_color(rgb(0x202630))
        // Top section: Logo brand mark
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_3()
                .child(render_brand_mark()),
        )
        // Bottom section: Arch indicator / version badge
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .size(px(8.))
                        .rounded_full()
                        .bg(rgb(0x6ee7b7))
                        .shadow_sm(),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0x64748b))
                        .child("v0.1"),
                ),
        )
}
