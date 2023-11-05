use std::collections::HashMap;

pub struct TextBox {
    pub input: String,
    input_width: HashMap<usize, u16>,
    current_with: usize,
    lines: usize,
}

impl TextBox {

    pub fn count_new_line(&self) -> u16 {
        self.input.chars().filter(|&x| x == '\n').count() as u16
    }

    pub fn set_input_width(&mut self) {
        let lines = self.input_width.get_mut(&self.lines).unwrap();
        *lines = (self.input.chars().count() - self.current_with) as u16;
    }


    pub fn input_width(&self) -> &u16 {
        let index = self.input_width.get(&self.lines).unwrap();
        index
    }

    pub fn add_newline(&mut self) {
        self.lines += 1;
        self.set_newline_input_width(self.lines.clone(), 0);
        self.set_current_input_width();
    }

    pub fn enter_command(&mut self) {
    }

    fn set_newline_input_width(&mut self, index: usize, width: u16) {
        self.input_width.insert(index, width);
    }

    fn set_current_input_width(&mut self) {
        self.current_with = self.input.chars().count();
    }

    /*
    // カーソルを行頭に持っていく
    fn input_width_init(&mut self) {
        self.input_width = HashMap::from([(0, 0)]);
        self.current_with = 0;
        self.lines = 0;
    }
    */
}

impl Default for TextBox {
    fn default() -> TextBox {
        TextBox {
            input: String::new(),
            input_width: HashMap::from([(0, 0)]),
            current_with: 0,
            lines: 0,
        }
    }
}
