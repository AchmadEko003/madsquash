use gpui_kit::*;

pub fn render_brand_mark() -> impl IntoElement {
    div()
        .size(px(28.))
        .rounded(px(5.))
        .border_2()
        .border_color(rgb(0x0fc5fc))
        .p(px(4.))
        .flex()
        .flex_col()
        .justify_between()
        .child(div().w_full().h(px(2.)).bg(rgb(0x0fc5fc)).rounded_full())
        .child(div().w_full().h(px(2.)).bg(rgb(0x0fc5fc)).rounded_full())
        .child(div().w(px(10.)).h(px(2.)).bg(rgb(0x0fc5fc)).rounded_full())
}

pub fn render_rail() -> impl IntoElement {
    div()
        .w(px(72.))
        .h_full()
        .flex()
        .flex_col()
        .items_center()
        .gap_3()
        .py_4()
        .border_r_1()
        .border_color(rgb(0x3a342c))
        .child(render_brand_mark())
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(2.))
                .mt_2()
                .children("MADSQUASH".chars().map(|ch| {
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0xf4efe8))
                        .child(ch.to_string())
                })),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap(px(2.))
                .mt_1()
                .children("APP".chars().map(|ch| {
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xa3988c))
                        .child(ch.to_string())
                })),
        )
}
