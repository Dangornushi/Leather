use crate::inputmode::InputMode; // appモジュールからApp構造体をインポート
use crate::textbox::TextBox; // appモジュールからApp構造体をインポート
use crate::ui; // appモジュールからApp構造体をインポート
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::Backend, backend::CrosstermBackend, Terminal};
use std::fs::File;
use std::io::Stdout;
use std::io::Write;
use std::process::exit;
use std::{self, error::Error, io};

pub enum Status {
    StatusExit,
    StatusPass,
}

/// App holds the state of the application
pub struct App {
    pub input_mode: InputMode,
    pub text_box: TextBox,
    pub command_text_box: TextBox,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}
impl App {
    pub fn default(terminal: Terminal<CrosstermBackend<Stdout>>) -> App {
        App {
            input_mode: InputMode::Normal,
            text_box: TextBox::default(),
            command_text_box: TextBox::default(),
            terminal: terminal,
        }
    }
    fn get_terminal(&mut self) -> Option<Terminal<CrosstermBackend<io::Stdout>>> {
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture).ok()?;
        return Some(Terminal::new(CrosstermBackend::new(io::stdout())).ok()?);
    }

    pub fn do_exit(&mut self) -> Result<(), Box<dyn Error>> {
        // restore terminal
        disable_raw_mode()?;
        let mut terminal = self.get_terminal().unwrap();
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        exit(0);
    }

    pub fn save(&mut self, text: String) -> Result<(), std::io::Error> {
        let mut file = File::create("hello.txt")?;
        file.write(text.as_bytes())?;

        return Ok(());
    }

    pub fn save_file_text_box_input(&mut self) -> Result<(), std::io::Error> {
        let input_text = self.text_box.input.clone();
        self.save(input_text)
    }

    pub fn exec_command(
        &mut self,
        code: KeyCode,
        modifiers: KeyModifiers,
    ) -> Result<Status, Box<dyn Error>> {
        match (code, modifiers) {
            (KeyCode::Enter, KeyModifiers::NONE) => {
                self.command_text_box.enter_command();
                match self.command_text_box.input.as_str() {
                    "q" => Ok(Status::StatusExit),
                    "w" => {
                        self.save_file_text_box_input();
                        Ok(Status::StatusPass)
                    }
                    _ => Ok(Status::StatusPass),
                }
            }
            (KeyCode::Char(c), _) => {
                self.command_text_box.input.push(c);
                self.command_text_box.set_input_width();
                Ok(Status::StatusPass)
            }
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                if let Some(last) = self.command_text_box.input.chars().last() {
                    if last != '\n' {
                        self.command_text_box.input.pop();
                        self.command_text_box.set_input_width();
                    }
                }
                Ok(Status::StatusPass)
            }
            (KeyCode::Esc, KeyModifiers::NONE) => {
                self.input_mode = InputMode::Normal;
                Ok(Status::StatusPass)
            }
            _ => Ok(Status::StatusPass),
        }
    }
}
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut system_status: Result<Status, Box<dyn Error>> = Ok(Status::StatusPass);
    // 描画する内容は全てui関数にまかせてる
    terminal.draw(|f| ui::ui(f, &app))?;
    // 入力を受け取った処理
    if let Event::Key(KeyEvent {
        code, modifiers, ..
    }) = event::read()?
    {
        match app.input_mode {
            // ノーマルモードは入力を直接動作に割り当てる
            InputMode::Normal => match (code, modifiers) {
                (KeyCode::Char('i'), KeyModifiers::NONE) => {
                    app.input_mode = InputMode::Editing;
                }
                (KeyCode::Char('h'), KeyModifiers::NONE) => {}
                (KeyCode::Char('g'), KeyModifiers::NONE) => {
                    app.text_box.input_width_init();
                }
                (KeyCode::Char(':'), KeyModifiers::NONE) => {
                    app.input_mode = InputMode::Command;
                }
                (KeyCode::Esc, KeyModifiers::NONE) => {}
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
                    if let Some(last) = app.text_box.input.chars().last() {
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
            InputMode::Command => system_status = app.exec_command(code, modifiers),
        }
    }
    match system_status {
        Ok(Status::StatusExit) => Ok(()),
        _ => run_app(terminal, app),
    }
}
