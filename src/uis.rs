use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Gauge, Paragraph, Wrap},
    Frame,
};
pub fn render_ui<'a, B: Backend>(f: &mut Frame<B>, msg: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(30)].as_ref())
        .split(f.size());

    let (idx, _) = msg.text.as_str().char_indices().nth(msg.index).unwrap();
    let text = &msg.text[idx..];
    let typed_text = &msg.text[..idx];

    let mut all_txt = msg.get_spans(typed_text);

    let prg_label = Span::styled(
        format!(
            "{}/100",
            if text.len() < 2 {
                100
            } else {
                typed_text.len() * 100 / msg.text.len()
            }
        ),
        Style::default().fg(Color::Gray).bg(Color::Black),
    );
    let progress = Gauge::default()
        .gauge_style(Style::default().fg(Color::Green))
        .label(prg_label)
        .percent((typed_text.len() * 100 / msg.text.len()) as u16);

    f.render_widget(progress, chunks[0]);

    let first_chr_idx = text.char_indices().nth(1).unwrap_or((0, ' ')).0;
    let msng_text = Span::styled(
        &text[first_chr_idx..],
        Style::default().add_modifier(Modifier::DIM),
    );
    let cursor_chr = Span::styled(
        &text[0..first_chr_idx],
        Style::default().fg(Color::Black).bg(Color::Gray),
    );
    all_txt.push(cursor_chr);
    all_txt.push(msng_text);
    let msg_box = Paragraph::new(Spans::from(all_txt))
        .block(Block::default().title("MSG").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(msg_box, chunks[1]);

    if msg.is_complete() {
        let (block, area) = popup_result(msg, &f.size());
        f.render_widget(Clear, area);
        f.render_widget(block, area);
    }
    //f.set_cursor(chunks[1].x + post_text.width() as u16 + 1, chunks[1].y + 1);
}

pub fn popup_result<'a>(msg: &'a App, size: &Rect) -> (Paragraph<'a>, Rect) {
    let total_chrs = msg.text.chars().count();
    let elapsed = msg.time_passed;
    //tui_rs requires them to be Spans to be newlined
    let stats = vec![
        Spans::from(Span::raw(format!(
            "WPM: {:.2}",
            total_chrs as f32 / 5f32 * 60f32 / elapsed as f32
        ))),
        Spans::from(Span::raw(format!(
            "time elapsed: {}m {}s",
            elapsed / 60,
            elapsed % 60
        ))),
        Spans::from(Span::raw(format!(
            "Accuracy: {:.2}%",
            msg.misspells.clone().into_iter().filter(|x| *x).count() as f32 * 100f32
                / (total_chrs - 1) as f32
        ))),
        Spans::from(Span::raw(format!(
            "Real Accuracy: {:.2}%",
            if msg.mistakes > total_chrs as u16 {
                0f32
            } else {
                ((total_chrs as u16 - msg.mistakes) * 100) as f32 / total_chrs as f32
            }
        ))),
        Spans::from(Span::raw(format!("Total typed: {}", msg.typed))),
    ];
    let block = Paragraph::new(stats).block(Block::default().title("Result").borders(Borders::ALL));
    let area = centered_rect(20, 40, *size);

    (block, area)
}
//fn shamessly ripped from tui_rs examples
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
