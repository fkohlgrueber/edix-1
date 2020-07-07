use yew::prelude::*;
use crate::controller::Controller;
use crate::content::RichContentOptions;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use yew::services::storage::{Area, StorageService};
use yew::format::Json;
use serde::{Deserialize, Serialize};

const KEY: &str = "edix1.self";
const DEFAULT_TEXT: &str = include_str!("../default-text.txt");

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    controller: Controller,
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FontSelection {
    Monospace,
    Sans,
    Mixed,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub content:String,
    pub selected_theme: String,
    pub selected_syntax: String,
    pub font_selection: FontSelection,
}

impl Default for State {
    fn default() -> State {
        State {
            content: DEFAULT_TEXT.to_string(),
            selected_theme: "InspiredGitHub".to_string(),
            selected_syntax: "Python".to_string(),
            font_selection: FontSelection::Mixed,
        }
    }
}


pub enum Msg {
    KeyDown(yew::events::KeyboardEvent),
    MouseClick(yew::events::MouseEvent),
    ThemeChange(ChangeData),
    SyntaxChange(ChangeData),
    FontChange(ChangeData),
    Reset,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let theme_set = ThemeSet::load_defaults();
        let syntax_set = SyntaxSet::load_defaults_newlines();

        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let state = {
            if let Json(Ok(restored_model)) = storage.restore(KEY) {
                let mut restored_model: State = restored_model;
                if syntax_set.find_syntax_by_name(&restored_model.selected_syntax).is_none() {
                    restored_model.selected_syntax = syntax_set.syntaxes()[0].name.to_string();
                }
                if !theme_set.themes.contains_key(&restored_model.selected_theme) {
                    restored_model.selected_theme = theme_set.themes.keys().next().unwrap().to_string();
                }
                restored_model
            } else {
                State::default()
            }
        };

        let options = RichContentOptions {
            line_height: 20,
            selected_syntax: state.selected_syntax,
            selected_theme: state.selected_theme,
            font_selection: state.font_selection
        };
        let controller = Controller::new(&state.content, options);

        let app = App {
            link,
            controller,
            storage,
            syntax_set,
            theme_set
        };

        app
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::KeyDown(e) => {
                //yew::services::ConsoleService::log(&format!("key: {}, alt: {}, shift: {}, meta: {}, ctrl: {}", e.key(), e.alt_key(), e.shift_key(), e.meta_key(), e.ctrl_key()));
                if e.alt_key() || e.ctrl_key() || e.meta_key() {
                    // ignore
                } else {
                    // HACK: prevent_default should only be called for known and handeled keys
                    loop {
                        match e.key().as_ref() {
                            "ArrowUp" => self.controller.cursor_up(),
                            "ArrowDown" => self.controller.cursor_down(),
                            "ArrowRight" => self.controller.cursor_right(),
                            "ArrowLeft" => self.controller.cursor_left(),
                            "Home" => self.controller.cursor_home(),
                            "End" => self.controller.cursor_end(),
                            "Backspace" => self.controller.key_backspace(),
                            "Enter" => self.controller.key_enter(),
                            "Delete" => self.controller.key_delete(),
                            c if c.chars().count() == 1 => self.controller.key_char(c.chars().next().unwrap()),
                            _ => { break; }
                        }
                        e.prevent_default();
                        break;
                    }
                }
            }
            Msg::MouseClick(e) => {
                let elmt = yew::utils::document().query_selector("#ed-view").unwrap().unwrap();
                let dims = elmt.get_bounding_client_rect();
                let (x, y) = (e.client_x() - dims.x() as i32, e.client_y() - dims.y() as i32);
                if x >= 0 && y >= 0 {
                    self.controller.mouse_click(x as usize, y as usize);
                }
                //e.prevent_default();
            }
            Msg::ThemeChange(cd) => {
                if let ChangeData::Select(elmt) = cd {
                    self.controller.set_theme(elmt.value());
                }
            }
            Msg::SyntaxChange(cd) => {
                if let ChangeData::Select(elmt) = cd {
                    self.controller.set_syntax(elmt.value());
                }
            }
            Msg::FontChange(cd) => {
                if let ChangeData::Select(elmt) = cd {
                    self.controller.set_font(match elmt.value().as_ref() {
                        "Monospace" => FontSelection::Monospace,
                        "Sans" => FontSelection::Sans,
                        "Mixed" => FontSelection::Mixed,
                        _ => unreachable!()
                    });
                }
            }
            Msg::Reset => {
                let state = State::default();
                let options = RichContentOptions {
                    line_height: 20,
                    selected_syntax: state.selected_syntax,
                    selected_theme: state.selected_theme,
                    font_selection: state.font_selection
                };
                let controller = Controller::new(&state.content, options);
                self.controller = controller;
            }
        }
        self.storage.store(KEY, Json(&self.controller.get_state()));
        true
    }

    fn view(&self) -> Html {
        let options = self.controller.get_options();
        let bg_color = self.theme_set.themes[&options.selected_theme].settings.background.as_ref().map(crate::highlight::to_html_color).unwrap_or("#0000".to_string());
        let fg_color = self.theme_set.themes[&options.selected_theme].settings.foreground.as_ref().map(crate::highlight::to_html_color).unwrap_or("#ffff".to_string());
        let cur_style = format!("top: {}px; left: {}px; background-color: {};", self.controller.get_y(), self.controller.get_x(), fg_color);
        html! {
            <>
            <div style="padding: 10px;">
                <span>{"Theme: "}</span><select onchange=self.link.callback(|e| Msg::ThemeChange(e))>
                    {for self.theme_set.themes.keys().map(
                        |n| html!(<option value=n selected=options.selected_theme==*n>{n}</option>)
                    )}
                </select>
                <span>{"Language: "}</span><select onchange=self.link.callback(|e| Msg::SyntaxChange(e))>
                    {for self.syntax_set.syntaxes().iter().map(
                        |s| html!(<option value=s.name selected=options.selected_syntax==s.name>{&s.name}</option>)
                    )}
                </select>
                <span>{"Font style: "}</span><select onchange=self.link.callback(|e| Msg::FontChange(e))>
                    <option value="Monospace" selected=options.font_selection==FontSelection::Monospace>{"Monospace"}</option>
                    <option value="Sans" selected=options.font_selection==FontSelection::Sans>{"Sans"}</option>
                    <option value="Mixed" selected=options.font_selection==FontSelection::Mixed>{"Mixed"}</option>
                </select>
                <button onclick=self.link.callback(|_| Msg::Reset)>{"Reset"}</button>
            </div>
            <div style="padding: 10px;">
                <div 
                    id="ed-view"
                    tabindex="0" 
                    onkeydown=self.link.callback(|e| Msg::KeyDown(e)) 
                    onmousedown=self.link.callback(|e| Msg::MouseClick(e))
                    style={format!("position:relative; background-color: {}; color: {};", bg_color, fg_color)}
                >
                    <span class="ed-cursor" style={cur_style}></span>
                    {self.controller.get_html()}
                </div>
            </div>
            </>
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
}
