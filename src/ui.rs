use ratatui::{
    backend::Backend,
    layout::{ Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Text, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crate::app::App; // appモジュールからApp構造体をインポート
use crate::inputmode::InputMode; // appモジュールからInputMode enumをインポート

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                // 文字を入力する部分とコマンドを書きこむ部分の二つに分かれている
                Constraint::Percentage(90),
                Constraint::Percentage(4),
                Constraint::Length(5),
            ]
            .as_ref(),
        )
        .split(f.size());


    let (msg, style) = match app.input_mode {
        // uiでモードをわけているけど、関数を分けて書いた方が良い
        InputMode::Normal => (
            vec![
                Span::styled("Normal mode", Style::default().add_modifier(Modifier::BOLD)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::styled("Editing mode", Style::default().add_modifier(Modifier::BOLD)),
            ],
            Style::default(),
        ),
        InputMode::Command => (
            vec![
                Span::styled("Command mode", Style::default().add_modifier(Modifier::BOLD)),
            ],
            Style::default(),
        ),
    };

    // 編集ウィンドウ -----
    let windows = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[0]);

    let input = Paragraph::new(app.text_box.input.as_ref())
        .style(match app.input_mode {
            InputMode::Editing => Style::default().fg(Color::Blue),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Input")).wrap(Wrap {trim : false});
    f.render_widget(input, windows[0]);
    // --------------------

    // ステータスメッセージ
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    // --------------------

    // コマンド入力ウィンドウ
    let command_box = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[2]);

    let command_input = Paragraph::new(app.command_text_box.input.as_ref())
        .style(match app.input_mode {
            InputMode::Command => Style::default().fg(Color::Green),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Command")).wrap(Wrap {trim : false});
    f.render_widget(command_input, command_box[0]);
    // --------------------

    match app.input_mode {
        InputMode::Normal => {}

        InputMode::Command => {
            f.set_cursor(
                command_box[0].x + app.command_text_box.input_width() + 1,
                command_box[0].y+1,
            );
        }

        InputMode::Editing => {
            f.set_cursor(
                windows[0].x + app.text_box.input_width() + 1,
                windows[0].y + app.text_box.count_new_line() + 1,
            )
        }
    }

}
