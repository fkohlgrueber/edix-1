/*

Cursor:
- y: usize (simple as that)
- x:
  - character-index: usize
  - pixel_pos: usize

Movement events:
- Left/Right: decrement/increment character-index and update pixel_pos
- Up/down: decrement/increment y and update character-index (using new line and pixel_pos)
- MouseClick: update y from mouse_y; set pixel_pos to mouse_x; update character-index from pixel_pos
*/

use crate::content::TextBackend;


pub struct Cursor {
    y: usize,
    x_idx: usize,  // counts in characters
    x_px: usize,   // counts in pixels
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            y: 0,
            x_idx: 0,
            x_px: 0,
        }
    }

    pub fn get_x<T: TextBackend>(&self, content: &T) -> usize {
        content.width_of_line(self.y, self.x_idx)
    }
    
    pub fn get_y<T: TextBackend>(&self, content: &T) -> usize {
        self.y * content.line_height()
    }

    pub fn get_idx<T: TextBackend>(&self, content: &T) -> usize {
        let mut sum = 0;
        for y in 0..self.y {
            sum += content.num_chars_of_line(y) + 1;
        }
        sum + self.x_idx
    }

    pub fn cursor_up<T: TextBackend>(&mut self, content: &T) {
        if self.y > 0 {
            self.y -= 1;
            self.update_x_idx(content);
        } else {
            self.x_idx = 0;
            self.x_px = 0;
        }
    }

    pub fn cursor_down<T: TextBackend>(&mut self, content: &T) {
        if self.y < content.num_lines() - 1 {
            self.y += 1;
            self.update_x_idx(content);
        } else {
            self.x_idx = content.num_chars_of_line(self.y);
            self.update_x_px(content);
        }
    }

    pub fn cursor_left<T: TextBackend>(&mut self, content: &T) {
        if self.x_idx > 0 {
            self.x_idx -= 1;
            self.update_x_px(content);
        } else if self.y > 0 {
            self.y -= 1;
            self.x_idx = content.num_chars_of_line(self.y);
            self.update_x_px(content);
        }
    }

    pub fn cursor_right<T: TextBackend>(&mut self, content: &T) {
        if self.x_idx < content.num_chars_of_line(self.y) {
            self.x_idx += 1;
            self.update_x_px(content);
        } else if self.y < content.num_lines() - 1 {
            self.y += 1;
            self.x_idx = 0;
            self.update_x_px(content);
        }
    }

    pub fn cursor_home<T: TextBackend>(&mut self, content: &T) {
        self.x_idx = 0;
        self.update_x_px(content);
    }

    pub fn cursor_end<T: TextBackend>(&mut self, content: &T) {
        self.x_idx = content.num_chars_of_line(self.y);
        self.update_x_px(content);
    }

    pub fn mouse_click<T: TextBackend>(&mut self, content: &T, x: usize, y: usize) {
        self.y = y / content.line_height();
        self.x_px = x;
        self.update_x_idx(content);
        self.update_x_px(content);
    }

    fn update_x_idx<T: TextBackend>(&mut self, content: &T) {
        let current_line_len = content.num_chars_of_line(self.y);
        
        let mut prev_width = 0;
        for i in 0..=current_line_len {
            let width = content.width_of_line(self.y, i);
            if width >= self.x_px {
                let prev_diff = self.x_px - prev_width;
                let curr_diff = width - self.x_px;
                self.x_idx = if curr_diff < prev_diff || i == 0 {
                    i
                } else {
                    i-1
                };
                return;
            }

            prev_width = width;
        }

        self.x_idx = current_line_len;
    }

    fn update_x_px<T: TextBackend>(&mut self, content: &T) {
        self.x_px = content.width_of_line(self.y, self.x_idx);
    }

}
