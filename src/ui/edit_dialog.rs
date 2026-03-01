use crate::app_data::MediaItem;
use crate::app_data::Priority;
use eframe::egui;

pub fn show(
    ctx: &egui::Context,
    editing_item: &mut Option<MediaItem>,
    data_categories: &[String],
    data_statuses: &[String],
    data_items: &mut [MediaItem],
    save_requested: &mut bool,
    set_gallery_state: &mut Option<(Vec<String>, usize)>,
) {
    let mut popup_open = editing_item.is_some();
    let mut close_dialog = false;

    if popup_open {
        let window_title = editing_item
            .as_ref()
            .map(|item| {
                if item.name.trim().is_empty() {
                    "Edit Media Content".to_string()
                } else {
                    item.name.clone()
                }
            })
            .unwrap_or_else(|| "Edit Media Content".to_string());

        egui::Window::new(window_title)
            .id(egui::Id::new("edit_dialog"))
            .collapsible(false)
            .resizable(false)
            .open(&mut popup_open)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                if let Some(edit_item) = editing_item {
                    ui.horizontal(|ui| {
                        ui.label("Title:");
                        ui.text_edit_singleline(&mut edit_item.name);
                    });

                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label("Alt/Original Title:");
                        let mut alt_text = edit_item.alt_names.clone().unwrap_or_default();
                        if ui.text_edit_singleline(&mut alt_text).changed() {
                            edit_item.alt_names = if alt_text.trim().is_empty() {
                                None
                            } else {
                                Some(alt_text)
                            };
                        }
                    });

                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label("Category:");
                        let mut current_cat = edit_item.category.clone();
                        egui::ComboBox::from_id_salt("edit_cat")
                            .selected_text(&current_cat)
                            .show_ui(ui, |ui| {
                                for cat in data_categories {
                                    ui.selectable_value(&mut current_cat, cat.clone(), cat);
                                }
                            });
                        edit_item.category = current_cat;
                    });

                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label("Status:");
                        let mut current_status = edit_item.status.clone();
                        egui::ComboBox::from_id_salt("edit_status")
                            .selected_text(&current_status)
                            .show_ui(ui, |ui| {
                                for stat in data_statuses {
                                    ui.selectable_value(&mut current_status, stat.clone(), stat);
                                }
                            });
                        edit_item.status = current_status;

                        ui.add_space(20.0);

                        ui.label("Rating:");
                        ui.add(egui::Slider::new(&mut edit_item.rating, 0..=10).text("★"));

                        ui.add_space(20.0);

                        ui.label("Priority:");
                        let mut current_priority = edit_item.priority;
                        egui::ComboBox::from_id_salt("edit_priority")
                            .selected_text(current_priority.to_string())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut current_priority, Priority::None, "None");
                                ui.selectable_value(&mut current_priority, Priority::Low, "Low");
                                ui.selectable_value(
                                    &mut current_priority,
                                    Priority::Medium,
                                    "Medium",
                                );
                                ui.selectable_value(&mut current_priority, Priority::High, "High");
                            });
                        edit_item.priority = current_priority;
                    });

                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label("Cover URL:");
                        let mut cover_text = edit_item.cover_url.clone().unwrap_or_default();
                        if ui.text_edit_singleline(&mut cover_text).changed() {
                            edit_item.cover_url = if cover_text.trim().is_empty() {
                                None
                            } else {
                                Some(cover_text)
                            };
                        }
                        if ui.button("📁 Browse...").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("Image", &["png", "jpg", "jpeg", "webp", "gif"])
                                .pick_file()
                            {
                                if let Some(file_name) = path.file_name() {
                                    std::fs::create_dir_all("covers").ok();
                                    let target = std::path::Path::new("covers").join(file_name);
                                    if std::fs::copy(&path, &target).is_ok() {
                                        // Make absolute file url to ensure it loads
                                        if let Ok(abs_target) = std::fs::canonicalize(&target) {
                                            let mut path_str =
                                                abs_target.to_string_lossy().replace('\\', "/");
                                            if let Some(stripped) = path_str.strip_prefix("//?/") {
                                                path_str = stripped.to_string();
                                            }
                                            let url = format!(
                                                "file:///{}",
                                                path_str.trim_start_matches('/')
                                            );
                                            edit_item.cover_url = Some(url);
                                        } else {
                                            let url = format!(
                                                "file://covers/{}",
                                                target
                                                    .file_name()
                                                    .unwrap_or_default()
                                                    .to_string_lossy()
                                            );
                                            edit_item.cover_url = Some(url);
                                        }
                                    }
                                }
                            }
                        }
                    });

                    ui.add_space(5.0);

                    ui.collapsing("🖼 Attachments", |ui| {
                        if !edit_item.attachments.is_empty() {
                            let mut to_remove_attachment = None;
                            ui.horizontal_wrapped(|ui| {
                                for (i, att) in edit_item.attachments.iter().enumerate() {
                                    if !att.trim().is_empty() {
                                        ui.vertical(|ui| {
                                            let img_response = ui.add(
                                                egui::Image::new(att)
                                                    .fit_to_exact_size(egui::vec2(100.0, 100.0))
                                                    .maintain_aspect_ratio(true)
                                                    .corner_radius(6.0)
                                                    .sense(egui::Sense::click()),
                                            );
                                            if img_response.clicked() {
                                                *set_gallery_state =
                                                    Some((edit_item.attachments.clone(), i));
                                            }
                                            if ui.button("❌ Remove").clicked() {
                                                to_remove_attachment = Some(i);
                                            }
                                        });
                                    }
                                }
                            });
                            if let Some(idx) = to_remove_attachment {
                                edit_item.attachments.remove(idx);
                            }
                            ui.add_space(5.0);
                        }

                        if ui.button("➕ Add Media Attachment").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("Image", &["png", "jpg", "jpeg", "webp", "gif"])
                                .pick_file()
                            {
                                if let Some(file_name) = path.file_name() {
                                    std::fs::create_dir_all("attachments").ok();
                                    let target =
                                        std::path::Path::new("attachments").join(file_name);
                                    if std::fs::copy(&path, &target).is_ok() {
                                        if let Ok(abs_target) = std::fs::canonicalize(&target) {
                                            let mut path_str =
                                                abs_target.to_string_lossy().replace('\\', "/");
                                            if let Some(stripped) = path_str.strip_prefix("//?/") {
                                                path_str = stripped.to_string();
                                            }
                                            let url = format!(
                                                "file:///{}",
                                                path_str.trim_start_matches('/')
                                            );
                                            edit_item.attachments.push(url);
                                        } else {
                                            let url = format!(
                                                "file://attachments/{}",
                                                target
                                                    .file_name()
                                                    .unwrap_or_default()
                                                    .to_string_lossy()
                                            );
                                            edit_item.attachments.push(url);
                                        }
                                    }
                                }
                            }
                        }
                    });

                    ui.add_space(5.0);

                    ui.label("Notes:");
                    let mut notes_text = edit_item.notes.clone().unwrap_or_default();
                    if ui
                        .add(egui::TextEdit::multiline(&mut notes_text).desired_rows(3))
                        .changed()
                    {
                        edit_item.notes = if notes_text.trim().is_empty() {
                            None
                        } else {
                            Some(notes_text)
                        };
                    }

                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "Added: {}",
                            edit_item.date_added.format("%Y-%m-%d")
                        ));
                        ui.add_space(20.0);

                        if let Some(comp) = edit_item.date_completed {
                            ui.label(format!("Completed: {}", comp.format("%Y-%m-%d")));
                            if ui.button("❌").clicked() {
                                edit_item.date_completed = None;
                            }
                        } else {
                            if ui.button("✔ Mark Completed").clicked() {
                                edit_item.date_completed = Some(chrono::Utc::now());
                            }
                        }
                    });

                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        if ui.button("💾 Save").clicked() {
                            if let Some(idx) = data_items.iter().position(|i| i.id == edit_item.id)
                            {
                                data_items[idx] = edit_item.clone();
                                *save_requested = true;
                            }
                            close_dialog = true;
                        }
                        if ui.button("Cancel").clicked() {
                            close_dialog = true;
                        }
                    });
                }
            });

        if close_dialog || !popup_open {
            // Closed by button OR the "X" on the window header
            *editing_item = None;
        }
    }
}
