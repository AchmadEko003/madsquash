use gpui_kit::component::IconName;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::*;

use crate::MadsquashApp;

pub fn render_search_bar(
    search_input: &Entity<InputState>,
    count_text: String,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    div()
        .w_full()
        .h(px(68.))
        .px_6()
        .py_3()
        .bg(rgb(0x0a0e14))
        .border_b_1()
        .border_color(rgb(0x182230))
        .flex()
        .items_center()
        .gap_4()
        // Search box (stretches across full available width)
        .child(
            div()
                .flex_1()
                .w_full()
                .min_w_0()
                .child(
                    Input::new(search_input)
                        .w_full()
                        .prefix(IconName::Search)
                        .cleanable(true)
                        .suffix(
                            div()
                                .text_xs()
                                .text_color(rgb(0x506079))
                                .border_1()
                                .border_color(rgb(0x1e2a3c))
                                .rounded_sm()
                                .px_1p5()
                                .child("/"),
                        ),
                ),
        )
        // Count Pill
        .child(
            div()
                .px_3()
                .py_1p5()
                .rounded_full()
                .bg(rgb(0x0e1b27))
                .border_1()
                .border_color(rgb(0x163447))
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .size(px(6.))
                        .rounded_full()
                        .bg(rgb(0x0fc5fc)),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x0fc5fc))
                        .child(count_text),
                ),
        )
        // "+ Pasang AppImage" Primary Button
        .child(
            div()
                .id("top-install-btn")
                .px_4()
                .py_2()
                .rounded_xl()
                .bg(rgb(0x0fc5fc))
                .text_color(rgb(0x06151c))
                .font_weight(FontWeight::BOLD)
                .text_sm()
                .flex()
                .items_center()
                .gap_1p5()
                .cursor_pointer()
                .hover(|s| s.bg(rgb(0x38d3ff)))
                .active(|s| s.bg(rgb(0x08a6d8)))
                .child(
                    div()
                        .text_base()
                        .line_height(px(16.))
                        .child("+"),
                )
                .child(div().child("Pasang"))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.open_install(cx);
                })),
        )
}
