use std::env;
use std::path::Path;

fn main() {
    // Only compile resources for Windows
    if let Ok(target_os) = env::var("CARGO_CFG_TARGET_OS") {
        if target_os == "windows" {
            let mut res = winres::WindowsResource::new();

            // Check if icon.ico exists in the root folder
            let icon_path = Path::new("icon.ico");
            if icon_path.exists() {
                res.set_icon("icon.ico");
            } else {
                println!(
                    "cargo:warning=No icon.ico found in the project root. Building the executable without a custom icon."
                );
            }

            // This compiles the icon into the final .exe
            if let Err(e) = res.compile() {
                println!("cargo:warning=Failed to compile windows resources: {}", e);
            }
        }
    }
}
