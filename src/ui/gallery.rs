use eframe::egui;

pub fn show(ctx: &egui::Context, gallery_state: &mut Option<(Vec<String>, usize)>) {
    if let Some((images, mut active_idx)) = gallery_state.clone() {
        let mut close_gallery = false;
        egui::Window::new("Image Gallery")
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .collapsible(false)
            .resizable(true)
            .show(ctx, |ui| {
                if images.is_empty() {
                    close_gallery = true;
                    return;
                }
                if active_idx >= images.len() {
                    active_idx = 0;
                }
                let current_image = &images[active_idx];

                // Draw Image
                ui.add(
                    egui::Image::new(current_image)
                        .fit_to_exact_size(egui::vec2(800.0, 600.0))
                        .maintain_aspect_ratio(true),
                );

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("◀ Previous").clicked() {
                        if active_idx == 0 {
                            active_idx = images.len() - 1;
                        } else {
                            active_idx -= 1;
                        }
                    }
                    ui.label(format!("{} / {}", active_idx + 1, images.len()));
                    if ui.button("Next ▶").clicked() {
                        if active_idx + 1 >= images.len() {
                            active_idx = 0;
                        } else {
                            active_idx += 1;
                        }
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("❌ Close").clicked() {
                            close_gallery = true;
                        }
                    });
                });
            });

        if close_gallery {
            *gallery_state = None;
        } else {
            *gallery_state = Some((images, active_idx));
        }
    }
}
