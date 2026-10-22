use gpui_kit::*;

use crate::MadsquashApp;
use crate::model::{AppItem, format_bytes, mono};

pub fn render_app_card(
    app: &AppItem,
    is_selected: bool,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement + use<> {
    let app_id = app.id.clone();
    let app_name = app.name.clone();
    let monogram = mono(&app.name);

    let ver_label = if app.version != "—" && !app.version.starts_with('v') && !app.version.starts_with('V') {
        format!("v{}", app.version)
    } else {
        app.version.clone()
    };

    div()
        .id(format!("card-{}", app.id))
        .w(px(216.))
        .h(px(150.))
        .p_4()
        .rounded_2xl()
        .border_1()
        .border_color(if is_selected {
            rgb(0x5bbfe6)
        } else {
            rgb(0x222a36)
        })
        .bg(if is_selected {
            rgb(0x162533)
        } else {
            rgb(0x171c24)
        })
        .hover(|s| {
            if !is_selected {
                s.bg(rgb(0x1c222c)).border_color(rgb(0x2e3a4b))
            } else {
                s
            }
        })
        .cursor_pointer()
        .flex()
        .flex_col()
        .justify_between()
        .on_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
            this.select_app(app_id.clone(), cx);
            if event.click_count() >= 2 {
                this.launch_app(&app_id, cx);
            }
        }))
        // Top row: Icon on left, version pill on right
        .child(
            div()
                .flex()
                .items_start()
                .justify_between()
                .child(if let Some(ref icon_path) = app.icon_path {
                    div()
                        .size(px(44.))
                        .rounded_xl()
                        .bg(rgb(0x1b222c))
                        .border_1()
                        .border_color(if is_selected {
                            rgb(0x5bbfe6)
                        } else {
                            rgb(0x283240)
                        })
                        .p(px(3.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(gpui::img(icon_path.clone()).size(px(36.)))
                } else {
                    div()
                        .size(px(44.))
                        .rounded_xl()
                        .bg(rgb(0x1b222c))
                        .border_1()
                        .border_color(if is_selected {
                            rgb(0x5bbfe6)
                        } else {
                            rgb(0x283240)
                        })
                        .text_color(rgb(0x7bc7e8))
                        .font_weight(FontWeight::BOLD)
                        .text_sm()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(monogram)
                })
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded_full()
                        .bg(if is_selected {
                            rgb(0x1d3345)
                        } else {
                            rgb(0x1c2430)
                        })
                        .border_1()
                        .border_color(if is_selected {
                            rgb(0x5bbfe6)
                        } else {
                            rgb(0x2a384a)
                        })
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(if is_selected {
                            rgb(0x7bc7e8)
                        } else {
                            rgb(0x94a3b8)
                        })
                        .child(ver_label),
                ),
        )
        // Bottom: App name and size/arch info
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(2.))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xe8edf4))
                        .child(app_name),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x8a99ad))
                        .child(format!("{} · {}", format_bytes(app.size), app.arch)),
                ),
        )
}

pub fn render_list_pane(
    visible: &[&AppItem],
    selected_id: Option<&str>,
    query_is_empty: bool,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    div()
        .flex_1()
        .h_full()
        .flex()
        .flex_col()
        .px_6()
        .pt_4()
        .pb_4()
        .overflow_hidden()
        // Scrollable Cards or Empty State
        .child(
            div()
                .id("cards-scroll-container")
                .flex_1()
                .overflow_y_scroll()
                .child(if visible.is_empty() {
                    if query_is_empty {
                        render_empty_apps_state(cx).into_any_element()
                    } else {
                        render_empty_search_state().into_any_element()
                    }
                } else {
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_3p5()
                        .pb_4()
                        .children(visible.iter().map(|app| {
                            let is_selected = selected_id == Some(app.id.as_str());
                            render_app_card(app, is_selected, cx)
                        }))
                        .into_any_element()
                }),
        )
}

fn render_empty_apps_state(cx: &mut Context<MadsquashApp>) -> impl IntoElement {
    div()
        .w_full()
        .min_h(px(400.))
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_3p5()
        .py_12()
        .px_4()
        .child(
            div()
                .size(px(72.))
                .rounded_2xl()
                .bg(rgb(0x182433))
                .border_1()
                .border_color(rgb(0x283d54))
                .flex()
                .items_center()
                .justify_center()
                .text_2xl()
                .text_color(rgb(0x7bc7e8))
                .child("✦"),
        )
        .child(
            div()
                .text_base()
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0xf1f5f9))
                .child("Belum Ada Aplikasi Terpasang"),
        )
        .child(
            div()
                .text_xs()
                .text_color(rgb(0x8a99ad))
                .text_center()
                .max_w(px(380.))
                .child("Tarik & jatuhkan berkas .AppImage langsung ke jendela ini, atau klik tombol di bawah untuk memasang aplikasi pertama Anda."),
        )
        .child(
            div()
                .id("empty-state-install-btn")
                .mt_2()
                .px_5()
                .py_2p5()
                .rounded_xl()
                .bg(rgb(0x5bbfe6))
                .hover(|s| s.bg(rgb(0x70c9ec)))
                .active(|s| s.bg(rgb(0x4baed7)))
                .text_color(rgb(0x0c141d))
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .cursor_pointer()
                .shadow_md()
                .child("+ Pasang Berkas AppImage")
                .on_click(cx.listener(|this, _, _, cx| {
                    this.open_install(cx);
                })),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .justify_center()
                .gap_2()
                .mt_4()
                .child(render_empty_pill("⚡ Ekstraksi Cepat SquashFS"))
                .child(render_empty_pill("🚀 Integrasi Menu & Desktop"))
                .child(render_empty_pill("🔒 Aman & Terisolasi")),
        )
}

fn render_empty_search_state() -> impl IntoElement {
    div()
        .w_full()
        .min_h(px(320.))
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_3()
        .py_12()
        .px_4()
        .child(
            div()
                .size(px(56.))
                .rounded_2xl()
                .bg(rgb(0x171c24))
                .border_1()
                .border_color(rgb(0x222a36))
                .flex()
                .items_center()
                .justify_center()
                .text_xl()
                .text_color(rgb(0x64748b))
                .child("🔍"),
        )
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0xe8edf4))
                .child("Tidak Ada Aplikasi yang Cocok"),
        )
        .child(
            div()
                .text_xs()
                .text_color(rgb(0x8a99ad))
                .child("Coba gunakan kata kunci pencarian lain."),
        )
}

fn render_empty_pill(text: &'static str) -> impl IntoElement {
    div()
        .px_3()
        .py_1()
        .rounded_full()
        .bg(rgb(0x161c24))
        .border_1()
        .border_color(rgb(0x222a36))
        .text_xs()
        .text_color(rgb(0x728296))
        .child(text)
}
