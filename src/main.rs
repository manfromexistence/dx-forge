use similar::{ChangeTag, TextDiff};
use std::fmt;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// Use lazy_static to load syntax definitions and themes once
lazy_static::lazy_static! {
    static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();
    static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();
}

struct Line(Option<usize>);

// Helper for line numbers
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(i) => write!(f, "{:<4}", i + 1),
        }
    }
}

fn show_highlighted_diff(old_text: &str, new_text: &str, syntax_extension: &str) {
    // Find the syntax definition for our file type (e.g., "tsx")
    let syntax = SYNTAX_SET
        .find_syntax_by_extension(syntax_extension)
        .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());

    // Load a theme
    let theme = &THEME_SET.themes["base16-ocean.dark"];

    // Create a text differ
    let diff = TextDiff::from_lines(old_text, new_text);

    // Create a highlighter
    let mut highlighter = HighlightLines::new(syntax, theme);

    println!(); // A little breathing room

    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            println!("{:-^1$}", "-", 80); // Separator between hunks
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, bg_color) = match change.tag() {
                    ChangeTag::Delete => ("-", "\x1b[48;5;52m"), // Red background
                    ChangeTag::Insert => ("+", "\x1b[48;5;22m"), // Green background
                    ChangeTag::Equal => (" ", "\x1b[0m"),        // No background
                };
                
                // Reset colors at the end of the line
                let reset_color = "\x1b[0m";

                print!(
                    "{}{}|{}",
                    Line(change.old_index()),
                    Line(change.new_index()),
                    bg_color
                );

                for (style, value) in highlighter
                    .highlight_line(change.value(), &SYNTAX_SET)
                    .unwrap()
                {
                    // Print the highlighted text with the correct background
                    print!("{}", as_24_bit_terminal_escaped(&style, value));
                }

                // If the line doesn't end with a newline, add one
                if !change.value().ends_with('\n') {
                    println!();
                }

                print!("{}", reset_color); // Important: reset colors
            }
        }
    }
    println!();
}


fn main() {
    let old_react_code = r#"import React from "react";

const MyComponent = () => {
  return <h1>Hello, World!</h1>;
};

export default MyComponent;
"#;

    let new_react_code = r#"import React from "react";

const MyComponent = ({ name }) => {
  return <h1>Hello, {name}!</h1>;
};

export default MyComponent;
"#;

    // Show the diff with "tsx" syntax highlighting
    show_highlighted_diff(old_react_code, new_react_code, "tsx");
}

// use dx::Text;

// fn main() {
//     let name = Text::new("What command you want to run?").prompt();

//     match name {
//         Ok(name) => println!("Command [{name}] is still in developement - it is coming soon..."),
//         Err(_) => println!("An error happened when running this command, try again later."),
//     }
// }

// mod chronicle;
// mod generator;
// mod observer;

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     println!("DX: Initializing...");

//     let chronicle_repo = match chronicle::initialize() {
//         Ok(repo) => repo,
//         Err(e) => {
//             eprintln!("DX Error: Failed to initialize the Chronicle: {}", e);
//             return Err(e);
//         }
//     };

//     if let Err(e) = observer::start(chronicle_repo.clone()).await {
//         eprintln!("DX Error: The observer failed with an error: {}", e);
//     }

//     println!("DX: Shutting down.");
//     Ok(())
// }
