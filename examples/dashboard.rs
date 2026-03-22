use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Flex, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Terminal,
};
use ratatui_braille_bar::BrailleBar;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut tick: u64 = 0;
    let mut cpu = 62.0f64;
    let cpu_peak = 78.0f64;
    let mem_bytes: f64 = 420.0 * 1024.0 * 1024.0;
    let mem_peak = 510.0 * 1024.0 * 1024.0;

    let dim = Style::default().fg(Color::Rgb(140, 140, 140));
    let bright = Style::default()
        .fg(Color::Rgb(200, 200, 200))
        .add_modifier(Modifier::BOLD);

    loop {
        terminal.draw(|frame| {
            let [_, center, _] = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(10),
                Constraint::Fill(1),
            ])
            .areas(frame.area());

            let [col] = Layout::horizontal([Constraint::Percentage(60)])
                .flex(Flex::Center)
                .areas(center);

            let rows = Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(col);

            // CPU
            frame.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled("CPU  ", dim),
                    Span::styled(format!("{:.0}%", cpu), bright),
                ])),
                rows[0],
            );
            frame.render_widget(
                BrailleBar::new(cpu, 100.0)
                    .peak(cpu_peak)
                    .fill_color(Color::Rgb(99, 102, 241)),
                rows[1],
            );

            // Memory
            let mem_mb = mem_bytes / (1024.0 * 1024.0);
            let mem_max = 1024.0 * 1024.0 * 1024.0;
            frame.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled("Memory  ", dim),
                    Span::styled(format!("{:.0} MB", mem_mb), bright),
                ])),
                rows[3],
            );
            frame.render_widget(
                BrailleBar::new(mem_bytes, mem_max)
                    .peak(mem_peak)
                    .fill_color(Color::Rgb(34, 197, 94)),
                rows[4],
            );

            // Traffic
            frame.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled("Traffic  ", dim),
                    Span::styled("1250 req/m", bright),
                ])),
                rows[6],
            );
            frame.render_widget(
                BrailleBar::new(1250.0, 5000.0)
                    .fill_color(Color::Rgb(59, 130, 246)),
                rows[7],
            );
        })?;

        if event::poll(Duration::from_millis(150))? {
            if let Event::Key(key) = event::read()? {
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                    break;
                }
            }
        }

        tick += 1;
        cpu = (cpu + (tick as f64 * 0.08).sin() * 2.0).clamp(0.0, 100.0);
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
