use crate::app_data::{AppData, MediaItem, Priority, SortCriterion, SortDirection};
use eframe::egui;

#[allow(clippy::too_many_arguments)]
pub fn show(
    ui: &mut egui::Ui,
    data: &mut AppData,
    new_item_name: &mut String,
    new_item_alt_names: &mut String,
    new_item_category: &mut String,
    new_item_status: &mut String,
    new_item_priority: &mut Priority,
    new_item_cover_url: &mut String,
    search_query: &str,
    sort_criterion: SortCriterion,
    sort_direction: SortDirection,
    selected_status_filter: &Option<String>,
    selected_category_filter: &Option<String>,
    selected_priority_filter: &Option<Priority>,
    editing_item: &mut Option<MediaItem>,
    set_gallery_state: &mut Option<(Vec<String>, usize)>,
    save_requested: &mut bool,
) {
    ui.heading("🎬 Media Content");
    ui.add_space(10.0);

    // Add new item panel
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label("Title:");
            ui.add(egui::TextEdit::singleline(new_item_name).hint_text("Title"));
            ui.label("Alt:");
            ui.add(egui::TextEdit::singleline(new_item_alt_names).hint_text("Optional..."));

            ui.label("Category:");
            let mut current_cat = new_item_category.clone();
            egui::ComboBox::from_id_salt("new_item_cat")
                .width(150.0)
                .selected_text(&current_cat)
                .show_ui(ui, |ui| {
                    for cat in &data.categories {
                        ui.selectable_value(&mut current_cat, cat.clone(), cat);
                    }
                });
            *new_item_category = current_cat;

            ui.label("Status:");
            let mut current_status = new_item_status.clone();
            egui::ComboBox::from_id_salt("new_item_status")
                .width(120.0)
                .selected_text(&current_status)
                .show_ui(ui, |ui| {
                    for stat in &data.statuses {
                        ui.selectable_value(&mut current_status, stat.clone(), stat);
                    }
                });
            *new_item_status = current_status;
        });
        ui.horizontal(|ui| {
            ui.label("Priority:");
            let mut current_priority = *new_item_priority;
            egui::ComboBox::from_id_salt("new_item_priority")
                .width(80.0)
                .selected_text(current_priority.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut current_priority, Priority::None, "None");
                    ui.selectable_value(&mut current_priority, Priority::Low, "Low");
                    ui.selectable_value(&mut current_priority, Priority::Medium, "Medium");
                    ui.selectable_value(&mut current_priority, Priority::High, "High");
                });
            *new_item_priority = current_priority;

            ui.label("Cover:");
            ui.add(egui::TextEdit::singleline(new_item_cover_url).hint_text("URL or Path"));
            if ui.button("📁 Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Image", &["png", "jpg", "jpeg", "webp", "gif"])
                    .pick_file()
                {
                    if let Some(file_name) = path.file_name() {
                        std::fs::create_dir_all("covers").ok();
                        let target = std::path::Path::new("covers").join(file_name);
                        if std::fs::copy(&path, &target).is_ok() {
                            if let Ok(abs_target) = std::fs::canonicalize(&target) {
                                let mut path_str = abs_target.to_string_lossy().replace('\\', "/");
                                if let Some(stripped) = path_str.strip_prefix("//?/") {
                                    path_str = stripped.to_string();
                                }
                                let url = format!("file:///{}", path_str.trim_start_matches('/'));
                                *new_item_cover_url = url;
                            } else {
                                let url = format!(
                                    "file://covers/{}",
                                    target.file_name().unwrap_or_default().to_string_lossy()
                                );
                                *new_item_cover_url = url;
                            }
                        }
                    }
                }
            }

            ui.add_space(20.0);

            if ui.button("➕ Add Media").clicked() {
                let name = new_item_name.trim();
                if !name.is_empty() && !new_item_category.is_empty() && !new_item_status.is_empty()
                {
                    let mut new_item = MediaItem::new(
                        data.next_id,
                        name.to_string(),
                        new_item_category.clone(),
                        new_item_status.clone(),
                    );
                    new_item.priority = *new_item_priority;
                    let alt_trim = new_item_alt_names.trim();
                    if !alt_trim.is_empty() {
                        new_item.alt_names = Some(alt_trim.to_string());
                    }
                    if !new_item_cover_url.trim().is_empty() {
                        new_item.cover_url = Some(new_item_cover_url.trim().to_string());
                    }
                    data.items.push(new_item);
                    data.next_id += 1;
                    new_item_name.clear();
                    new_item_alt_names.clear();
                    new_item_cover_url.clear();
                    *save_requested = true;
                }
            }
        });
    });

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(10.0);

    // List of items
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let mut items_to_remove = Vec::new();
            let mut items_changed = false;

            let mut view_indices: Vec<usize> = (0..data.items.len()).collect();

            // Filter
            view_indices.retain(|&idx| {
                let item = &data.items[idx];
                if let Some(status_filter) = selected_status_filter {
                    if &item.status != status_filter {
                        return false;
                    }
                }
                if let Some(cat_filter) = selected_category_filter {
                    if &item.category != cat_filter {
                        return false;
                    }
                }
                if let Some(priority_filter) = selected_priority_filter {
                    if item.priority != *priority_filter {
                        return false;
                    }
                }
                if !search_query.is_empty() {
                    let q = search_query.to_lowercase();
                    let matches_name = item.name.to_lowercase().contains(&q);
                    let matches_alt = item
                        .alt_names
                        .as_ref()
                        .map(|a| a.to_lowercase().contains(&q))
                        .unwrap_or(false);
                    if !matches_name && !matches_alt {
                        return false;
                    }
                }
                true
            });

            // Sort
            view_indices.sort_by(|&a_idx, &b_idx| {
                let a = &data.items[a_idx];
                let b = &data.items[b_idx];
                let cmp = match sort_criterion {
                    SortCriterion::DateAdded => a.date_added.cmp(&b.date_added),
                    SortCriterion::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                    SortCriterion::Rating => a.rating.cmp(&b.rating),
                    SortCriterion::Status => a.status.cmp(&b.status),
                    SortCriterion::Priority => a.priority.cmp(&b.priority),
                };
                if sort_direction == SortDirection::Ascending {
                    cmp
                } else {
                    cmp.reverse()
                }
            });

            for idx in view_indices {
                let item = &data.items[idx];

                // Read-Only Item View
                egui::Frame::window(ui.style()).show(ui, |ui| {
                    ui.set_min_width(ui.available_width()); // Fill width
                    ui.horizontal(|ui| {
                        // Cover Image Thumbnail
                        if let Some(url) = &item.cover_url {
                            if !url.trim().is_empty() {
                                ui.add(
                                    egui::Image::new(url)
                                        .fit_to_exact_size(egui::vec2(120.0, 180.0))
                                        .maintain_aspect_ratio(true)
                                        .corner_radius(8.0),
                                );
                                ui.add_space(10.0);
                            }
                        }

                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                // Title & Category
                                ui.heading(&item.name);
                                ui.label(
                                    egui::RichText::new(format!("({})", item.category))
                                        .color(egui::Color32::GRAY),
                                );

                                // Push Delete / Edit buttons to right
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui.button("🗑 Delete").clicked() {
                                            items_to_remove.push(idx);
                                        }
                                        if ui.button("✏ Edit").clicked() {
                                            *editing_item = Some(item.clone());
                                        }
                                    },
                                );
                            });

                            if let Some(alt) = &item.alt_names {
                                ui.label(
                                    egui::RichText::new(alt)
                                        .color(egui::Color32::DARK_GRAY)
                                        .italics(),
                                );
                            }

                            if let Some(notes) = &item.notes {
                                if !notes.trim().is_empty() {
                                    ui.add_space(4.0);
                                    ui.label(
                                        egui::RichText::new(notes).color(egui::Color32::LIGHT_GRAY),
                                    );
                                }
                            }

                            if !item.attachments.is_empty() {
                                ui.add_space(5.0);
                                ui.horizontal_wrapped(|ui| {
                                    for (i, attachment) in item.attachments.iter().enumerate() {
                                        if !attachment.trim().is_empty() {
                                            let img_response = ui.add(
                                                egui::Image::new(attachment)
                                                    .fit_to_exact_size(egui::vec2(80.0, 80.0))
                                                    .maintain_aspect_ratio(true)
                                                    .corner_radius(4.0)
                                                    .sense(egui::Sense::click()),
                                            );
                                            if img_response.clicked() {
                                                *set_gallery_state =
                                                    Some((item.attachments.clone(), i));
                                            }
                                        }
                                    }
                                });
                            }

                            ui.add_space(6.0);

                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("Status: {}", item.status))
                                        .color(egui::Color32::KHAKI),
                                );

                                if item.priority != Priority::None {
                                    ui.add_space(15.0);
                                    let color = match item.priority {
                                        Priority::High => egui::Color32::LIGHT_RED,
                                        Priority::Medium => egui::Color32::LIGHT_YELLOW,
                                        Priority::Low => egui::Color32::LIGHT_BLUE,
                                        Priority::None => egui::Color32::GRAY,
                                    };
                                    ui.label(
                                        egui::RichText::new(format!("Pri: {}", item.priority))
                                            .color(color),
                                    );
                                }

                                ui.add_space(15.0);

                                ui.label(
                                    egui::RichText::new(format!("Rating: {} ★", item.rating))
                                        .color(egui::Color32::GOLD),
                                );

                                ui.add_space(15.0);

                                ui.label(
                                    egui::RichText::new(format!(
                                        "Added: {}",
                                        item.date_added.format("%Y-%m-%d")
                                    ))
                                    .color(egui::Color32::GRAY),
                                );

                                if let Some(comp) = item.date_completed {
                                    ui.add_space(15.0);
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "Completed: {}",
                                            comp.format("%Y-%m-%d")
                                        ))
                                        .color(egui::Color32::LIGHT_GREEN),
                                    );
                                }
                            });
                        });
                    });
                });

                ui.add_space(8.0);
            }

            if !items_to_remove.is_empty() {
                items_to_remove.sort_unstable_by(|a, b| b.cmp(a));
                for idx in items_to_remove {
                    data.items.remove(idx);
                }
                items_changed = true;
            }

            if items_changed {
                *save_requested = true;
            }
        });
}
