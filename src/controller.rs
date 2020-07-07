
use crate::cursor::{
    Cursor,
};

use crate::app::FontSelection;
use crate::content::{
    Content,
    RichContentOptions
};

/*

Cursor
Content
  RawText
  Text

RawText -> Text (syntax highlighting)

Events:
Cursor movement
Copy/Paste/Cut
Char
Bs/Del
Enter

State Updates:
update_raw_text(t) (updates raw_text and text)


*/


pub struct Controller {
    cursor: Cursor,
    content: Content,
}

impl Controller {
    pub fn new(txt: &str, options: RichContentOptions) -> Self {
        Self {
            cursor: Cursor::new(),
            content: Content::from_str(txt, options),
        }
    }

    pub fn get_x(&self) -> usize {
        self.cursor.get_x(&self.content)
    }
    
    pub fn get_y(&self) -> usize {
        self.cursor.get_y(&self.content)
    }

    pub fn get_html(&self) -> yew::Html {
        self.content.to_html()
    }

    pub fn get_raw_text(&self) -> String {
        self.content.get_raw_text()
    }

    pub fn cursor_up(&mut self) {
        self.cursor.cursor_up(&self.content)
    }

    pub fn cursor_down(&mut self) {
        self.cursor.cursor_down(&self.content)
    }

    pub fn cursor_left(&mut self) {
        self.cursor.cursor_left(&self.content)
    }

    pub fn cursor_right(&mut self) {
        self.cursor.cursor_right(&self.content)
    }

    pub fn cursor_home(&mut self) {
        self.cursor.cursor_home(&self.content)
    }

    pub fn cursor_end(&mut self) {
        self.cursor.cursor_end(&self.content)
    }

    pub fn mouse_click(&mut self, x: usize, y: usize) {
        self.cursor.mouse_click(&self.content, x, y)
    }

    pub fn key_enter(&mut self) {
        self.key_char('\n')
    }

    pub fn key_backspace(&mut self) {
        let idx = self.cursor.get_idx(&self.content);
        if idx > 0 {
            self.cursor_left();
            self.content.update(idx-1, idx, "");
        }
    }

    pub fn key_delete(&mut self) {
        let idx = self.cursor.get_idx(&self.content);
        if idx < self.content.num_chars() {
            self.content.update(idx, idx+1, "");
        }
    }

    pub fn key_char(&mut self, c: char) {
        let idx = self.cursor.get_idx(&self.content);
        self.content.update(idx, idx, &c.to_string());
        self.cursor_right()
    }

    pub fn set_font(&mut self, font: FontSelection) {
        self.content.set_font(font)
    }

    pub fn set_syntax(&mut self, syntax: String) {
        self.content.set_syntax(syntax)
    }

    pub fn set_theme(&mut self, theme: String) {
        self.content.set_theme(theme)
    }

    pub fn get_state(&self) -> crate::app::State {
        self.content.get_state()
    }

    pub fn get_options(&self) -> &RichContentOptions {
        &self.content.options
    }
}