use crate::app_data::{AppData, MediaItem, Priority, SortCriterion, SortDirection};
use crate::ui;
use eframe::egui;

pub struct SudoSeeApp {
    pub data: AppData,
    pub new_item_name: String,
    pub new_item_cover_url: String,
    pub new_item_alt_names: String,
    pub new_item_category: String,
    pub new_item_status: String,
    pub new_item_priority: Priority,

    pub new_category_name: String,
    pub new_status_name: String,

    pub selected_status_filter: Option<String>,
    pub selected_category_filter: Option<String>,
    pub selected_priority_filter: Option<Priority>,

    // Dialog state
    pub editing_item: Option<MediaItem>,

    // Search and Sort
    pub search_query: String,
    pub sort_criterion: SortCriterion,
    pub sort_direction: SortDirection,

    // Gallery Overlay
    pub gallery_state: Option<(Vec<String>, usize)>,
}

impl Default for SudoSeeApp {
    fn default() -> Self {
        let data = AppData::load();
        let default_category = data.categories.first().cloned().unwrap_or_default();
        let default_status = data.statuses.first().cloned().unwrap_or_default();
        Self {
            data,
            new_item_name: String::new(),
            new_item_cover_url: String::new(),
            new_item_alt_names: String::new(),
            new_item_category: default_category,
            new_item_status: default_status,
            new_item_priority: Priority::None,

            new_category_name: String::new(),
            new_status_name: String::new(),

            selected_status_filter: None,
            selected_category_filter: None,
            selected_priority_filter: None,

            editing_item: None,

            search_query: String::new(),
            sort_criterion: SortCriterion::DateAdded,
            sort_direction: SortDirection::Descending,

            gallery_state: None,
        }
    }
}

impl eframe::App for SudoSeeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut save_requested = false;
        let mut set_gallery_state = None;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui::top_panel::show(
                ui,
                &mut self.data.items,
                &mut self.data.next_id,
                &mut save_requested,
            );
        });

        ui::edit_dialog::show(
            ctx,
            &mut self.editing_item,
            &self.data.categories,
            &self.data.statuses,
            &mut self.data.items,
            &mut save_requested,
            &mut set_gallery_state,
        );

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .min_width(220.0)
            .show(ctx, |ui| {
                ui::left_panel::show(
                    ui,
                    &mut self.search_query,
                    &mut self.sort_criterion,
                    &mut self.sort_direction,
                    &mut self.selected_category_filter,
                    &mut self.selected_status_filter,
                    &mut self.selected_priority_filter,
                    &mut self.data.categories,
                    &mut self.data.statuses,
                    &mut self.data.items,
                    &mut self.new_category_name,
                    &mut self.new_status_name,
                    &mut self.new_item_category,
                    &mut self.new_item_status,
                    &mut save_requested,
                );
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui::central_panel::show(
                ui,
                &mut self.data,
                &mut self.new_item_name,
                &mut self.new_item_alt_names,
                &mut self.new_item_category,
                &mut self.new_item_status,
                &mut self.new_item_priority,
                &mut self.new_item_cover_url,
                &self.search_query,
                self.sort_criterion,
                self.sort_direction,
                &self.selected_status_filter,
                &self.selected_category_filter,
                &self.selected_priority_filter,
                &mut self.editing_item,
                &mut set_gallery_state,
                &mut save_requested,
            );
        });

        if save_requested {
            self.data.save();
        }

        if set_gallery_state.is_some() {
            self.gallery_state = set_gallery_state;
        }

        ui::gallery::show(ctx, &mut self.gallery_state);
    }
}
