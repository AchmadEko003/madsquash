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
        .h(px(64.))
        .px_5()
        .py_3()
        .flex()
        .items_center()
        .gap_4()
        // Search box (stretches across full available width)
        .child(
            div().flex_1().w_full().min_w_0().child(
                Input::new(search_input)
                    .w_full()
                    .prefix(IconName::Search)
                    .cleanable(true)
                    .suffix(
                        div()
                            .text_xs()
                            .text_color(rgb(0xa3988c))
                            .border_1()
                            .border_color(rgb(0x3a342c))
                            .rounded_sm()
                            .px_1()
                            .child("/"),
                    ),
            ),
        )
        // Count Label
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(0xa3988c))
                .child(count_text),
        )
        // "Pasang" Button (Accent #0fc5fc)
        .child(
            div()
                .id("top-install-btn")
                .px_4()
                .py_2()
                .rounded_lg()
                .bg(rgb(0x0fc5fc))
                .text_color(rgb(0x07151a))
                .font_weight(FontWeight::SEMIBOLD)
                .text_sm()
                .cursor_pointer()
                .hover(|s| s.bg(rgb(0x3bd4fd)))
                .active(|s| s.bg(rgb(0x09a5d4)))
                .child("Pasang")
                .on_click(cx.listener(|this, _, _, cx| {
                    this.open_install(cx);
                })),
        )
}
