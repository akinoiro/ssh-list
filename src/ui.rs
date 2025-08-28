use crate::*;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Cell, Clear, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation, Table},
};

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
        _ => "",
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
        " SSH options (e.g., -D 1337) ",
        &app.field_inputs.options_input,
        Focus::OptionsField,
    );
}

pub fn render_footer(app: &mut App, frame: &mut Frame, area: Rect) {
    let footer_text = match app.app_mode {
        AppMode::Normal => 
            if app.ssh_connections.is_empty() {
                "[A] add | [I] import | [Esc] quit"
            }
            else {
                "[Enter] connect | [R] run  | [/] search | [I] import | [Esc] quit  \n    [A] add     | [E] edit | [C] copy   | [M] move   | [Del] delete"
            }
        AppMode::New => "[Enter] save | [Esc] cancel",
        AppMode::Edit => "[Enter] save | [Esc] cancel",
        AppMode::Move => "[↓] move down | [↑] move up | [Esc] back",
        AppMode::ImportExport => "[I] import | [Esc] back",
        AppMode::Error => "[Esc] back",
        AppMode::RunCommand => "[Enter] run command | [Esc] back",
        AppMode::Search => "[Enter] connect | [Ctrl+R] run | [Ctrl+E] edit | [Del] delete | [Esc] back",
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

fn get_constraint(app: &App) -> Vec<Constraint> {
    let server_name_len = app.ssh_connections.iter().map(|i| i.server_name.len()).max().unwrap_or(0);
    let group_name_len = app.ssh_connections.iter().map(|i| i.group_name.len()).max().unwrap_or(0);
    let username_len = app.ssh_connections.iter().map(|i| i.username.len()).max().unwrap_or(0);
    let hostname_len = app.ssh_connections.iter().map(|i| i.hostname.len()).max().unwrap_or(0);

    vec![
            Constraint::Length(server_name_len.clamp(15, 30) as u16),
            Constraint::Length(group_name_len.clamp(15, 30) as u16),
            Constraint::Length(username_len.clamp(15, 30) as u16),
            Constraint::Length(hostname_len.clamp(15, 50) as u16),
            Constraint::Length(10), //port
            Constraint::Min(10),    //options
        ]
}

pub fn render_table(app: &mut App, frame: &mut Frame, area: Rect) {
    let header_style = Style::default().fg(Color::Gray).bg(Color::Indexed(235));
    let selected_row_style = Style::default().add_modifier(Modifier::REVERSED).fg(Color::Yellow);
    let header = [" Name", " Group", " Username", " Hostname", " Port", " Options"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);
    let mut rows = Vec::new();
    if app.app_mode == AppMode::Normal
        || app.app_mode == AppMode::New
        || app.app_mode == AppMode::Move
        || app.app_mode == AppMode::ImportExport  {
        for (i, data) in app.ssh_connections.iter().enumerate() {
            let color = match i % 2 {
                0 => Color::Black,
                _ => Color::Indexed(235),
            };
            let item = data.ref_array();
            let row = item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n {content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(Color::White).bg(color))
                .height(3);
            rows.push(row);
        }
    } else {
        for (i, &index) in app.search_index.iter().enumerate() {
            let data = &app.ssh_connections[index];
            let color = match i % 2 {
                0 => Color::Black,
                _ => Color::Indexed(235),
            };
            let item = data.ref_array();
            let row = item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n {content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(Color::White).bg(color))
                .height(3);
            rows.push(row);
        }
    }
    let t = Table::new(
        rows,
        get_constraint(&app),
    )
    .header(header)
    .row_highlight_style(selected_row_style)
    .bg(Color::Black)
    .highlight_spacing(HighlightSpacing::Always);
    frame.render_stateful_widget(t, area, &mut app.table_state);
}

pub fn render_config_popup(frame: &mut Frame, area: Rect) {
    let title_text = " Config Settings ";
    let popup_block = Block::bordered().title(title_text).title_alignment(Alignment::Center);
    let area = config_popup_area(area);
    let inner = popup_block.inner(area);
    frame.render_widget(Clear, area);
    frame.render_widget(popup_block, area);

    let vertical_popup = &Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(2), // text1
        Constraint::Length(1),
        Constraint::Length(2), // text2
    ]);
    let rects_popup = vertical_popup.split(inner);

    let text1 = format!("Press [I] to import connections from:\n{}",parse::get_sshconfig_path().display());
    let info_footer = Paragraph::new(text1)
        .style(Style::new().fg(Color::White))
        .centered();
    frame.render_widget(info_footer, rects_popup[1]);

    let text2 = "Username, hostname, port, and \nidentity file will be imported";
    let info_footer = Paragraph::new(text2)
        .style(Style::new().fg(Color::White))
        .centered();
    frame.render_widget(info_footer, rects_popup[3]);
}

pub fn render_error_popup(frame: &mut Frame, area: Rect, error_text: String) {
    let title_text = " Error ";
    let popup_block = Block::bordered().title(title_text).title_alignment(Alignment::Center);
    let area = error_popup_area(area);
    let inner = popup_block.inner(area);
    frame.render_widget(Clear, area);
    frame.render_widget(popup_block, area);

    let vertical_popup = &Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(2),
    ]);
    let rects_popup = vertical_popup.split(inner);

    let text1 = format!("{}",error_text);
    let info_footer = Paragraph::new(text1)
        .style(Style::new().fg(Color::White))
        .centered();
    frame.render_widget(info_footer, rects_popup[1]);

}

pub fn render_run_popup(app: &mut App, frame: &mut Frame, area: Rect) {
    let popup_block = Block::new();
    frame.render_widget(Clear, run_popup_area(area));
    frame.render_widget(popup_block, run_popup_area(area));

    render_input(
        app,
        frame,
        run_popup_area(area),
        " Command (e.g., uptime) ",
        &app.run_input,
        Focus::RunField,
    );
}

pub fn render_search(app: &mut App, frame: &mut Frame, area: Rect) {
    let popup_block = Block::new();
    frame.render_widget(popup_block, search_area(area));
    render_input(
        app,
        frame,
        search_area(area),
        " Search ",
        &app.search_input,
        Focus::SearchField,
    );
}