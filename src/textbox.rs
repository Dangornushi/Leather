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
}

impl TextBox {
    pub fn count_new_line(&self) -> u16 {
        //self.input.chars().filter(|&x| x == '\n').count() as u16
        self.cursor_y
    }

    pub fn set_input_width(&mut self) {
        println!("{:?}", self.input_width.get(&self.lines).unwrap());

        let lines = self.input_width.get_mut(&self.lines).unwrap();
        *lines = (self.input.chars().count() - self.current_with) as u16;
    }

    pub fn input_width(&self) -> u16 {
        //let index = self.input_width.get(&self.lines).unwrap();
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
        if self.input_index < self.input.len() {
            self.cursor_x += 1;
            self.input_index += 1;
        } else {
        }
    }

    pub fn cursor_up(&mut self) {
        self.cursor_y -= 1;
        self.lines -= 1;
        self.cursor_line_end();
    }

    pub fn cursor_down(&mut self) {
        self.cursor_y += 1;
        self.lines += 1;

        let slice = &self.input[self.input_index - 1..];

        for c in slice.chars() {
            if c == '\n' {
                break;
            }
            self.input_index += 1;
        }

        let index = self.input_width.get(&self.lines).unwrap();

        if *index > self.cursor_x {
            self.input_index += self.cursor_x as usize;
        } else {
            self.input_index += *index as usize;
            self.cursor_x = *index;
        }
    }

    pub fn cursor_line_end(&mut self) {
        let index = self.input_width.get(&self.lines).unwrap();
        self.cursor_x = *index;
        self.input_index = *index as usize;
    }

    pub fn cursor_line_start(&mut self) {
        if self.cursor_x > 1 && self.input_index > 0 {
            /*
            println!(
                "{:?} - {:?} : {:?} - {:?}",
                index, self.input_index, self.cursor_x, self.input_index
            );*/
            self.cursor_x = 0;
            self.input_index = 0;
        } else {
            exit(0);
        }
    }

    pub fn add_newline(&mut self) {
        self.lines += 1;
        self.cursor_y += 1;
        self.cursor_x = 0;
        self.set_newline_input_width(self.lines.clone(), 0);
        self.set_current_input_width();
    }

    pub fn delete_input_data(&mut self) {
        if self.input_index > 0 {
            self.input.remove(self.input_index - 1);
            self.cursor_left();
        } else {
        }
    }

    pub fn add_input_data(&mut self, c: char) {
        self.input.insert(self.input_index, c);
        self.cursor_right();
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
        //self.input_width = HashMap::from([(0, 0)]);
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
        }
    }
}
