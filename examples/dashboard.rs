use std::{io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Flex, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Terminal,
};
use ratatui_braille_bar::{BrailleBar, BrailleSpinner};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut tick: u64 = 0;
    let mut cpu = 62.0f64;
    let cpu_peak = 78.0f64;
    let mut mem_bytes: f64 = 420.0 * 1024.0 * 1024.0;
    let mem_peak = 510.0 * 1024.0 * 1024.0;
    let mut traffic = 1250.0f64;

    let dim = Style::default().fg(Color::Rgb(140, 140, 140));
    let bright = Style::default()
        .fg(Color::Rgb(200, 200, 200))
        .add_modifier(Modifier::BOLD);

    loop {
        terminal.draw(|frame| {
            let [_, center, _] = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(11),
                Constraint::Fill(1),
            ])
            .areas(frame.area());

            let [col] = Layout::horizontal([Constraint::Percentage(60)])
                .flex(Flex::Center)
                .areas(center);

            let rows = Layout::vertical([
                Constraint::Length(1), // 0: CPU label
                Constraint::Length(1), // 1: CPU bar
                Constraint::Length(1), // 2: blank
                Constraint::Length(1), // 3: Memory label
                Constraint::Length(1), // 4: Memory bar
                Constraint::Length(1), // 5: blank
                Constraint::Length(1), // 6: Traffic label
                Constraint::Length(1), // 7: Traffic bar
                Constraint::Length(1), // 8: blank
                Constraint::Length(1), // 9: Preparing label
                Constraint::Length(1), // 10: Preparing spinner
            ])
            .split(col);

            // CPU
            frame.render_widget(
                Paragraph::new(Line::from(vec![Span::styled("CPU", dim)])),
                rows[0],
            );
            frame.render_widget(
                Paragraph::new(Line::from(vec![Span::styled(
                    format!("{:.0}%", cpu),
                    bright,
                )]))
                .alignment(Alignment::Right),
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
                Paragraph::new(Line::from(vec![Span::styled("Memory", dim)])),
                rows[3],
            );
            frame.render_widget(
                Paragraph::new(Line::from(vec![Span::styled(
                    format!("{:.0} MB", mem_mb),
                    bright,
                )]))
                .alignment(Alignment::Right),
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
                Paragraph::new(Line::from(vec![Span::styled("Traffic", dim)])),
                rows[6],
            );
            frame.render_widget(
                Paragraph::new(Line::from(vec![Span::styled(
                    format!("{:.0} req/m", traffic),
                    bright,
                )]))
                .alignment(Alignment::Right),
                rows[6],
            );
            frame.render_widget(
                BrailleBar::new(traffic, 5000.0).fill_color(Color::Rgb(59, 130, 246)),
                rows[7],
            );

            // Preparing (spinner)
            frame.render_widget(
                Paragraph::new(Line::from(vec![Span::styled("Preparing...", dim)])),
                rows[9],
            );
            frame.render_widget(
                BrailleSpinner::new().color(Color::Rgb(99, 102, 241)),
                rows[10],
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
        let t = tick as f64;
        cpu = (cpu + (t * 0.08).sin() * 2.0).clamp(0.0, 100.0);
        mem_bytes = (mem_bytes + (t * 0.05).cos() * 5.0 * 1024.0 * 1024.0)
            .clamp(200.0 * 1024.0 * 1024.0, 800.0 * 1024.0 * 1024.0);
        traffic = (traffic + (t * 0.12).sin() * 30.0).clamp(200.0, 4500.0);
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
