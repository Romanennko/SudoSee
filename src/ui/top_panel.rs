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

        if ui.button("📦 Backup (ZIP)").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("ZIP Backup", &["zip"])
                .set_file_name("sudosee_backup.zip")
                .save_file()
            {
                if let Ok(file) = std::fs::File::create(&path) {
                    let mut zip = zip::ZipWriter::new(file);
                    let options = zip::write::SimpleFileOptions::default()
                        .compression_method(zip::CompressionMethod::Stored);

                    // 1. Write data.json
                    if let Ok(json) = serde_json::to_string_pretty(&*data_items) {
                        let _ = zip.start_file("data.json", options);
                        use std::io::Write;
                        let _ = zip.write_all(json.as_bytes());
                    }

                    // Helper to add directory
                    let mut add_dir_to_zip = |dir_path: &str| {
                        let _ = zip.add_directory(dir_path, options);
                        if let Ok(entries) = std::fs::read_dir(dir_path) {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if path.is_file() {
                                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                        if let Ok(mut f) = std::fs::File::open(&path) {
                                            use std::io::Read;
                                            let mut buffer = Vec::new();
                                            if f.read_to_end(&mut buffer).is_ok() {
                                                let zip_path = format!("{}/{}", dir_path, name);
                                                let _ = zip.start_file(zip_path, options);
                                                use std::io::Write;
                                                let _ = zip.write_all(&buffer);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    };

                    add_dir_to_zip("covers");
                    add_dir_to_zip("attachments");

                    let _ = zip.finish();
                }
            }
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

        ui.separator();

        if ui.button("📥 Restore (ZIP)").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("ZIP Backup", &["zip"])
                .pick_file()
            {
                if let Ok(file) = std::fs::File::open(&path) {
                    if let Ok(mut archive) = zip::ZipArchive::new(file) {
                        for i in 0..archive.len() {
                            if let Ok(mut file) = archive.by_index(i) {
                                let outpath = match file.enclosed_name() {
                                    Some(path) => path.to_owned(),
                                    None => continue,
                                };

                                if file.is_dir() {
                                    let _ = std::fs::create_dir_all(&outpath);
                                } else {
                                    if let Some(p) = outpath.parent() {
                                        if !p.exists() {
                                            let _ = std::fs::create_dir_all(p);
                                        }
                                    }

                                    if outpath.to_string_lossy() == "data.json" {
                                        use std::io::Read;
                                        let mut content = String::new();
                                        if file.read_to_string(&mut content).is_ok() {
                                            if let Ok(items) =
                                                serde_json::from_str::<Vec<MediaItem>>(&content)
                                            {
                                                for mut item in items {
                                                    item.id = *next_id;
                                                    *next_id += 1;
                                                    data_items.push(item);
                                                }
                                                *save_requested = true;
                                            }
                                        }
                                    } else {
                                        if let Ok(mut outfile) = std::fs::File::create(&outpath) {
                                            let _ = std::io::copy(&mut file, &mut outfile);
                                        }
                                    }
                                }
                            }
                        }
                    }
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
