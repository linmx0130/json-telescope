use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Screen};
use crate::json_tree::{NodeKind, PathSegment, TreeState, VisibleNode};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    match &mut app.screen {
        Screen::List => draw_list(frame, app, area),
        Screen::Inspect(tree) => draw_inspect(frame, tree, area),
    }
}

fn draw_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let content_area = chunks[0];
    let help_area = chunks[1];

    let items: Vec<ListItem> = app
        .entries
        .iter()
        .enumerate()
        .map(|(i, val)| {
            let preview = format_value_preview(val);
            let content = format!("{:>4}: {}", i, preview);
            let mut item = ListItem::new(content);
            if i == app.list_selected {
                item = item.style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                );
            }
            item
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title(" JSONL Entries ").borders(Borders::ALL));

    frame.render_widget(list, content_area);

    let help =
        Paragraph::new("↑/↓ or k/j: scroll | Enter/l: inspect | q: quit")
            .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, help_area);
}

fn format_value_preview(val: &serde_json::Value) -> String {
    let s = match val {
        serde_json::Value::Object(map) => format!("{{ {} fields }}", map.len()),
        serde_json::Value::Array(arr) => format!("[ {} items ]", arr.len()),
        other => other.to_string(),
    };
    if s.len() > 120 {
        format!("{}...", &s[..117])
    } else {
        s
    }
}

fn draw_inspect(frame: &mut Frame, tree: &mut TreeState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let content_area = chunks[0];
    let help_area = chunks[1];

    let visible = tree.flatten();
    tree.clamp_selected(visible.len());
    tree.ensure_visible(content_area.height as usize);

    let lines: Vec<Line> = visible
        .iter()
        .enumerate()
        .map(|(i, node)| {
            let mut spans = render_node(node);
            if i == tree.selected {
                spans = spans
                    .into_iter()
                    .map(|s| {
                        s.patch_style(
                            Style::default()
                                .bg(Color::DarkGray)
                                .add_modifier(Modifier::BOLD),
                        )
                    })
                    .collect();
            }
            Line::from(spans)
        })
        .collect();

    let paragraph = Paragraph::new(lines)
        .block(Block::default().title(" Inspect Entry ").borders(Borders::ALL))
        .scroll((tree.scroll_offset as u16, 0))
        .wrap(if tree.wrap { Wrap { trim: true } } else { Wrap::default() });

    frame.render_widget(paragraph, content_area);

    let help = Paragraph::new(
        "↑/↓ or k/j: scroll | Enter/→/l: expand | ←/h: collapse | w: toggle wrap | q/Esc: back",
    )
    .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, help_area);
}

fn render_node(node: &VisibleNode) -> Vec<Span<'static>> {
    let mut spans = Vec::new();

    for _ in 0..node.depth {
        spans.push(Span::raw("  "));
    }

    if !node.path.is_empty() {
        match node.path.last().unwrap() {
            PathSegment::Key(k) => {
                spans.push(Span::styled(
                    format!("{}: ", k),
                    Style::default().fg(Color::Cyan),
                ));
            }
            PathSegment::Index(i) => {
                spans.push(Span::styled(
                    format!("[{}]: ", i),
                    Style::default().fg(Color::Yellow),
                ));
            }
        }
    }

    match &node.kind {
        NodeKind::Object { len, expanded } => {
            let chevron = if *expanded { "▾ " } else { "▸ " };
            spans.push(Span::raw(chevron));
            spans.push(Span::styled(
                format!("{{ {} fields }}", len),
                Style::default().fg(Color::DarkGray),
            ));
        }
        NodeKind::Array { len, expanded } => {
            let chevron = if *expanded { "▾ " } else { "▸ " };
            spans.push(Span::raw(chevron));
            spans.push(Span::styled(
                format!("[ {} items ]", len),
                Style::default().fg(Color::DarkGray),
            ));
        }
        NodeKind::Primitive(val) => {
            spans.push(primitive_span(val));
        }
    }

    spans
}

fn primitive_span(val: &serde_json::Value) -> Span<'static> {
    match val {
        serde_json::Value::String(s) => {
            Span::styled(format!("\"{}\"", s), Style::default().fg(Color::Green))
        }
        serde_json::Value::Number(_) => {
            Span::styled(val.to_string(), Style::default().fg(Color::Yellow))
        }
        serde_json::Value::Bool(_) => {
            Span::styled(val.to_string(), Style::default().fg(Color::Magenta))
        }
        serde_json::Value::Null => Span::styled("null", Style::default().fg(Color::DarkGray)),
        _ => unreachable!(),
    }
}
