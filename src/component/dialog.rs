use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

use crate::MadsquashApp;
use crate::component::detail::{render_meta_col, render_meta_wide};
use crate::model::{PendingInstall, format_bytes, mono};

pub fn render_install_dialog(
    pending: Option<PendingInstall>,
    drop_error: Option<String>,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    let has_pending = pending.is_some();
    let is_exists = pending.as_ref().map(|p| p.exists).unwrap_or(false);

    div()
        .id("install-dialog-overlay")
        .absolute()
        .inset_0()
        .bg(rgba(0x04070be6))
        .flex()
        .items_center()
        .justify_center()
        .on_click(cx.listener(|this, _, _, cx| {
            this.close_dialog(cx);
        }))
        .on_drop(cx.listener(|this, paths: &ExternalPaths, _window, cx| {
            this.handle_dropped_paths(paths, cx);
        }))
        .child(
            div()
                .id("install-dialog-content")
                .w(px(460.))
                .p_6()
                .rounded_2xl()
                .bg(rgb(0x0e141d))
                .border_1()
                .border_color(rgb(0x1a2636))
                .shadow_2xl()
                .flex()
                .flex_col()
                .gap_3()
                .on_click(|_, _, cx| cx.stop_propagation())
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_lg()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xf0f6fc))
                                .child("Pasang AppImage"),
                        )
                        .child(
                            div()
                                .id("dialog-close-x")
                                .size(px(24.))
                                .rounded_md()
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(rgb(0x506079))
                                .cursor_pointer()
                                .hover(|s| s.text_color(rgb(0xf0f6fc)))
                                .child("✕")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.close_dialog(cx);
                                })),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x7e8fa6))
                        .child("Berkas disalin ke direktori aplikasi Madsquash, lalu didaftarkan ke launcher OS."),
                )
                .child(match pending {
                    None => render_drop_zone(drop_error, cx).into_any_element(),
                    Some(ref p) => render_install_preview(p, cx).into_any_element(),
                })
                // Dialog buttons
                .child(
                    div()
                        .flex()
                        .justify_end()
                        .gap_2()
                        .mt_3()
                        .child(
                            div()
                                .id("dialog-cancel-btn")
                                .px_4()
                                .py_2()
                                .rounded_xl()
                                .border_1()
                                .border_color(rgb(0x1f2c3e))
                                .bg(rgb(0x111925))
                                .text_color(rgb(0xf0f6fc))
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0x162334)))
                                .child("Batal")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.close_dialog(cx);
                                })),
                        )
                        .when(has_pending, |this| {
                            this.child(
                                div()
                                    .id("dialog-confirm-btn")
                                    .px_5()
                                    .py_2()
                                    .rounded_xl()
                                    .bg(rgb(0x0fc5fc))
                                    .text_color(rgb(0x06151c))
                                    .font_weight(FontWeight::BOLD)
                                    .text_xs()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(0x38d3ff)))
                                    .child(if is_exists { "Ganti Versi Ini" } else { "Pasang ke Menu" })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.confirm_install(cx);
                                    })),
                            )
                        }),
                ),
        )
}

pub fn render_drop_zone(
    drop_error: Option<String>,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    let has_error = drop_error.is_some();
    let headline = drop_error.unwrap_or_else(|| "Jatuhkan berkas .AppImage di sini".to_string());

    div()
        .flex()
        .flex_col()
        .gap_2p5()
        .child(
            div()
                .id("install-dropzone")
                .h(px(140.))
                .rounded_2xl()
                .border_2()
                .when(!has_error, |this| {
                    this.border_dashed()
                        .border_color(rgb(0x1f2c3e))
                        .bg(rgb(0x090e15))
                })
                .when(has_error, |this| {
                    this.border_color(rgb(0xf87171)).bg(rgb(0x1a0d10))
                })
                .cursor_pointer()
                .hover(|s| s.border_color(rgb(0x0fc5fc)).bg(rgb(0x0c1e2b)))
                .drag_over::<ExternalPaths>(|s, _, _, _| {
                    s.border_color(rgb(0x0fc5fc)).bg(rgb(0x0c1e2b))
                })
                .on_drop(cx.listener(|this, paths: &ExternalPaths, _window, cx| {
                    this.handle_dropped_paths(paths, cx);
                }))
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap_1p5()
                .on_click(cx.listener(|this, _, _, cx| {
                    this.browse_file(cx);
                }))
                .child(
                    div()
                        .size(px(36.))
                        .rounded_xl()
                        .bg(rgb(0x101b27))
                        .border_1()
                        .border_color(rgb(0x1c374d))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(rgb(0x0fc5fc))
                        .text_lg()
                        .child("↓"),
                )
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(if has_error {
                            rgb(0xf87171)
                        } else {
                            rgb(0xf0f6fc)
                        })
                        .child(headline),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x7e8fa6))
                        .child("atau klik di sini untuk memilih berkas dari komputer"),
                ),
        )
        .child(
            div()
                .id("sample-kdenlive-btn")
                .cursor_pointer()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(0x0fc5fc))
                .hover(|s| s.text_color(rgb(0x38d3ff)))
                .child("✦ Coba pasang contoh Kdenlive 25.04.0")
                .on_click(cx.listener(|this, _, _, cx| {
                    this.set_sample_pending(cx);
                })),
        )
}

pub fn render_install_preview(
    pending: &PendingInstall,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    div()
        .id("install-preview-zone")
        .flex()
        .flex_col()
        .gap_2p5()
        .mt_1()
        .on_drop(cx.listener(|this, paths: &ExternalPaths, _window, cx| {
            this.handle_dropped_paths(paths, cx);
        }))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_2()
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x7e8fa6))
                        .child(pending.file_name.clone()),
                )
                .child(
                    div()
                        .id("change-file-btn")
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x0fc5fc))
                        .hover(|s| s.text_color(rgb(0x38d3ff)))
                        .child("Ganti berkas")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.browse_file(cx);
                        })),
                ),
        )
        // Extracted app card
        .child(
            div()
                .rounded_xl()
                .bg(rgb(0x111925))
                .border_1()
                .border_color(rgb(0x1a2636))
                .p_3()
                .flex()
                .items_center()
                .gap_3()
                .child(if let Some(ref icon_path) = pending.icon_path {
                    div()
                        .size(px(48.))
                        .rounded_xl()
                        .bg(rgb(0x141c28))
                        .border_1()
                        .border_color(rgb(0x0fc5fc))
                        .p(px(3.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(gpui::img(icon_path.clone()).size(px(42.)))
                } else {
                    div()
                        .size(px(48.))
                        .rounded_xl()
                        .bg(rgb(0x141c28))
                        .border_1()
                        .border_color(rgb(0x0fc5fc))
                        .text_color(rgb(0x0fc5fc))
                        .font_weight(FontWeight::BOLD)
                        .text_lg()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(mono(&pending.name))
                })
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .gap(px(1.))
                        .child(
                            div()
                                .text_base()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xf0f6fc))
                                .child(pending.name.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x7e8fa6))
                                .child(if !pending.comment.is_empty() {
                                    pending.comment.clone()
                                } else {
                                    "Paket Aplikasi AppImage".to_string()
                                }),
                        ),
                ),
        )
        // Metadata Table
        .child(
            div()
                .rounded_xl()
                .bg(rgb(0x111925))
                .border_1()
                .border_color(rgb(0x1a2636))
                .p_3()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .flex()
                        .child(render_meta_col("Versi Terdeteksi", &pending.version))
                        .child(render_meta_col("Ukuran", &format_bytes(pending.size))),
                )
                .child(
                    div()
                        .flex()
                        .child(render_meta_col("Arsitektur", &pending.arch))
                        .child(render_meta_col("Format", &pending.app_type)),
                )
                .when_some(pending.categories.as_ref(), |this, cat| {
                    this.child(render_meta_wide("Kategori", cat))
                })
                .child(render_meta_wide(
                    "Tujuan Pemasangan",
                    &format!("~/.local/share/madsquash/app/{}/", pending.id),
                )),
        )
        .when(pending.exists, |this| {
            this.child(
                div()
                    .p_3()
                    .rounded_xl()
                    .bg(rgb(0x122432))
                    .border_1()
                    .border_color(rgb(0x0fc5fc))
                    .text_xs()
                    .text_color(rgb(0xf0f6fc))
                    .child(format!(
                        "ℹ {} sudah terpasang. Memasang kembali akan menimpa binary yang ada ke versi ini.",
                        pending.name
                    )),
            )
        })
}

pub fn render_remove_dialog(
    app_name: String,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    div()
        .id("remove-dialog-overlay")
        .absolute()
        .inset_0()
        .bg(rgba(0x04070be6))
        .flex()
        .items_center()
        .justify_center()
        .on_click(cx.listener(|this, _, _, cx| {
            this.close_dialog(cx);
        }))
        .child(
            div()
                .id("remove-dialog-content")
                .w(px(420.))
                .p_6()
                .rounded_2xl()
                .bg(rgb(0x0e141d))
                .border_1()
                .border_color(rgb(0x1a2636))
                .shadow_2xl()
                .flex()
                .flex_col()
                .gap_3()
                .on_click(|_, _, cx| cx.stop_propagation())
                .child(
                    div()
                        .size(px(40.))
                        .rounded_xl()
                        .bg(rgb(0x241114))
                        .border_1()
                        .border_color(rgb(0x4c1d24))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(rgb(0xf87171))
                        .text_lg()
                        .child("⚠"),
                )
                .child(
                    div()
                        .text_lg()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xf0f6fc))
                        .child(format!("Lepas {}?", app_name)),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x7e8fa6))
                        .child("Entri launcher OS, ikon, dan berkas salinan AppImage di Madsquash akan dihapus dari sistem."),
                )
                .child(
                    div()
                        .flex()
                        .justify_end()
                        .gap_2()
                        .mt_4()
                        .child(
                            div()
                                .id("remove-cancel-btn")
                                .px_4()
                                .py_2()
                                .rounded_xl()
                                .border_1()
                                .border_color(rgb(0x1f2c3e))
                                .bg(rgb(0x111925))
                                .text_color(rgb(0xf0f6fc))
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0x162334)))
                                .child("Batal")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.close_dialog(cx);
                                })),
                        )
                        .child(
                            div()
                                .id("confirm-remove-btn")
                                .px_5()
                                .py_2()
                                .rounded_xl()
                                .bg(rgb(0xef4444))
                                .text_color(rgb(0xffffff))
                                .font_weight(FontWeight::BOLD)
                                .text_xs()
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0xdc2626)))
                                .child("Lepas Aplikasi")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.confirm_remove(cx);
                                })),
                        ),
                ),
        )
}
