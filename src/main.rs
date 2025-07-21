// src/main.rs

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use similar::{ChangeTag, TextDiff};
use std::{error::Error, io};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style as SyntectStyle, ThemeSet},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

// Mock data representing a file before and after a commit.
const OLD_CODE: &str = r#"
import React from "react";

function Greeting({ name }) {
  return <h1>Hello, {name}!</h1>;
}

export default Greeting;
"#;

const NEW_CODE: &str = r#"
import React, { useState } from "react";

function Greeting({ name }) {
  const [greeting, setGreeting] = useState(`Hello, ${name}!`);

  return (
    <div>
      <h1>{greeting}</h1>
      <button onClick={() => setGreeting("Hi there!")}>Change Greeting</button>
    </div>
  );
}

export default Greeting;
"#;


/// App holds the state of the application
struct App {
    commits: Vec<String>,
    commit_list_state: ListState,
}

impl App {
    fn new() -> App {
        // Mock data for git commits. Replace this with actual git log data.
        let commits = vec![
            "feat(auth): add google sign-in provider".to_string(),
            "fix(ui): correct button padding".to_string(),
            "docs(readme): update installation guide".to_string(),
            "refactor(state): switch to dx-store".to_string(),
            "chore: initial commit".to_string(),
        ];

        let mut commit_list_state = ListState::default();
        commit_list_state.select(Some(0)); // Select the first item by default

        App {
            commits,
            commit_list_state,
        }
    }

    // These methods handle the selection logic for the commit list
    pub fn next_commit(&mut self) {
        let i = match self.commit_list_state.selected() {
            Some(i) => {
                if i >= self.commits.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.commit_list_state.select(Some(i));
    }

    pub fn previous_commit(&mut self) {
        let i = match self.commit_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.commits.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.commit_list_state.select(Some(i));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next_commit(),
                KeyCode::Up => app.previous_commit(),
                _ => {}
            }
        }
    }
}

// NOTE: The function signature is changed to `&mut Frame` for v0.29.0
fn ui(f: &mut Frame, app: &mut App) {
    // Create two chunks with a 30/70 split
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        // NOTE: Changed from .size() to .area()
        .split(f.area());

    // Render the commit list
    render_commit_list(f, app, chunks[0]);

    // Render the diff view
    render_diff_view(f, chunks[1]);
}

// NOTE: The function signature is changed to `&mut Frame`
fn render_commit_list(f: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .commits
        .iter()
        .map(|c| ListItem::new(c.as_str()))
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Commits"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, area, &mut app.commit_list_state);
}

// NOTE: The function signature is changed to `&mut Frame`
fn render_diff_view(f: &mut Frame, area: Rect) {
    // For this example, the diff is always the same, regardless of the selected commit.
    // In a real app, you would generate this diff based on the selected commit.
    let diff_text = create_diff_text(OLD_CODE, NEW_CODE);
    
    let paragraph = Paragraph::new(diff_text)
        .block(Block::default().borders(Borders::ALL).title("Diff Preview"));
        
    f.render_widget(paragraph, area);
}

/// This is the core function that creates the highlighted diff text.
fn create_diff_text<'a>(old: &'a str, new: &'a str) -> Text<'a> {
    // Setup syntect for syntax highlighting
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    
    // FIX 1: Safely find syntax, falling back to plain text if not found.
    // This prevents the `Option::unwrap()` panic if the 'tsx' syntax is missing.
    let syntax = ps.find_syntax_by_extension("tsx")
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    let theme = &ts.themes["base16-ocean.dark"];
    let mut h = HighlightLines::new(syntax, theme);

    // Use `similar` to calculate the diff
    let diff = TextDiff::from_lines(old, new);
    let mut lines = Vec::new();

    for change in diff.iter_all_changes() {
        let (marker, style, bg_color) = match change.tag() {
            ChangeTag::Delete => (
                "-",
                Style::default().fg(Color::Red),
                Some(Color::Rgb(50, 20, 20)), // Dark red background
            ),
            ChangeTag::Insert => (
                "+",
                Style::default().fg(Color::Green),
                Some(Color::Rgb(20, 50, 20)), // Dark green background
            ),
            ChangeTag::Equal => (" ", Style::default().fg(Color::DarkGray), None),
        };

        if let Some(value) = change.as_str() {
            for line in LinesWithEndings::from(value) {
                // FIX 2: Use a match to handle potential errors during highlighting.
                // This prevents a panic if a line fails to highlight.
                let spans: Vec<Span> = match h.highlight_line(line, &ps) {
                    Ok(ranges) => ranges
                        .into_iter()
                        .map(|(style, content)| {
                            let color = style.foreground;
                            Span::styled(
                                content.to_string(),
                                Style::default().fg(Color::Rgb(color.r, color.g, color.b)),
                            )
                        })
                        .collect(),
                    Err(_) => {
                        // If highlighting fails, just show the line as plain text.
                        vec![Span::raw(line)]
                    }
                };
                
                let mut line_spans = vec![Span::styled(marker.to_string() + " ", style)];
                line_spans.extend(spans);

                let mut styled_line = Line::from(line_spans);
                if let Some(bg) = bg_color {
                    styled_line = styled_line.style(Style::default().bg(bg));
                }
                lines.push(styled_line);
            }
        }
    }

    Text::from(lines)
}
