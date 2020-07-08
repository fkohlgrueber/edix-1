
use crate::content::Region;
use lazy_static::lazy_static;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style, Highlighter};
use syntect::highlighting::{
    HighlightState, HighlightIterator
};
use syntect::parsing::{
    ParseState, ScopeStack
};
use crate::app::FontSelection;
use crate::content::RichContentOptions;

pub fn highlight(s: &str, options: &RichContentOptions) -> Vec<Vec<Region>> {
    lazy_static!(
        static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
        static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    );
    
    let mut content_styled: Vec<Vec<(String, String)>> = vec!();

    let syntax = SYNTAX_SET.find_syntax_by_name(&options.selected_syntax).unwrap();
    let highlighter = Highlighter::new(&THEME_SET.themes[&options.selected_theme]);
    let mut highlight_state = HighlightState::new(&highlighter, ScopeStack::new());
    let mut parse_state = ParseState::new(syntax);
    
    for line in syntect::util::LinesWithEndings::from(s) {
        let ops = parse_state.parse_line(line, &SYNTAX_SET);
        //yew::services::ConsoleService::log(&format!("{:?}", ops));
        let iter = HighlightIterator::new(&mut highlight_state, &ops[..], line, &highlighter);
        
        let ranges = iter.collect::<Vec<_>>();
        //let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        content_styled.push(ranges.into_iter().map(|(sty, s)| (s.to_owned(), style_to_css(&sty))).collect())
    }


    let mut highlight_state = HighlightState::new(&highlighter, ScopeStack::new());
    let mut parse_state = ParseState::new(syntax);
    let mut scopes = vec!();
    for line in syntect::util::LinesWithEndings::from(s) {
        let ops = parse_state.parse_line(line, &SYNTAX_SET);
        
        let mut v = vec!();
        let mut start = 0;
        for (idx, stack_op) in ops {
            if idx > start {
                let scope_str = match highlight_state.path.scopes.last() {
                    Some(x) => format!("{}",x),
                    None => format!(""),
                };
                v.push(scope_str);
                //yew::services::ConsoleService::log(&format!("Input: {:?}, {}", &line[start..idx], scope_str));
                start = idx;
            }
            highlight_state.path.apply(&stack_op);
        }
        if line.chars().count() > start {
            let scope_str = match highlight_state.path.scopes.last() {
                Some(x) => format!("{}",x),
                None => format!(""),
            };
            v.push(scope_str);
        }
        scopes.push(v);
    }

    assert_eq!(scopes.len(), content_styled.len());
    for (a, b) in scopes.iter().zip(content_styled.iter()) {
        assert_eq!(a.len(), b.len());
    }

    content_styled.into_iter().zip(scopes.into_iter()).map(
        |(a, scopes)| a.into_iter().zip(scopes.into_iter()).map(
            |((s, sty), scope)| Region::new(
                s.replace('\n', ""), 
                format!("{}{}", sty, font_for_scope(&scope, &options.font_selection)), 
                format!("{} ({})", scope, if scope_is_proportional(&scope) { "Sans" } else { "Monospace" })
            )
        ).collect()
    ).collect()
}

pub fn to_html_color(c: &syntect::highlighting::Color) -> String {
    format!("#{:02x}{:02x}{:02x}{:02x}", c.r, c.g, c.b, c.a)
}

fn style_to_css(sty: &Style) -> String {
    format!("color: {}; background-color: {}; ", to_html_color(&sty.foreground), to_html_color(&sty.background))
}

fn font_for_scope(s: &str, font_selection: &FontSelection) -> String {
    // description of scopes: https://www.sublimetext.com/docs/3/scope_naming.html
    let mono = "\"Fira Code\", monospace";
    let sans = "\"Fira\", sans-serif";
    let f = match font_selection {
        FontSelection::Monospace => mono,
        FontSelection::Sans => sans,
        FontSelection::Mixed => {
            if scope_is_proportional(s) {
                sans
            } else {
                mono
            }
        }
    };
    format!("font-family: {}; font-size: 15px;", f)
}

fn scope_is_proportional(s: &str) -> bool {
    s.starts_with("comment") 
    || s.starts_with("string") 
    || s.starts_with("constant") && !s.starts_with("constant.character.escape") 
    || s.starts_with("entity")
    || s.starts_with("variable") 
    || s.starts_with("meta.generic-name") 
    || s.starts_with("support")
}