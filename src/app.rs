use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io;
use ratatui::{
    backend::Backend,
    Terminal,
};
use crate::textbox::TextBox; // appモジュールからApp構造体をインポート
use crate::inputmode::InputMode; // appモジュールからApp構造体をインポート
use crate::ui; // appモジュールからApp構造体をインポート

/// App holds the state of the application
pub struct App {
    pub input_mode: InputMode,
    pub text_box: TextBox,
    pub command_text_box: TextBox,
}
impl Default for App {
    fn default() -> App {
        App {
            input_mode: InputMode::Normal,
            text_box: TextBox::default(),
            command_text_box: TextBox::default(),
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        // 描画する内容は全てui関数にまかせてる
        terminal.draw(|f| ui::ui(f, &app))?;
        // 入力を受け取った処理
        if let Event::Key(KeyEvent {code, modifiers, ..}) = event::read()? {
            match app.input_mode {
                // ノーマルモードは入力を直接動作に割り当てる
                InputMode::Normal => match (code, modifiers) {
                    (KeyCode::Char('i'), KeyModifiers::NONE) => {
                        app.input_mode = InputMode::Editing;
                    }
                    (KeyCode::Char(':'), KeyModifiers::NONE) => {
                        app.input_mode = InputMode::Command;
                    }
                    (KeyCode::Esc, KeyModifiers::NONE) => {
                        return Ok(());
                    }
                    _ => {}
                },
                // 編集モードは入力をappに保存する
                InputMode::Editing => match (code, modifiers) {
                    (KeyCode::Enter, KeyModifiers::NONE) => {
                        app.text_box.input.push('\n');
                        app.text_box.add_newline();
                    }
                    (KeyCode::Char(c), _) => {
                        app.text_box.input.push(c);
                        app.text_box.set_input_width();
                    }
                    (KeyCode::Backspace, KeyModifiers::NONE) => {
                        if let Some(last)  = app.text_box.input.chars().last() {
                            if last != '\n' {
                                app.text_box.input.pop();
                                app.text_box.set_input_width();
                            }
                        }
                    }
                    (KeyCode::Esc, KeyModifiers::NONE) => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                // コマンドモードはcommand_inputに入力を保存する
                // Enterが押された時点でコマンドを実行
                InputMode::Command => match (code, modifiers) {
                    (KeyCode::Enter, KeyModifiers::NONE) => {
                        app.command_text_box.enter_command();
                        match app.command_text_box.input.as_str() {
                            "q" => {return Ok(())},
                            _ => {},
                        }
                    }
                    (KeyCode::Char(c), _) => {
                        app.command_text_box.input.push(c);
                        app.command_text_box.set_input_width();
                    }
                    (KeyCode::Backspace, KeyModifiers::NONE) => {
                        if let Some(last)  = app.command_text_box.input.chars().last() {
                            if last != '\n' {
                                app.command_text_box.input.pop();
                                app.command_text_box.set_input_width();
                            }
                        }
                    }
                    (KeyCode::Esc, KeyModifiers::NONE) => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}
