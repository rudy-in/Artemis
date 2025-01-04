use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    cursor,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut step = 0;
    let mut spinner = 0;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(10),
                    Constraint::Percentage(75),
                    Constraint::Percentage(15),
                ])
                .split(f.area());

            let header = Paragraph::new(center_text(
                "🚀 EndeavourOS Installer",
                f.size().width as usize,
                Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD),
            ))
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::default().fg(Color::White)),
            );
            f.render_widget(header, chunks[0]);

            let content = match step {
                0 => welcome_screen(f.size().width as usize),
                1 => language_selection_screen(f.size().width as usize),
                _ => completion_screen(f.size().width as usize),
            };
            f.render_widget(content, chunks[1]);

            let footer = Paragraph::new(center_text(
                &spinner_animation(spinner),
                f.size().width as usize,
                Style::default().fg(Color::Gray),
            ))
            .block(Block::default().borders(Borders::TOP).border_style(Style::default().fg(Color::White)));
            f.render_widget(footer, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Enter => {
                    step = (step + 1) % 3;
                }
                _ => {}
            }
        }

        spinner = (spinner + 1) % 4;
        thread::sleep(Duration::from_millis(200));
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}

fn center_text<'a>(text: &str, width: usize, style: Style) -> Vec<Line<'a>> {
    let padding = (width.saturating_sub(text.len())) / 2;
    vec![Line::from(Span::styled(
        format!("{:width$}{}", "", text, width = padding),
        style,
    ))]
}

fn spinner_animation(frame: usize) -> String {
    let spinner_frames = ["|", "/", "-", "\\"];
    spinner_frames[frame].to_string()
}

fn welcome_screen(width: usize) -> Paragraph<'static> {
    Paragraph::new(vec![
        center_line("Welcome to EndeavourOS!", width, Color::Magenta, Modifier::BOLD),
        center_line("", width, Color::Reset, Modifier::empty()),
        center_line(
            "This installer will guide you through the installation process.",
            width,
            Color::Gray,
            Modifier::empty(),
        ),
        center_line("", width, Color::Reset, Modifier::empty()),
        center_line(
            "Press 'Enter' to proceed to the next step.",
            width,
            Color::LightGreen,
            Modifier::empty(),
        ),
    ])
    .block(Block::default().borders(Borders::ALL).title("🌟 Welcome"))
}

fn language_selection_screen(width: usize) -> Paragraph<'static> {
    Paragraph::new(vec![
        center_line("Select your language:", width, Color::Cyan, Modifier::BOLD),
        center_line("", width, Color::Reset, Modifier::empty()),
        center_line("→ English", width, Color::LightGreen, Modifier::empty()),
        center_line("  Français", width, Color::Gray, Modifier::empty()),
        center_line("  Español", width, Color::Gray, Modifier::empty()),
        center_line("", width, Color::Reset, Modifier::empty()),
        center_line(
            "Use arrow keys to navigate and 'Enter' to select.",
            width,
            Color::Gray,
            Modifier::empty(),
        ),
    ])
    .block(Block::default().borders(Borders::ALL).title("🌐 Language Selection"))
}

fn completion_screen(width: usize) -> Paragraph<'static> {
    Paragraph::new(vec![
        center_line(
            "Installation Complete! 🎉",
            width,
            Color::LightGreen,
            Modifier::BOLD,
        ),
        center_line("", width, Color::Reset, Modifier::empty()),
        center_line(
            "You can now restart your system and enjoy EndeavourOS.",
            width,
            Color::Gray,
            Modifier::empty(),
        ),
        center_line("", width, Color::Reset, Modifier::empty()),
        center_line("Press 'Q' to exit.", width, Color::LightCyan, Modifier::empty()),
    ])
    .block(Block::default().borders(Borders::ALL).title("✅ Completion"))
}

fn center_line<'a>(
    text: &str,
    width: usize,
    fg: Color,
    modifier: Modifier,
) -> Line<'a> {
    let padding = (width.saturating_sub(text.len())) / 2;
    Line::from(Span::styled(
        format!("{:width$}{}", "", text, width = padding),
        Style::default().fg(fg).add_modifier(modifier),
    ))
}
