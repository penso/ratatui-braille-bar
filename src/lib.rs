//! # ratatui-braille-bar
//!
//! Once-style braille progress bars for [ratatui](https://ratatui.rs).
//!
//! Renders a horizontal bar using braille characters (`⢾⣿⣿⣿⣿⣿⡷`) with
//! rounded end caps, a fill color, an optional peak marker, and a dim
//! empty region — inspired by [Basecamp's Once](https://github.com/basecamp/once)
//! dashboard meters.
//!
//! ## Quick start
//!
//! ```rust,ignore
//! use ratatui_braille_bar::BrailleBar;
//!
//! // Inside your ratatui draw closure:
//! frame.render_widget(
//!     BrailleBar::new(62.0, 100.0)
//!         .peak(78.0)
//!         .fill_color(Color::Rgb(99, 102, 241)),
//!     area,
//! );
//! ```
//!
//! The bar automatically fills `area.width` — no manual width needed.

use rand::Rng;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

const BAR_FULL: &str = "\u{28FF}"; // ⣿
const BAR_ROUND_LEFT: &str = "\u{28BE}"; // ⢾
const BAR_ROUND_RIGHT: &str = "\u{2877}"; // ⡷

/// A braille progress bar widget for ratatui.
///
/// Renders a single-row bar of braille characters with rounded end caps.
/// Supports an optional peak marker (e.g. for showing max CPU over a window).
///
/// # Example
///
/// ```rust,no_run
/// use ratatui::style::Color;
/// use ratatui_braille_bar::BrailleBar;
///
/// let bar = BrailleBar::new(0.62, 1.0)
///     .peak(0.78)
///     .fill_color(Color::Rgb(99, 102, 241))
///     .peak_color(Color::Rgb(251, 146, 60))
///     .empty_color(Color::Rgb(60, 60, 60));
/// ```
#[derive(Debug, Clone)]
pub struct BrailleBar {
    current: f64,
    peak: f64,
    scale_max: f64,
    fill_color: Color,
    peak_color: Color,
    empty_color: Color,
}

impl BrailleBar {
    /// Create a new bar. `current` is the value to fill, `scale_max` is the
    /// upper bound of the scale (e.g. 100.0 for percentages).
    pub fn new(current: f64, scale_max: f64) -> Self {
        Self {
            current,
            peak: 0.0,
            scale_max,
            fill_color: Color::Rgb(99, 102, 241),
            peak_color: Color::Rgb(251, 146, 60),
            empty_color: Color::Rgb(60, 60, 60),
        }
    }

    /// Set a peak marker position (e.g. max value over a sliding window).
    /// Rendered as a single cell in `peak_color`. Set to 0.0 to disable.
    pub fn peak(mut self, peak: f64) -> Self {
        self.peak = peak;
        self
    }

    /// Color for filled cells. Default: indigo `#6366F1`.
    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }

    /// Color for the peak marker cell. Default: orange `#FB923C`.
    pub fn peak_color(mut self, color: Color) -> Self {
        self.peak_color = color;
        self
    }

    /// Color for empty (unfilled) cells. Default: dark gray `#3C3C3C`.
    pub fn empty_color(mut self, color: Color) -> Self {
        self.empty_color = color;
        self
    }

    /// Render to a [`Line`] with explicit width. Useful when composing
    /// bars into larger widgets without going through the `Widget` trait.
    pub fn into_line(self, width: usize) -> Line<'static> {
        if width == 0 {
            return Line::default();
        }

        let filled = ((self.current / self.scale_max) * width as f64) as usize;
        let filled = filled.min(width);

        let mut peak_pos = ((self.peak / self.scale_max) * width as f64) as usize;
        peak_pos = peak_pos.min(width.saturating_sub(1));

        let mut cell_styles = vec![Style::default().fg(self.empty_color); width];

        if self.peak > 0.0 {
            peak_pos = peak_pos.max(filled.saturating_sub(1));
            peak_pos = peak_pos.min(width - 1);
            for (i, style) in cell_styles.iter_mut().enumerate().take(width) {
                if i == peak_pos {
                    *style = Style::default().fg(self.peak_color);
                } else if i < filled {
                    *style = Style::default().fg(self.fill_color);
                }
            }
        } else {
            for style in cell_styles.iter_mut().take(filled) {
                *style = Style::default().fg(self.fill_color);
            }
        }

        let spans: Vec<Span<'static>> = cell_styles
            .iter()
            .enumerate()
            .map(|(i, style)| {
                let ch = match i {
                    0 => BAR_ROUND_LEFT,
                    _ if i == width - 1 => BAR_ROUND_RIGHT,
                    _ => BAR_FULL,
                };
                Span::styled(ch.to_string(), *style)
            })
            .collect();

        Line::from(spans)
    }
}

impl Widget for BrailleBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.into_line(area.width as usize)).render(area, buf);
    }
}

/// A random braille dot pattern widget, used as an indeterminate spinner.
///
/// Fills the area with random braille characters (U+2800–U+28FF), producing
/// an animated shimmer effect when re-rendered each frame — just like the
/// "Preparing..." state in Basecamp's Once installer.
///
/// # Example
///
/// ```rust,ignore
/// // Re-render every ~50ms for the animated effect
/// frame.render_widget(
///     BrailleSpinner::new().color(Color::Rgb(99, 102, 241)),
///     area,
/// );
/// ```
#[derive(Debug, Clone)]
pub struct BrailleSpinner {
    color: Color,
}

impl BrailleSpinner {
    /// Create a new spinner with the default indigo color.
    pub fn new() -> Self {
        Self {
            color: Color::Rgb(99, 102, 241),
        }
    }

    /// Set the color for the braille dots.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Render to a [`Line`] with explicit width.
    pub fn into_line(self, width: usize) -> Line<'static> {
        if width == 0 {
            return Line::default();
        }

        let style = Style::default().fg(self.color);
        let mut rng = rand::rng();
        let spans: Vec<Span<'static>> = (0..width)
            .map(|_| {
                let ch = char::from_u32(0x2800 + rng.random_range(0..256)).unwrap_or('⣿');
                Span::styled(ch.to_string(), style)
            })
            .collect();

        Line::from(spans)
    }
}

impl Default for BrailleSpinner {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for BrailleSpinner {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.into_line(area.width as usize)).render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_bar() {
        let line = BrailleBar::new(0.0, 100.0).into_line(10);
        assert_eq!(line.spans.len(), 10);
    }

    #[test]
    fn full_bar() {
        let line = BrailleBar::new(100.0, 100.0).into_line(10);
        // All cells should be fill_color
        for span in &line.spans {
            assert_eq!(span.style.fg, Some(Color::Rgb(99, 102, 241)));
        }
    }

    #[test]
    fn half_bar() {
        let line = BrailleBar::new(50.0, 100.0).into_line(10);
        assert_eq!(line.spans.len(), 10);
        // First 5 filled, last 5 empty
        assert_eq!(line.spans[0].style.fg, Some(Color::Rgb(99, 102, 241)));
        assert_eq!(line.spans[9].style.fg, Some(Color::Rgb(60, 60, 60)));
    }

    #[test]
    fn peak_marker() {
        let line = BrailleBar::new(30.0, 100.0).peak(70.0).into_line(10);
        // Cell at position 7 should be peak color
        assert_eq!(line.spans[7].style.fg, Some(Color::Rgb(251, 146, 60)));
    }

    #[test]
    fn rounded_caps() {
        let line = BrailleBar::new(50.0, 100.0).into_line(5);
        assert_eq!(line.spans[0].content.as_ref(), "⢾");
        assert_eq!(line.spans[4].content.as_ref(), "⡷");
        assert_eq!(line.spans[2].content.as_ref(), "⣿");
    }

    #[test]
    fn zero_width() {
        let line = BrailleBar::new(50.0, 100.0).into_line(0);
        assert!(line.spans.is_empty());
    }
}
