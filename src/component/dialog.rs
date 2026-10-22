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
        .bg(rgba(0x080706bf))
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
                .w(px(440.))
                .p_6()
                .rounded_2xl()
                .bg(rgb(0x1c1a17))
                .border_1()
                .border_color(rgb(0x3a342c))
                .shadow_2xl()
                .flex()
                .flex_col()
                .gap_3()
                .on_click(|_, _, cx| cx.stop_propagation())
                .child(
                    div()
                        .text_lg()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xf4efe8))
                        .child("Pasang AppImage"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0xa3988c))
                        .child("Berkas disalin ke slot, lalu didaftarkan ke menu aplikasi."),
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
                                .rounded_lg()
                                .border_1()
                                .border_color(rgb(0x3a342c))
                                .text_color(rgb(0xf4efe8))
                                .text_sm()
                                .cursor_pointer()
                                .hover(|s| s.bg(rgba(0xffffff0f)))
                                .child("Batal")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.close_dialog(cx);
                                })),
                        )
                        .when(has_pending, |this| {
                            this.child(
                                div()
                                    .id("dialog-confirm-btn")
                                    .px_4()
                                    .py_2()
                                    .rounded_lg()
                                    .bg(rgb(0x0fc5fc))
                                    .text_color(rgb(0x07151a))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_sm()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(0x3bd4fd)))
                                    .child(if is_exists { "Ganti" } else { "Pasang" })
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
    let headline = drop_error.unwrap_or_else(|| "Jatuhkan .AppImage di sini".to_string());

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .id("install-dropzone")
                .h(px(130.))
                .rounded_xl()
                .border_1()
                .when(!has_error, |this| {
                    this.border_dashed().border_color(rgb(0x5c5146))
                })
                .when(has_error, |this| this.border_color(rgb(0x0fc5fc)))
                .bg(rgb(0x141210))
                .cursor_pointer()
                .hover(|s| s.border_color(rgb(0x0fc5fc)).bg(rgb(0x102f3d)))
                .drag_over::<ExternalPaths>(|s, _, _, _| {
                    s.border_color(rgb(0x0fc5fc)).bg(rgb(0x102f3d))
                })
                .on_drop(cx.listener(|this, paths: &ExternalPaths, _window, cx| {
                    this.handle_dropped_paths(paths, cx);
                }))
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap_1()
                .on_click(cx.listener(|this, _, _, cx| {
                    this.browse_file(cx);
                }))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(if has_error {
                            rgb(0x0fc5fc)
                        } else {
                            rgb(0xf4efe8)
                        })
                        .child(headline),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0xa3988c))
                        .child("atau klik untuk memilih berkas"),
                ),
        )
        .child(
            div()
                .id("sample-kdenlive-btn")
                .cursor_pointer()
                .text_xs()
                .text_color(rgb(0x0fc5fc))
                .hover(|s| s.text_color(rgb(0x3bd4fd)))
                .child("Coba dengan Kdenlive 25.04.0")
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
        .gap_2()
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
                        .text_color(rgb(0xa3988c))
                        .child(pending.file_name.clone()),
                )
                .child(
                    div()
                        .id("change-file-btn")
                        .cursor_pointer()
                        .text_xs()
                        .text_color(rgb(0x0fc5fc))
                        .hover(|s| s.text_color(rgb(0x3bd4fd)))
                        .child("Ganti berkas")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.browse_file(cx);
                        })),
                ),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .py_1()
                .child(if let Some(ref icon_path) = pending.icon_path {
                    div()
                        .size(px(44.))
                        .rounded_xl()
                        .bg(rgb(0x141210))
                        .p(px(2.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(gpui::img(icon_path.clone()).size(px(40.)))
                } else {
                    div()
                        .size(px(44.))
                        .rounded_xl()
                        .bg(rgb(0x141210))
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
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_base()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xf4efe8))
                                .child(pending.name.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0xa3988c))
                                .child(if !pending.comment.is_empty() {
                                    pending.comment.clone()
                                } else {
                                    "AppImage".to_string()
                                }),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .border_t_1()
                .border_color(rgb(0x3a342c))
                .child(
                    div()
                        .flex()
                        .child(render_meta_col("Versi", &pending.version))
                        .child(render_meta_col("Ukuran", &format_bytes(pending.size))),
                )
                .child(
                    div()
                        .flex()
                        .child(render_meta_col("Arsitektur", &pending.arch))
                        .child(render_meta_col("Tipe", &pending.app_type)),
                )
                .when_some(pending.categories.as_ref(), |this, cat| {
                    this.child(render_meta_wide("Kategori", cat))
                })
                .child(render_meta_wide(
                    "Tujuan",
                    &format!("~/.local/share/madsquash/apps/{}/", pending.id),
                )),
        )
        .when(pending.exists, |this| {
            this.child(
                div()
                    .mt_2()
                    .p_3()
                    .rounded_lg()
                    .bg(rgb(0x1a2e38))
                    .border_1()
                    .border_color(rgb(0x0fc5fc))
                    .text_xs()
                    .text_color(rgb(0xf4efe8))
                    .child(format!(
                        "{} sudah ada di slot. Memasang lagi mengganti salinan itu, bukan menambah versi kedua.",
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
        .bg(rgba(0x080706bf))
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
                .bg(rgb(0x1c1a17))
                .border_1()
                .border_color(rgb(0x3a342c))
                .shadow_2xl()
                .flex()
                .flex_col()
                .gap_3()
                .on_click(|_, _, cx| cx.stop_propagation())
                .child(
                    div()
                        .text_lg()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xf4efe8))
                        .child(format!("Lepas {}?", app_name)),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0xa3988c))
                        .child("Entri menu, ikon, dan salinan AppImage di Madsquash akan dihapus."),
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
                                .rounded_lg()
                                .border_1()
                                .border_color(rgb(0x3a342c))
                                .text_color(rgb(0xf4efe8))
                                .text_sm()
                                .cursor_pointer()
                                .hover(|s| s.bg(rgba(0xffffff0f)))
                                .child("Batal")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.close_dialog(cx);
                                })),
                        )
                        .child(
                            div()
                                .id("confirm-remove-btn")
                                .px_4()
                                .py_2()
                                .rounded_lg()
                                .bg(rgb(0xf4efe8))
                                .text_color(rgb(0x1a100c))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_sm()
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0xe5ddd1)))
                                .child("Lepas")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.confirm_remove(cx);
                                })),
                        ),
                ),
        )
}
