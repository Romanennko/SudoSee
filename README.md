# SudoSee

SudoSee is a fast, lightweight, and visually pleasing desktop application for tracking and organizing your media collection. Built entirely in Rust using [`eframe`](https://github.com/emilk/egui/tree/master/crates/eframe) and [`egui`](https://github.com/emilk/egui), it provides a fast, native GUI experience to manage movies, TV shows, board games, and more.

## Features

- **Media Tracking:** Keep track of movies, TV shows, board games, or any custom categories you create.
- **Detailed Metadata:** Store names, alternative names, categories, statuses (e.g., Planned, In Progress, Completed), ratings (0-10), priorities, and text notes.
- **Visuals & Attachments:** Add cover images via URLs and attach local files to your media items.
- **Local First & Privacy Focused:** All your data is saved locally in a `data.json` file. No accounts, no cloud sync—just your data on your machine.
- **Sleek Interface:** Modern, responsive dark-themed GUI powered by `egui`.
- **Fast & Efficient:** Built with Rust, meaning low resource consumption and instant startup times.

## Download & Run

The easiest way to use SudoSee is to download the pre-compiled executable:

1. Go to the [Releases](https://github.com/Romanennko/SudoSee/releases) page.
2. Download the latest `SudoSee-vX.X-Windows.zip`.
3. Extract the folder anywhere on your computer.
4. Double-click `SudoSee.exe` to run the application!

*Note: SudoSee will automatically create a `data.json` file in the same folder to store your media collection locally.*

---

## Development & Building from Source

To run and build SudoSee from source, you will need the Rust toolchain installed.

1. Install Rust via [rustup](https://rustup.rs/).

### Running the Application

Clone the repository and run the application using `cargo`:

```bash
git clone https://github.com/Romanennko/SudoSee.git
cd SudoSee
cargo run --release
```

*(Note: The first compilation might take a few minutes as dependencies are downloaded and built. Subsequent runs will be much faster.)*

## Building for Release

To build a standalone executable that you can run without `cargo`, use:

```bash
cargo build --release
```

The compiled binary will be located in `target/release/SudoSee.exe` (on Windows) or `target/release/SudoSee` (on Linux/macOS).

## Roadmap / Future Ideas
- Advanced sorting and filtering
- Statistics and visual data plots (powered by `egui_plot`)

## License

This project is open-source. Feel free to fork and modify!
