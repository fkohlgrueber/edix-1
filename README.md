# edix-1: Mixing Fonts

[![LOC](https://tokei.rs/b1/github/fkohlgrueber/edix-1?category=code)](https://github.com/fkohlgrueber/edix-1)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](https://github.com/fkohlgrueber/terminal-editor-rs/blob/master/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue)](https://github.com/fkohlgrueber/terminal-editor-rs/blob/master/LICENSE-MIT)

*This project is the first part of a series of editor experiments (edix) in which I'll be exploring alternative code editing UX.*

## About

Monospace fonts are the de-facto standard for editing code. While proportional fonts have been used for documents and websites for decades, terminals and text editors intended for programming have largely remained to use monospace fonts. 

Of course, the choice of a font is a matter of taste and there won't be a single "right" one. What works for me might not work for others, and that's ok. Arguing with taste is difficult because it can easily lead to ["holy war"](https://en.wikipedia.org/wiki/Editor_war) like discussions. That being said, it seems like there are widely accepted arguments regarding proportional and monospace fonts for programming. Detailed discussions on monospace vs. proportional fonts for programming can easily be found on the internet, for example [here](https://stackoverflow.com/a/218749/3594526) and [here](https://softwareengineering.stackexchange.com/questions/5473/does-anyone-prefer-proportional-fonts). I won't reiterate all of these arguments, but instead focus on two major ones.

For general-purpose texts (documents, UI, websites, ...), most people prefer using proportional fonts. Different widths of characters make scanning words and finding typos in them easier. Compared to monospace fonts, proportional fonts also usually have a higher density, so you can fit more words on a line. On the other hand, some punctuation characters ("()[];,.") are very narrow in proportional fonts. These characters play a more important role in programming languages than in general text. Punctuation characters are often larger in monospace fonts than in proportional fonts and therefore easier to read and distinguish.

Given the two arguments above, which font should be used for programming? For things like comments, strings and identifiers, a proportional font would make more sense, but this would make punctuation characters less readable. A monospace font is a good fit for punctuation characters, but not as good for displaying comments etc.. Choosing a single font for programming therefore is a trade-off between the readability of different parts of source code. Most people choose monospace fonts and live with comments being slightly less readable, some choose proportional fonts and try to get used to the dense punctuation characters. Neither approach is optimal.

But what if you didn't have to choose a single font? what if you could use different fonts for different parts of your source code? Writing a comment would automatically change to a proportional font, while whitespace and punctuation characters would use a monospace font. This experiment explores this idea.

## The experiment

TODO

- Text field with syntax highlighting
  - although not feature complete (no selections, cut/copy/paste, ...)
- Written in Rust
- Live at ...
- uses a syntax highlighter (syntect) and defines additional rules for choosing a font based on the syntax kind identified by the highlighter

### Build

Make sure to have [Rust](https://www.rust-lang.org/) and [wasm-pack](https://github.com/rustwasm/wasm-pack) installed. Then run the following commands from the project folder:

```
wasm-pack build --target web
rollup ./main.js --format iife --file ./pkg/bundle.js
```

You can locally host the experiment, e.g. by using Python's simple http server:

```
python3 -m http.server
```


## Impressions

TODO

## Next steps

TODO
- Alignment (ref: Elastic tabstops)
- Line-wrapping for comments
- Block-layout
- ...
