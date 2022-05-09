#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;

use editor::Editor;
pub use terminal::Terminal;
pub use editor::Position;
fn main() {
    Editor::default().run();
}