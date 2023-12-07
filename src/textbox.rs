use std::collections::HashMap;
use std::process::exit;

pub struct TextBox {
    pub input: String,
    pub current_with: usize,
    input_index: usize,
    input_width: HashMap<usize, u16>,
    lines: usize,
    cursor_x: u16,
    cursor_y: u16,
    input_counter: usize,
}

impl TextBox {
    pub fn input_width_reload(&mut self) {
        self.set_newline_input_width(
            self.cursor_y as usize,
            (self.input_counter).try_into().unwrap(),
        );
    }

    pub fn get_now_line_width(&mut self) -> u16 {
        *self.input_width.get(&(self.cursor_y as usize)).unwrap()
    }

    pub fn count_new_line(&self) -> u16 {
        self.cursor_y
    }

    pub fn input_width(&self) -> u16 {
        let index = self.cursor_x as u16;
        index.clone()
    }

    pub fn cursor_left(&mut self) {
        if self.input_index > 0 && self.cursor_x > 0 {
            self.cursor_x -= 1;
            self.input_index -= 1;
        } else {
        }
    }

    pub fn cursor_right(&mut self) {
        if self.cursor_x < self.get_now_line_width() {
            //println!(">{:?}, {:?}<", self.get_now_line_width(), self.cursor_x);
            self.cursor_x += 1;
            self.input_index += 1;
            //exit(0);
        } else {
            //println!(">{:?}, {:?}<", self.get_now_line_width(), self.cursor_y);
        }
    }

    pub fn cursor_up(&mut self) {
        if self.cursor_y < 1 {
            return;
        }

        self.cursor_y -= 1;
        let jamp_line_len = self.input_width.get(&(self.cursor_y as usize)).unwrap(); // ジャンプ先の行の文字数
        self.input_index -= self.cursor_x as usize + 1;
        self.cursor_x = *jamp_line_len;
        self.input_counter = *jamp_line_len as usize;
    }

    pub fn cursor_down(&mut self) {
        if usize::from(self.cursor_y + 1) >= self.input_width.len() {
            return;
        }
        self.cursor_y += 1;

        let jamp_line_len = self.input_width.get(&((self.cursor_y) as usize)).unwrap();

        let slice = &self.input[self.input_index..];

        for c in slice.chars() {
            if c == '\n' {
                break;
            }
            self.input_index += 1;
        }

        self.cursor_x = *jamp_line_len;
        self.input_index += *jamp_line_len as usize + 1;
        self.input_counter = *jamp_line_len as usize;
    }

    pub fn cursor_line_end(&mut self) {
        let index = self.input_width.get(&self.lines).unwrap();
        self.cursor_x = *index;
        self.input_index = *index as usize;
    }

    pub fn cursor_line_start(&mut self) {
        if self.cursor_x > 1 && self.input_index > 0 {
            self.cursor_x = 0;
            self.input_index = 0;
        } else {
            exit(0);
        }
    }

    pub fn add_newline(&mut self) {
        let slice = &self.input[self.input_index..];

        for c in slice.chars() {
            if c == '\n' {
                break;
            }
            self.input_counter += 1;
        }
        self.cursor_x = 0;
        self.input_counter -= 1;
        self.lines += 1;

        let slice = &self.input[self.input_index..];
        let mut enter_counter = 0;

        for c in slice.chars() {
            if c == '\n' {
                break;
            }
            enter_counter += 1;
        }

        let now_line_len = self.input_width.get(&((self.cursor_y) as usize)).unwrap();
        self.input_counter = *now_line_len as usize - enter_counter - 1;

        self.input_width_reload();
        self.input_counter = 0;
        self.cursor_y += 1;

        self.cursor_x = 0;
        self.input_counter = enter_counter;

        self.input_width_reload();
    }

    pub fn delete_input_data(&mut self) {
        if self.input_index <= 0 || self.cursor_x <= 0 {
            return;
        }
        self.input.remove(self.input_index - 1);
        self.input_counter -= 1;
        self.cursor_left();
    }

    pub fn add_input_data(&mut self, c: char) {
        self.input.insert(self.input_index, c);
        self.input_index += 1;
        self.cursor_x += 1;
        self.input_counter += 1;
        self.input_width_reload();
    }

    pub fn enter_command(&mut self) {}

    fn set_newline_input_width(&mut self, index: usize, width: u16) {
        self.input_width.insert(index, width);
    }

    fn set_current_input_width(&mut self) {
        self.cursor_x = 0;
        self.current_with = self.input.chars().count();
    }

    // カーソルを行頭に持っていく
    pub fn input_width_init(&mut self) {
        self.current_with = 0;
        self.lines = 0;
        self.input_index = 0;
        self.cursor_x = 0;
        self.cursor_y = 0;
    }
}

impl Default for TextBox {
    fn default() -> TextBox {
        TextBox {
            input: String::new(),
            input_index: 0,
            input_width: HashMap::from([(0, 0)]),
            current_with: 0,
            lines: 0,
            cursor_x: 0,
            cursor_y: 0,
            input_counter: 0,
        }
    }
}
