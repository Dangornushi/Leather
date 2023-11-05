pub mod ui;
pub mod app;
pub mod textbox;
pub mod inputmode;

// ライブラリの公開インターフェースを定義
pub use ui::ui; 
pub use app::App;
pub use app::run_app;
pub use textbox::TextBox;
pub use inputmode::InputMode;

