use crate::app_data::{MediaItem, Priority, SortCriterion, SortDirection};
use eframe::egui;

#[allow(clippy::too_many_arguments)]
pub fn show(
    ui: &mut egui::Ui,
    search_query: &mut String,
    sort_criterion: &mut SortCriterion,
    sort_direction: &mut SortDirection,
    selected_category_filter: &mut Option<String>,
    selected_status_filter: &mut Option<String>,
    selected_priority_filter: &mut Option<Priority>,
    data_categories: &mut Vec<String>,
    data_statuses: &mut Vec<String>,
    data_items: &mut Vec<MediaItem>,
    new_category_name: &mut String,
    new_status_name: &mut String,
    new_item_category: &mut String,
    new_item_status: &mut String,
    save_requested: &mut bool,
) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::CollapsingHeader::new("🔍 Search & Sort")
            .default_open(true)
            .show(ui, |ui| {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label("🔍");
                    ui.add(egui::TextEdit::singleline(search_query).hint_text("Search..."));
                });
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label("Sort by:");
                    egui::ComboBox::from_id_salt("sort_crit")
                        .selected_text(format!("{:?}", sort_criterion))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                sort_criterion,
                                SortCriterion::DateAdded,
                                "Date Added",
                            );
                            ui.selectable_value(sort_criterion, SortCriterion::Name, "Name");
                            ui.selectable_value(sort_criterion, SortCriterion::Rating, "Rating");
                            ui.selectable_value(sort_criterion, SortCriterion::Status, "Status");
                            ui.selectable_value(
                                sort_criterion,
                                SortCriterion::Priority,
                                "Priority",
                            );
                        });
                    if ui
                        .button(if *sort_direction == SortDirection::Ascending {
                            "⬆"
                        } else {
                            "⬇"
                        })
                        .clicked()
                    {
                        *sort_direction = if *sort_direction == SortDirection::Ascending {
                            SortDirection::Descending
                        } else {
                            SortDirection::Ascending
                        };
                    }
                });
            });
        ui.add_space(10.0);

        egui::CollapsingHeader::new("🏷 Filters")
            .default_open(true)
            .show(ui, |ui| {
                ui.add_space(5.0);

                if ui
                    .selectable_label(
                        selected_category_filter.is_none()
                            && selected_status_filter.is_none()
                            && selected_priority_filter.is_none(),
                        "🌟 All Content",
                    )
                    .clicked()
                {
                    *selected_category_filter = None;
                    *selected_status_filter = None;
                    *selected_priority_filter = None;
                }

                ui.separator();
                ui.label("📂 By Category:");
                for cat in data_categories.iter() {
                    let is_selected = selected_category_filter.as_deref() == Some(cat.as_str());
                    if ui.selectable_label(is_selected, cat).clicked() {
                        *selected_category_filter = Some(cat.clone());
                        *selected_status_filter = None;
                        *selected_priority_filter = None;
                    }
                }

                ui.add_space(10.0);
                ui.label("📌 By Status:");
                for status in data_statuses.iter() {
                    let is_selected = selected_status_filter.as_deref() == Some(status.as_str());
                    if ui.selectable_label(is_selected, status).clicked() {
                        *selected_status_filter = Some(status.clone());
                        *selected_category_filter = None;
                        *selected_priority_filter = None;
                    }
                }

                ui.add_space(10.0);
                ui.label("⭐ By Priority:");
                for priority in [
                    Priority::High,
                    Priority::Medium,
                    Priority::Low,
                    Priority::None,
                ] {
                    let is_selected = *selected_priority_filter == Some(priority);
                    if ui
                        .selectable_label(is_selected, priority.to_string())
                        .clicked()
                    {
                        *selected_priority_filter = Some(priority);
                        *selected_status_filter = None;
                        *selected_category_filter = None;
                    }
                }
            });

        ui.add_space(10.0);
        egui::CollapsingHeader::new("📈 Statistics")
            .default_open(false)
            .show(ui, |ui| {
                let total = data_items.len();
                let completed = data_items
                    .iter()
                    .filter(|i| i.date_completed.is_some())
                    .count();
                let rated_items: Vec<_> = data_items.iter().filter(|i| i.rating > 0).collect();
                let avg_rating = if rated_items.is_empty() {
                    0.0
                } else {
                    rated_items.iter().map(|i| i.rating as f32).sum::<f32>()
                        / rated_items.len() as f32
                };

                ui.label(format!("Total Items: {}", total));
                ui.label(format!(
                    "Completed: {} ({:.1}%)",
                    completed,
                    if total == 0 {
                        0.0
                    } else {
                        (completed as f32 / total as f32) * 100.0
                    }
                ));
                ui.label(format!("Avg Rating: {:.1} ★", avg_rating));

                ui.add_space(10.0);

                if total > 0 {
                    ui.collapsing("📊 View Charts", |ui| {
                        ui.label("Rating Distribution");
                        let mut rating_counts = [0; 11]; // 0 to 10
                        for item in data_items.iter() {
                            if item.rating <= 10 {
                                rating_counts[item.rating as usize] += 1;
                            }
                        }
                        use egui_plot::{Bar, BarChart, Plot};
                        let bars: Vec<Bar> = rating_counts
                            .iter()
                            .enumerate()
                            .map(|(i, &count)| Bar::new(i as f64, count as f64).width(0.8))
                            .collect();
                        let chart = BarChart::new("Ratings", bars).color(egui::Color32::GOLD);
                        Plot::new("rating_plot")
                            .height(120.0)
                            .allow_drag(false)
                            .allow_zoom(false)
                            .allow_scroll(false)
                            .show(ui, |plot_ui| plot_ui.bar_chart(chart));

                        ui.add_space(10.0);

                        // Category Pie Chart (Custom Draw)
                        ui.label("Categories");
                        let mut cat_counts = std::collections::HashMap::new();
                        for item in data_items.iter() {
                            *cat_counts.entry(item.category.clone()).or_insert(0) += 1;
                        }
                        let (rect, _response) =
                            ui.allocate_exact_size(egui::vec2(120.0, 120.0), egui::Sense::hover());
                        let center = rect.center();
                        let radius = 50.0;
                        let mut start_angle = 0.0_f32;
                        let colors = [
                            egui::Color32::from_rgb(200, 100, 100),
                            egui::Color32::from_rgb(100, 200, 100),
                            egui::Color32::from_rgb(100, 100, 200),
                            egui::Color32::from_rgb(200, 200, 100),
                            egui::Color32::from_rgb(200, 100, 200),
                            egui::Color32::from_rgb(100, 200, 200),
                        ];

                        if !cat_counts.is_empty() {
                            let mut sorted_cats: Vec<_> = cat_counts.into_iter().collect();
                            sorted_cats.sort_by(|a, b| b.1.cmp(&a.1));

                            ui.horizontal(|ui| {
                                for (i, (cat, count)) in sorted_cats.iter().enumerate() {
                                    let angle_extent =
                                        (*count as f32 / total as f32) * std::f32::consts::TAU;
                                    let color = colors[i % colors.len()];

                                    ui.painter().add(egui::Shape::Path(egui::epaint::PathShape {
                                        points: {
                                            let mut pts = vec![center];
                                            let steps = 16.max((angle_extent * 10.0) as usize);
                                            for step in 0..=steps {
                                                let a = start_angle
                                                    + angle_extent * (step as f32 / steps as f32);
                                                pts.push(
                                                    center
                                                        + egui::vec2(
                                                            a.cos() * radius,
                                                            a.sin() * radius,
                                                        ),
                                                );
                                            }
                                            pts
                                        },
                                        closed: true,
                                        fill: color,
                                        stroke: egui::epaint::PathStroke::new(
                                            0.0,
                                            egui::Color32::TRANSPARENT,
                                        ),
                                    }));

                                    start_angle += angle_extent;
                                    ui.label(
                                        egui::RichText::new(format!("{}: {}", cat, count))
                                            .color(color),
                                    );
                                }
                            });
                        }
                    });
                }
            });

        ui.add_space(10.0);

        egui::CollapsingHeader::new("⚙ Management")
            .default_open(false)
            .show(ui, |ui| {
                ui.add_space(10.0);

                // Category Management
                ui.group(|ui| {
                    ui.label("Categories");
                    ui.horizontal(|ui| {
                        let response = ui.add(
                            egui::TextEdit::singleline(new_category_name).hint_text("New Category"),
                        );
                        if ui.button("➕").clicked()
                            || (response.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                        {
                            let cat = new_category_name.trim();
                            if !cat.is_empty() && !data_categories.iter().any(|c| c == cat) {
                                data_categories.push(cat.to_string());
                                // User feature request: Immediately populate combobox selection
                                *new_item_category = cat.to_string();
                                new_category_name.clear();
                                *save_requested = true;
                            }
                        }
                    });

                    let mut cat_to_remove = None;
                    for (i, cat) in data_categories.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(cat);
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button("🗑").clicked() {
                                        cat_to_remove = Some(i);
                                    }
                                },
                            );
                        });
                    }
                    if let Some(idx) = cat_to_remove {
                        let removed = data_categories.remove(idx);
                        // Fallback category for items missing it
                        let fallback = data_categories
                            .first()
                            .cloned()
                            .unwrap_or_else(|| "General".to_string());
                        for item in data_items.iter_mut() {
                            if item.category == removed {
                                item.category = fallback.clone();
                            }
                        }
                        if *new_item_category == removed {
                            *new_item_category = fallback;
                        }
                        if selected_category_filter.as_deref() == Some(removed.as_str()) {
                            *selected_category_filter = None;
                        }
                        *save_requested = true;
                    }
                });

                ui.add_space(10.0);

                // Status Management
                ui.group(|ui| {
                    ui.label("Statuses");
                    ui.horizontal(|ui| {
                        let response = ui.add(
                            egui::TextEdit::singleline(new_status_name).hint_text("New Status"),
                        );
                        if ui.button("➕").clicked()
                            || (response.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                        {
                            let stat = new_status_name.trim();
                            if !stat.is_empty() && !data_statuses.iter().any(|s| s == stat) {
                                data_statuses.push(stat.to_string());
                                // User feature request: Immediately populate combobox selection
                                *new_item_status = stat.to_string();
                                new_status_name.clear();
                                *save_requested = true;
                            }
                        }
                    });

                    let mut status_to_remove = None;
                    for (i, stat) in data_statuses.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(stat);
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.button("🗑").clicked() {
                                        status_to_remove = Some(i);
                                    }
                                },
                            );
                        });
                    }
                    if let Some(idx) = status_to_remove {
                        let removed = data_statuses.remove(idx);
                        let fallback = data_statuses
                            .first()
                            .cloned()
                            .unwrap_or_else(|| "Backlog".to_string());
                        for item in data_items.iter_mut() {
                            if item.status == removed {
                                item.status = fallback.clone();
                            }
                        }
                        if *new_item_status == removed {
                            *new_item_status = fallback;
                        }
                        if selected_status_filter.as_deref() == Some(removed.as_str()) {
                            *selected_status_filter = None;
                        }
                        *save_requested = true;
                    }
                });
            });
    });
}
