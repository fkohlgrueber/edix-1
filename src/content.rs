use crate::highlight::highlight;
use crate::app::FontSelection;
use crate::app::State;

pub trait TextBackend {
    fn num_lines(&self) -> usize;

    fn num_chars_of_line(&self, y: usize) -> usize;

    // width of chars `0..x_max`
    // x_max is in range 0..=line_len
    fn width_of_line(&self, y: usize, x_max: usize) -> usize;

    // height of each line in px
    fn line_height(&self) -> usize;
}

pub struct Content {
    raw: String,
    rich: Vec<Vec<Region>>,
    pub options: RichContentOptions,
}

impl Content {
    pub fn from_str(s: &str, options: RichContentOptions) -> Self {
        let mut c = Self {
            raw: s.to_string(),
            rich: vec!(), 
            options,
        };
        c.update_rich();
        c
    }

    pub fn update(&mut self, from: usize, to: usize, s: &str) {
        let from_idx = self.raw.char_indices().nth(from).map(|x| x.0).unwrap_or(self.raw.len());
        let to_idx = self.raw.char_indices().nth(to).map(|x| x.0).unwrap_or(self.raw.len());
        self.raw.replace_range(from_idx..to_idx, s);
        self.update_rich();
    }

    pub fn get_raw_text(&self) -> String {
        self.raw.clone()
    }

    pub fn num_chars(&self) -> usize {
        self.raw.chars().count()
    }

    pub fn to_html(&self) -> yew::Html {
        use yew::html;
        html!(
            {for self.rich.iter().map(|elmts| {
                html!(
                    <div class="ed-line">
                        {for elmts.iter().map(
                            |r| html!(<span title=r.title style={&r.sty}>{&r.s.replace(' ', "\u{00a0}")}</span>)
                        )}
                </div>
            )
        })})
    }


    fn update_rich(&mut self) {
        /*
        if self.raw.is_empty() {
            self.rich = vec!(vec!());
            return;
        }
        let mut regions = vec!();
        for line in self.raw.lines() {
            let mut line_regions = vec!();
            let mut monospace = false;
            for word in line.split(' ') {
                let sty = format!("font-family: {};", if monospace { "mono" } else { "prop" });
                line_regions.push(Region::new(word.to_string(), sty.to_string()));
                line_regions.push(Region::new(" ".to_string(), sty.to_string()));
                monospace = !monospace;
            }
            line_regions.pop(); // pop last space
            regions.push(line_regions);
        }
        self.rich = regions;
        */
        //yew::services::ConsoleService::log(&format!("{:?}", self.raw));
        self.rich = highlight(&self.raw, &self.options);

        let exp_lines = self.raw.chars().filter(|x| x==&'\n').count() + 1;
        
        while self.rich.len() < exp_lines {
            self.rich.push(vec!());
        }
    }

    pub fn set_font(&mut self, font: FontSelection) {
        self.options.font_selection = font;
        self.update_rich()
    }

    pub fn set_syntax(&mut self, syntax: String) {
        self.options.selected_syntax = syntax;
        self.update_rich()
    }

    pub fn set_theme(&mut self, theme: String) {
        self.options.selected_theme = theme;
        self.update_rich()
    }

    pub fn get_state(&self) -> State {
        State {
            content: self.raw.clone(),
            font_selection: self.options.font_selection.clone(),
            selected_theme: self.options.selected_theme.clone(),
            selected_syntax: self.options.selected_syntax.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RichContentOptions {
    pub font_selection: FontSelection,
    pub selected_syntax: String,
    pub selected_theme: String,
    pub line_height: usize,
}

impl TextBackend for Content {

    fn num_lines(&self) -> usize {
        self.rich.len()
    }

    fn num_chars_of_line(&self, y: usize) -> usize {
        self.rich[y].iter().map(|r| r.s.chars().count()).sum()
    }

    fn width_of_line(&self, y: usize, x_max: usize) -> usize {
        let document = web_sys::window().unwrap().document().unwrap();

        let create_span = |s: &str, sty: &str| -> web_sys::Element {
            let s = s.replace(' ', "\u{00a0}");
            let elmt = document.create_element("span").unwrap();
            let elmt_text = document.create_text_node(&s);
            elmt.append_child(&elmt_text).unwrap();
            elmt.set_attribute("style", &sty).unwrap();
            elmt
        };

        let hidden_div = document.get_element_by_id("hidden-div").expect("didn't find hidden-div");
        // remove all children
        while let Some(n) = hidden_div.first_child() {
            hidden_div.remove_child(&n).unwrap();
        }

        // append x_max styled chars to hidden-div
        let mut num_chars = 0;
        for Region { s, sty , ..} in self.rich[y].iter() {
            if num_chars >= x_max { break; }
            
            let text_len = s.chars().count();
            if x_max - num_chars >= text_len {
                let tmp_span = create_span(s, sty);
                hidden_div.append_child(&tmp_span).unwrap();
                num_chars += text_len;
            } else {
                let tmp_span = create_span(&s.chars().take(x_max-num_chars).collect::<String>(), sty);
                hidden_div.append_child(&tmp_span).unwrap();
                break;
            }
        }

        let width = hidden_div.client_width();
        width as usize
    }

    fn line_height(&self) -> usize {
        self.options.line_height
    }
}

#[derive(Clone)]
pub struct Region {
    s: String,
    sty: String,
    title: String
}

impl Region {
    pub fn new(s: String, sty: String, title: String) -> Region {
        Region { s, sty, title }
    }
}