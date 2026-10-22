mod component;
mod model;

use std::path::PathBuf;
use std::time::Duration;

use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::component::theme::{Theme, ThemeMode};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

use crate::component::{
    card::render_list_pane,
    detail::render_detail_container,
    dialog::{render_install_dialog, render_remove_dialog},
    rail::render_rail,
    search::render_search_bar,
    toast::render_toast,
};
use crate::model::{
    AppItem, DialogKind, PendingInstall, app_storage_dir, compute_file_hash,
    inspect_appimage_metadata, install_app_files, launch_app, load_installed_apps,
    save_installed_apps, seed_apps, slug, uninstall_app_files,
};

pub struct MadsquashApp {
    apps: Vec<AppItem>,
    selected_id: Option<String>,
    query: String,
    search_input: Entity<InputState>,
    dialog: Option<DialogKind>,
    pending: Option<PendingInstall>,
    drop_error: Option<String>,
    toast: Option<String>,
    toast_seq: usize,
}

impl MadsquashApp {
    pub fn new(search_input: Entity<InputState>, cx: &mut Context<Self>) -> Self {
        let apps = load_installed_apps();
        let selected_id = apps.first().map(|a| a.id.clone());

        cx.subscribe(
            &search_input,
            |this: &mut Self, input_state, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Change) {
                    this.query = input_state.read(cx).text().to_string();
                    cx.notify();
                }
            },
        )
        .detach();

        Self {
            apps,
            selected_id,
            query: String::new(),
            search_input,
            dialog: None,
            pending: None,
            drop_error: None,
            toast: None,
            toast_seq: 0,
        }
    }

    pub fn visible_apps(&self) -> Vec<AppItem> {
        let q = self.query.trim().to_lowercase();
        if q.is_empty() {
            return self.apps.clone();
        }
        self.apps
            .iter()
            .filter(|app| {
                app.name.to_lowercase().contains(&q)
                    || app.comment.to_lowercase().contains(&q)
                    || app.version.to_lowercase().contains(&q)
            })
            .cloned()
            .collect()
    }

    pub fn selected_app(&self) -> Option<AppItem> {
        let id = self.selected_id.as_ref()?;
        self.apps.iter().find(|a| &a.id == id).cloned()
    }

    pub fn select_app(&mut self, id: String, cx: &mut Context<Self>) {
        self.selected_id = Some(id);
        cx.notify();
    }

    pub fn show_toast(&mut self, text: impl Into<String>, cx: &mut Context<Self>) {
        self.toast_seq += 1;
        let seq = self.toast_seq;
        self.toast = Some(text.into());
        cx.notify();

        cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(2500))
                .await;
            this.update(cx, |this, cx| {
                if this.toast_seq == seq {
                    this.toast = None;
                    cx.notify();
                }
            })
            .ok();
        })
        .detach();
    }

    pub fn open_install(&mut self, cx: &mut Context<Self>) {
        self.dialog = Some(DialogKind::Install);
        self.pending = None;
        self.drop_error = None;
        cx.notify();
    }

    pub fn open_remove(&mut self, cx: &mut Context<Self>) {
        if self.selected_app().is_some() {
            self.dialog = Some(DialogKind::Remove);
            cx.notify();
        }
    }

    pub fn close_dialog(&mut self, cx: &mut Context<Self>) {
        self.dialog = None;
        self.pending = None;
        self.drop_error = None;
        cx.notify();
    }

    pub fn handle_dropped_paths(&mut self, paths: &ExternalPaths, cx: &mut Context<Self>) {
        if let Some(path) = paths.paths().first() {
            self.take_file_path(path.clone(), cx);
        }
    }

    pub fn browse_file(&mut self, cx: &mut Context<Self>) {
        let rx = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Pilih berkas .AppImage".into()),
        });

        cx.spawn(async move |this, cx| {
            if let Ok(Ok(Some(paths))) = rx.await
                && let Some(path) = paths.into_iter().next()
            {
                this.update(cx, |this, cx| {
                    this.take_file_path(path, cx);
                })
                .ok();
            }
        })
        .detach();
    }

    pub fn take_file_path(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let is_appimage =
            file_name.len() > 9 && file_name[file_name.len() - 9..].eq_ignore_ascii_case(".appimage");

        self.dialog = Some(DialogKind::Install);

        if !is_appimage {
            self.pending = None;
            self.drop_error = Some("Berkas ini bukan .AppImage.".to_string());
            cx.notify();
            return;
        }

        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        let details = inspect_appimage_metadata(&path, &file_name);
        let id = slug(&details.name);
        let exists = self.apps.iter().any(|a| a.id == id);

        self.drop_error = None;
        self.pending = Some(PendingInstall {
            comment: details.comment,
            id,
            name: details.name,
            version: details.version,
            size,
            file_name,
            arch: details.arch,
            app_type: details.app_type,
            exists,
            source_path: Some(path),
            icon_path: details.icon_path,
            categories: details.categories,
            exec: details.exec,
        });
        cx.notify();
    }

    pub fn set_sample_pending(&mut self, cx: &mut Context<Self>) {
        let id = "kdenlive".to_string();
        let exists = self.apps.iter().any(|a| a.id == id);
        self.drop_error = None;
        self.pending = Some(PendingInstall {
            id,
            name: "Kdenlive".into(),
            version: "25.04.0".into(),
            size: 327155712,
            file_name: "Kdenlive-25.04.0-x86_64.AppImage".into(),
            comment: "Penyunting video".into(),
            arch: "x86_64".into(),
            app_type: "AppImage 2".into(),
            exists,
            source_path: None,
            icon_path: None,
            categories: Some("AudioVideo;Video;Recorder;".into()),
            exec: Some("kdenlive %U".into()),
        });
        cx.notify();
    }

    pub fn confirm_install(&mut self, cx: &mut Context<Self>) {
        let Some(pending) = self.pending.clone() else {
            return;
        };

        match install_app_files(&pending) {
            Ok(_) => {
                let seed_str = format!("{}|{}|{}", pending.name, pending.size, pending.file_name);
                let sha256 = compute_file_hash(pending.source_path.as_deref(), &seed_str);

                let new_app = AppItem {
                    id: pending.id.clone(),
                    name: pending.name.clone(),
                    comment: pending.comment,
                    version: pending.version,
                    size: pending.size,
                    arch: pending.arch,
                    app_type: pending.app_type,
                    sha256,
                    installed_at: "22 Okt 2026".into(),
                    categories: pending.categories,
                    exec: pending.exec,
                    icon_path: pending.icon_path,
                };

                self.apps.retain(|a| a.id != pending.id);
                self.apps.insert(0, new_app);
                self.selected_id = Some(pending.id);
                self.dialog = None;
                self.pending = None;
                self.drop_error = None;
                save_installed_apps(&self.apps);

                self.show_toast(format!("{} dipasang ke menu", pending.name), cx);
            }
            Err(e) => {
                self.drop_error = Some(e);
                cx.notify();
            }
        }
    }

    pub fn confirm_remove(&mut self, cx: &mut Context<Self>) {
        let Some(selected) = self.selected_app() else {
            return;
        };
        let name = selected.name;
        let id = selected.id;

        let _ = uninstall_app_files(&id);
        self.apps.retain(|a| a.id != id);
        self.selected_id = self.apps.first().map(|a| a.id.clone());
        self.dialog = None;
        save_installed_apps(&self.apps);
        self.show_toast(format!("{} dilepas", name), cx);
    }

    pub fn launch_app(&mut self, id: &str, cx: &mut Context<Self>) {
        let app_name = self
            .apps
            .iter()
            .find(|a| a.id == id)
            .map(|a| a.name.clone())
            .unwrap_or_else(|| "Aplikasi".to_string());

        match launch_app(id) {
            Ok(()) => {
                self.show_toast(format!("Membuka {}", app_name), cx);
            }
            Err(e) => {
                self.show_toast(e, cx);
            }
        }
    }

    pub fn open_app_folder(&mut self, id: &str, cx: &mut Context<Self>) {
        let folder = app_storage_dir(id);
        if folder.exists() {
            let _ = std::process::Command::new("xdg-open")
                .arg(&folder)
                .spawn();
            self.show_toast("Membuka folder aplikasi...", cx);
        } else {
            self.show_toast("Folder belum ada di disk (pasang berkas terlebih dahulu)", cx);
        }
    }

    pub fn reset_seed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.apps = seed_apps();
        save_installed_apps(&self.apps);
        self.selected_id = self.apps.first().map(|a| a.id.clone());
        self.query.clear();
        self.search_input
            .update(cx, |s, cx| s.set_value("", window, cx));
        self.show_toast("Contoh dikembalikan", cx);
    }
}

impl Render for MadsquashApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let visible = self.visible_apps();
        let total = self.apps.len();
        let query_is_empty = self.query.trim().is_empty();
        let count_text = if query_is_empty {
            if total > 0 {
                format!("{} terpasang", total)
            } else {
                "Kosong".to_string()
            }
        } else {
            format!("{} dari {}", visible.len(), total)
        };

        let selected = self.selected_app();
        let selected_id = self.selected_id.clone();
        let search_input = self.search_input.clone();
        let dialog = self.dialog.clone();
        let pending = self.pending.clone();
        let drop_error = self.drop_error.clone();
        let toast = self.toast.clone();

        div()
            .id("madsquash-root")
            .relative()
            .size_full()
            .bg(rgb(0x13161c))
            .text_color(rgb(0xe8edf4))
            .flex()
            .flex_row()
            .drag_over::<ExternalPaths>(|s, _, _, _| {
                s.border_2().border_dashed().border_color(rgb(0x5bbfe6)).bg(rgb(0x162533))
            })
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _window, cx| {
                this.handle_dropped_paths(paths, cx);
            }))
            // Left Sidebar Rail
            .child(render_rail())
            // Main Stage
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    // Top Bar (Search + Counter + Install)
                    .child(render_search_bar(&search_input, count_text, cx))
                    // Workspace (Cards Grid + Detail Pane)
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_row()
                            .overflow_hidden()
                            .child(render_list_pane(
                                visible,
                                selected_id.as_deref(),
                                query_is_empty,
                                cx,
                            ))
                            .child(render_detail_container(selected.clone(), cx)),
                    ),
            )
            // Modal Dialog (when open)
            .when_some(dialog, |this, dlg| match dlg {
                DialogKind::Install => {
                    this.child(render_install_dialog(pending, drop_error, cx))
                }
                DialogKind::Remove => {
                    let app_name = selected.as_ref().map(|a| a.name.clone()).unwrap_or_default();
                    this.child(render_remove_dialog(app_name, cx))
                }
            })
            // Toast Notification Banner (when active)
            .when_some(toast, |this, toast_msg| this.child(render_toast(toast_msg)))
    }
}

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            gpui_kit::init(cx);
            Theme::change(ThemeMode::Dark, None, cx);
            Theme::update(cx, |theme| {
                theme.background = rgb(0x12110f).into();
                theme.foreground = rgb(0xf4efe8).into();
                theme.muted = rgb(0x221f1b).into();
                theme.muted_foreground = rgb(0xa3988c).into();
                theme.border = rgb(0x3a342c).into();
                theme.input = rgb(0x3a342c).into();
                theme.primary = rgb(0x0fc5fc).into();
                theme.primary_foreground = rgb(0x07151a).into();
                theme.accent = rgb(0x0fc5fc).into();
                theme.accent_foreground = rgb(0x07151a).into();
                theme.ring = rgb(0x0fc5fc).into();
            });

            let window_bounds = WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(1060.0), px(700.0)),
                cx,
            ));

            let window_options = WindowOptions {
                window_bounds: Some(window_bounds),
                titlebar: Some(TitlebarOptions {
                    title: Some("Madsquash — AppImage".into()),
                    ..Default::default()
                }),
                ..Default::default()
            };

            gpui_kit::open_window(window_options, cx, |window, cx| {
                let search_input =
                    cx.new(|cx| InputState::new(window, cx).placeholder("Cari slot"));
                cx.new(|cx| MadsquashApp::new(search_input, cx))
            })
            .expect("failed to open window");
        });
}
