use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    None,
    Low,
    Medium,
    High,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::None
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::None => write!(f, "None"),
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SortCriterion {
    DateAdded,
    Name,
    Rating,
    Status,
    Priority,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SortDirection {
    Descending,
    Ascending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: usize,
    pub name: String,
    #[serde(default)]
    pub alt_names: Option<String>,
    pub category: String,
    pub status: String,
    pub rating: u8, // 0 to 10
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default = "default_date_added")]
    pub date_added: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub date_completed: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default)]
    pub cover_url: Option<String>,
    #[serde(default)]
    pub attachments: Vec<String>,
    #[serde(default)]
    pub priority: Priority,
}

fn default_date_added() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

impl MediaItem {
    pub fn new(id: usize, name: String, category: String, status: String) -> Self {
        Self {
            id,
            name,
            alt_names: None,
            category,
            status,
            rating: 0,
            notes: None,
            date_added: chrono::Utc::now(),
            date_completed: None,
            cover_url: None,
            attachments: Vec::new(),
            priority: Priority::None,
        }
    }
}

fn default_statuses() -> Vec<String> {
    vec![
        "Planned".to_string(),
        "In Progress".to_string(),
        "Completed".to_string(),
        "Dropped".to_string(),
        "On Hold".to_string(),
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub items: Vec<MediaItem>,
    pub categories: Vec<String>,
    #[serde(default = "default_statuses")]
    pub statuses: Vec<String>,
    pub next_id: usize,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            categories: vec![
                "Movies".to_string(),
                "TV Shows".to_string(),
                "Board Games".to_string(),
            ],
            statuses: default_statuses(),
            next_id: 1,
        }
    }
}

// Helper struct to parse old data formats gracefully
#[derive(Deserialize)]
struct OldAppData {
    items: Vec<serde_json::Value>,
    categories: Vec<String>,
    next_id: usize,
}

impl AppData {
    pub fn load() -> Self {
        if let Ok(data) = fs::read_to_string("data.json") {
            // Priority 1: Try exact modern parse.
            if let Ok(mut app_data) = serde_json::from_str::<AppData>(&data) {
                if app_data.statuses.is_empty() {
                    app_data.statuses = default_statuses();
                }
                if app_data.categories.is_empty() {
                    app_data.categories.push("General".to_string());
                }
                return app_data;
            }

            // Priority 2: Fallback parse. (if statuses array missing, or item status is enum instead of string)
            if let Ok(old_data) = serde_json::from_str::<OldAppData>(&data) {
                let mut migrated_items = Vec::new();

                for item_val in old_data.items {
                    let id = item_val.get("id").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                    let name = item_val
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown")
                        .to_string();
                    let category = item_val
                        .get("category")
                        .and_then(|v| v.as_str())
                        .unwrap_or("General")
                        .to_string();
                    let rating = item_val.get("rating").and_then(|v| v.as_u64()).unwrap_or(0) as u8;
                    let alt_names = item_val
                        .get("alt_names")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let notes = item_val
                        .get("notes")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let cover_url = item_val
                        .get("cover_url")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());

                    // Legacy status was an enum string like "Backlog" or {"Backlog": null} depending on serde configuration
                    let status = if let Some(s) = item_val.get("status") {
                        if let Some(str_val) = s.as_str() {
                            str_val.to_string()
                        } else if s.is_object() {
                            s.as_object()
                                .unwrap()
                                .keys()
                                .next()
                                .unwrap_or(&"Backlog".to_string())
                                .clone()
                        } else {
                            "Backlog".to_string()
                        }
                    } else {
                        "Backlog".to_string()
                    };

                    migrated_items.push(MediaItem {
                        id,
                        name,
                        alt_names,
                        category,
                        status,
                        rating,
                        notes,
                        date_added: chrono::Utc::now(),
                        date_completed: None,
                        cover_url,
                        attachments: Vec::new(),
                        priority: Priority::None,
                    });
                }

                return AppData {
                    items: migrated_items,
                    categories: old_data.categories,
                    statuses: default_statuses(),
                    next_id: old_data.next_id,
                };
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write("data.json", json);
        }
    }
}
