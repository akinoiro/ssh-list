use crate::*;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Cell, Clear, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table},
};
use tui_input::Input;

pub fn render_input(app: &App, frame: &mut Frame, area: Rect, title: &str, selected_input: &Input, focused: Focus) {
    let width = area.width.max(3) - 3;
    let scroll = selected_input.visual_scroll(width as usize);
    let input = Paragraph::new(selected_input.value())
        .scroll((0, scroll as u16))
        .block(Block::bordered().title(title));
    frame.render_widget(input, area);
    let is_focused = app.focus == focused;
    if is_focused {
        let x = selected_input.visual_cursor().max(scroll) - scroll + 1;
        frame.set_cursor_position((area.x + x as u16, area.y + 1))
    }
}

pub fn render_popup(app: &mut App, frame: &mut Frame, area: Rect) {
    let title_text = match app.app_mode {
        AppMode::New => " Add new connection ",
        AppMode::Edit => " Edit connection ",
        AppMode::Move => "",
        AppMode::Normal => "",
    };

    let popup_block = Block::bordered().title(title_text).title_alignment(Alignment::Center);
    let area = popup_area(area);
    let inner = popup_block.inner(area);
    frame.render_widget(Clear, area);
    frame.render_widget(popup_block, area);

    let vertical_popup = &Layout::vertical([
        Constraint::Length(1),
        Constraint::Max(3),
        Constraint::Max(3),
        Constraint::Max(3),
        Constraint::Max(3),
        Constraint::Max(3),
        Constraint::Max(3),
    ]);
    let rects_popup = vertical_popup.split(inner);
    render_input(
        app,
        frame,
        rects_popup[1],
        " Friendly name ",
        &app.field_inputs.server_name_input,
        Focus::ServerNameField,
    );
    render_input(
        app,
        frame,
        rects_popup[2],
        " Group (e.g., Home, Work) ",
        &app.field_inputs.group_name_input,
        Focus::GroupNameField,
    );
    render_input(
        app,
        frame,
        rects_popup[3],
        " Username ",
        &app.field_inputs.username_input,
        Focus::UsernameField,
    );
    render_input(
        app,
        frame,
        rects_popup[4],
        " Hostname (IP or Domain) ",
        &app.field_inputs.hostname_input,
        Focus::HostnameField,
    );
    render_input(
        app,
        frame,
        rects_popup[5],
        " Port ",
        &app.field_inputs.port_input,
        Focus::PortField,
    );
    render_input(
        app,
        frame,
        rects_popup[6],
        " SSH option (e.g., -D 1337) ",
        &app.field_inputs.options_input,
        Focus::OptionsField,
    );
}

pub fn render_footer(app: &mut App, frame: &mut Frame, area: Rect) {
    let footer_text = match app.app_mode {
        AppMode::Normal => "[Enter] - connect | [A] - add | [E] - edit | [Del] - delete | [M] - move | [Esc] - quit",
        AppMode::New => "[Enter] - save | [Esc] - cancel",
        AppMode::Edit => "[Enter] - save | [Esc] - cancel",
        AppMode::Move => "[↓] - move down | [↑] - move up | [Esc] - back",
    };

    let info_footer = Paragraph::new(footer_text)
        .style(Style::new().fg(Color::White).bg(Color::Black))
        .centered()
        .block(
            Block::bordered()
                .border_type(BorderType::Double)
                .border_style(Style::new().fg(Color::Yellow)),
        );
    frame.render_widget(info_footer, area);
}

pub fn render_scrollbar(app: &mut App, frame: &mut Frame, area: Rect) {
    frame.render_stateful_widget(
        Scrollbar::default().orientation(ScrollbarOrientation::VerticalRight),
        area.inner(Margin {
            vertical: 0,
            horizontal: 1,
        }),
        &mut app.scroll_state,
    );
}

pub fn render_table(app: &mut App, frame: &mut Frame, area: Rect) {
    let header_style = Style::default().fg(Color::Gray).bg(Color::Indexed(235));
    let selected_row_style = Style::default().add_modifier(Modifier::REVERSED).fg(Color::Yellow);
    let header = [" Name", " Group", " Username", " Hostname", " Port", " Option"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);
    let rows = app.ssh_connections.iter().enumerate().map(|(i, data)| {
        let color = match i % 2 {
            0 => Color::Black,
            _ => Color::Indexed(235),
        };
        let item = data.ref_array();
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("\n {content}\n"))))
            .collect::<Row>()
            .style(Style::new().fg(Color::White).bg(color))
            .height(3)
    });
    let t = Table::new(
        rows,
        [
            Constraint::Min(15),
            Constraint::Min(10),
            Constraint::Min(10),
            Constraint::Min(15),
            Constraint::Min(6),
            Constraint::Min(10),
        ],
    )
    .header(header)
    .row_highlight_style(selected_row_style)
    .bg(Color::Black)
    .highlight_spacing(HighlightSpacing::Always);
    frame.render_stateful_widget(t, area, &mut app.table_state);
}
