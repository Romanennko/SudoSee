use crate::app_data::MediaItem;
use eframe::egui;

pub fn show(
    ui: &mut egui::Ui,
    data_items: &mut Vec<MediaItem>,
    next_id: &mut usize,
    save_requested: &mut bool,
) {
    egui::MenuBar::new().ui(ui, |ui| {
        ui.label("SudoSee");
        if ui.button("💾 Save Data").clicked() {
            *save_requested = true;
        }

        ui.separator();

        if ui.button("📁 Export CSV").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("CSV", &["csv"])
                .set_file_name("sudosee_export.csv")
                .save_file()
            {
                if let Ok(mut wtr) = csv::Writer::from_path(path) {
                    for item in data_items.iter() {
                        let _ = wtr.serialize(item);
                    }
                }
            }
        }
        if ui.button("📁 Export JSON").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("JSON", &["json"])
                .set_file_name("sudosee_export.json")
                .save_file()
            {
                if let Ok(json) = serde_json::to_string_pretty(&*data_items) {
                    let _ = std::fs::write(path, json);
                }
            }
        }
        if ui.button("📂 Import").clicked() {
            if let Some(paths) = rfd::FileDialog::new()
                .add_filter("Data", &["json", "csv"])
                .pick_files()
            {
                for path in paths {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if ext == "json" {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                if let Ok(items) = serde_json::from_str::<Vec<MediaItem>>(&content)
                                {
                                    for mut item in items {
                                        item.id = *next_id;
                                        *next_id += 1;
                                        data_items.push(item);
                                    }
                                }
                            }
                        } else if ext == "csv" {
                            if let Ok(mut rdr) = csv::Reader::from_path(&path) {
                                for result in rdr.deserialize::<MediaItem>() {
                                    if let Ok(mut item) = result {
                                        item.id = *next_id;
                                        *next_id += 1;
                                        data_items.push(item);
                                    }
                                }
                            }
                        }
                    }
                }
                *save_requested = true;
            }
        }
    });
}
