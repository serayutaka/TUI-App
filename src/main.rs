use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture,Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

use tokio::time::sleep as delay_for;

use std::time::Duration;

use isahc::{http::StatusCode, HttpClient};

pub enum InputMode {
    Normal,
    InsertingName,
    InsertingEmail,
    InsertingWebsite,
    InsertingHour,
    InsertingMinute,
}

pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    pub input_name: String,
    pub input_email: String,
    pub input_website: String,
    pub cursor_position_name: usize,
    pub cursor_position_email: usize,
    pub cursor_position_website: usize,
    pub input_mode: InputMode,
    pub name: String,
    pub email: String,
    pub website: String,
    pub hr: String,
    pub min: String,
    pub hr_items: StatefulList<&'a str>,
    pub min_items: StatefulList<&'a str>,
    pub ending: bool,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            titles: vec!["Home", "Check", "About", "Quit"],
            index: 0,
            input_name: String::new(),
            input_email: String::new(),
            input_website: String::new(),
            cursor_position_name: 0,
            cursor_position_email: 0,
            cursor_position_website: 0,
            input_mode: InputMode::Normal,
            name: String::new(),
            email: String::new(),
            website: String::new(),
            hr: String::new(),
            min: String::new(),
            hr_items: StatefulList::with_items(vec![
                "01","02","03","04","05","06",
                "07","08","09","10","11","12",
                "13","14","15","16","17","18",
                "19","20","21","22","23","24",
            ]),
            min_items: StatefulList::with_items(vec![
                "00","01","02","03","04","05","06","07","08","09",
                "10","11","12","13","14","15","16","17","18","19",
                "20","21","22","23","24","25","26","27","28","29",
                "30","31","32","33","34","35","36","37","38","39",
                "40","41","42","43","44","45","46","47","48","49",
                "50","51","52","53","54","55","56","57","58","59",
            ]),
            ending: false,
        }
    }

    pub fn c_render(&mut self) {
        self.index = 1;
    }

    pub fn a_render(&mut self) {
        self.index = 2;
    }

    pub fn h_render(&mut self) {
        self.index = 0;
    }

    pub fn move_cursor_left_name(&mut self) {
        let cursor_moved_left = self.cursor_position_name.saturating_sub(1);
        self.cursor_position_name = self.clamp_cursor_name(cursor_moved_left);
    }

    pub fn move_cursor_right_name(&mut self) {
        let cursor_moved_right = self.cursor_position_name.saturating_add(1);
        self.cursor_position_name = self.clamp_cursor_name(cursor_moved_right);
    }

    pub fn move_cursor_left_email(&mut self) {
        let cursor_moved_left = self.cursor_position_email.saturating_sub(1);
        self.cursor_position_email = self.clamp_cursor_email(cursor_moved_left);
    }

    pub fn move_cursor_right_email(&mut self) {
        let cursor_moved_right = self.cursor_position_email.saturating_add(1);
        self.cursor_position_email = self.clamp_cursor_email(cursor_moved_right);
    }

    pub fn move_cursor_left_website(&mut self) {
        let cursor_moved_left = self.cursor_position_website.saturating_sub(1);
        self.cursor_position_website = self.clamp_cursor_website(cursor_moved_left);
    }

    pub fn move_cursor_right_website(&mut self) {
        let cursor_moved_right = self.cursor_position_website.saturating_add(1);
        self.cursor_position_website = self.clamp_cursor_website(cursor_moved_right);
    }

    pub fn enter_char_name(&mut self, new_char: char) {
        self.input_name.insert(self.cursor_position_name, new_char);

        self.move_cursor_right_name();
    }

    pub fn enter_char_email(&mut self, new_char: char) {
        self.input_email.insert(self.cursor_position_email, new_char);

        self.move_cursor_right_email();
    }

    pub fn enter_char_website(&mut self, new_char: char) {
        self.input_website.insert(self.cursor_position_website, new_char);

        self.move_cursor_right_website();
    }

    pub fn delete_char_name(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position_name != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position_name;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input_name.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input_name.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input_name = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left_name();
        }
    }

    pub fn delete_char_email(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position_email != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position_email;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input_email.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input_email.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input_email = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left_email();
        }
    }

    pub fn delete_char_website(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position_website != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position_website;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input_website.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input_website.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input_website = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left_website();
        }
    }

    pub fn clamp_cursor_name(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input_name.len())
    }

    pub fn clamp_cursor_email(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input_email.len())
    }

    pub fn clamp_cursor_website(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input_website.len())
    }

    pub fn reset_cursor_name(&mut self) {
        self.cursor_position_name = 0;
    }

    pub fn reset_cursor_email(&mut self) {
        self.cursor_position_email = 0;
    }

    pub fn reset_cursor_website(&mut self) {
        self.cursor_position_website = 0;
    }

    pub fn submit_name(&mut self) {
        self.name.push_str(&self.input_name.clone());
        self.input_name.clear();
        self.reset_cursor_name();
    }

    pub fn submit_email(&mut self) {
        self.email.push_str(&self.input_email.clone());
        self.input_email.clear();
        self.reset_cursor_email();
    }

    pub fn submit_website(&mut self) {
        self.website.push_str(&self.input_website.clone());
        self.input_website.clear();
        self.reset_cursor_website();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

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

        if app.ending == true { return Ok(()) }

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('h') => app.h_render(),
                    KeyCode::Char('c') => {
                        app.c_render();
                        app.input_mode = InputMode::InsertingName;
                    }
                    KeyCode::Char('a') => app.a_render(),
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                },
                InputMode::InsertingName if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        app.submit_name();
                        app.input_mode = InputMode::InsertingEmail;
                    }
                    KeyCode::Char(to_insert) => {
                        app.enter_char_name(to_insert);
                    }
                    KeyCode::Backspace => {
                        app.delete_char_name();
                    }
                    KeyCode::Left => {
                        app.move_cursor_left_name();
                    }
                    KeyCode::Right => {
                        app.move_cursor_right_name();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {},
                },
                InputMode::InsertingEmail if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        app.submit_email();
                        app.input_mode = InputMode::InsertingWebsite;
                    }
                    KeyCode::Char(to_insert) => {
                        app.enter_char_email(to_insert);
                    }
                    KeyCode::Backspace => {
                        app.delete_char_email();
                    }
                    KeyCode::Left => {
                        app.move_cursor_left_email();
                    }
                    KeyCode::Right => {
                        app.move_cursor_right_email();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {},
                },
                InputMode::InsertingWebsite if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        app.submit_website();
                        app.input_mode = InputMode::InsertingHour;
                        app.hr_items.next();
                    }
                    KeyCode::Char(to_insert) => {
                        app.enter_char_website(to_insert);
                    }
                    KeyCode::Backspace => {
                        app.delete_char_website();
                    }
                    KeyCode::Left => {
                        app.move_cursor_left_website();
                    }
                    KeyCode::Right => {
                        app.move_cursor_right_website();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {},
                },
                InputMode::InsertingHour if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        app.hr.push_str(&app.hr_items.items[app.hr_items.state.selected().unwrap()]);
                        app.input_mode = InputMode::InsertingMinute;
                        app.min_items.next();
                    }
                    KeyCode::Up => {
                        app.hr_items.previous()
                    }
                    KeyCode::Down => {
                        app.hr_items.next()
                    }
                    _ => {},
                },
                InputMode::InsertingMinute if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        app.min.push_str(&app.min_items.items[app.min_items.state.selected().unwrap()]);
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Up => {
                        app.min_items.previous()
                    }
                    KeyCode::Down => {
                        app.min_items.next()
                    }
                    _ => {},
                },
                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    
    let chunks1 = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
            ].as_ref()
        )
        .split(size);

    let menu = app.titles.iter().cloned()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow).add_modifier(Modifier::UNDERLINED)),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    let tabs = Tabs::new(menu)
        .select(app.index)
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));

    let thx = Paragraph::new("Made with 💖 from @serayutaka")
        .block(Block::new()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White)))
        .style(Style::default().fg(Color::Rgb(31, 81, 255)))
        .alignment(Alignment::Center);

    f.render_widget(tabs, chunks1[0]);
    f.render_widget(thx, chunks1[2]);

    let input_name = Paragraph::new(app.input_name.as_str())
    .style(match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::InsertingName => Style::default().fg(Color::Yellow),
        InputMode::InsertingEmail => Style::default(),
        InputMode::InsertingWebsite => Style::default(),
        InputMode::InsertingHour => Style::default(),
        InputMode::InsertingMinute => Style::default(),
    })
    .block(Block::default().borders(Borders::ALL).title("Name"));

    let input_email = Paragraph::new(app.input_email.as_str())
    .style(match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::InsertingName => Style::default(),
        InputMode::InsertingEmail => Style::default().fg(Color::Yellow),
        InputMode::InsertingWebsite => Style::default(),
        InputMode::InsertingHour => Style::default(),
        InputMode::InsertingMinute => Style::default(),
    })
    .block(Block::default().borders(Borders::ALL).title("Email"));

    let input_website = Paragraph::new(app.input_website.as_str())
    .style(match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::InsertingName => Style::default(),
        InputMode::InsertingEmail => Style::default(),
        InputMode::InsertingWebsite => Style::default().fg(Color::Yellow),
        InputMode::InsertingHour => Style::default(),
        InputMode::InsertingMinute => Style::default(),
    })
    .block(Block::default().borders(Borders::ALL).title("Website"));

    let hr_items: Vec<ListItem> = app
        .hr_items
        .items
        .iter()
        .map(
            |i| {
                ListItem::new(*i).style(Style::default())
            },
        )
        .collect();
    let hr_list = List::new(hr_items)
        .block(Block::default().title("HR").borders(Borders::ALL))
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::InsertingName => Style::default(),
            InputMode::InsertingEmail => Style::default(),
            InputMode::InsertingWebsite => Style::default(),
            InputMode::InsertingHour => Style::default().fg(Color::Yellow),
            InputMode::InsertingMinute => Style::default(),
            }
        )
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    let min_items: Vec<ListItem> = app
        .min_items
        .items
        .iter()
        .map(
            |i| {
                ListItem::new(*i).style(Style::default())
            },
        )
        .collect();
    let min_list = List::new(min_items)
        .block(Block::default().title("MIN").borders(Borders::ALL))
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::InsertingName => Style::default(),
            InputMode::InsertingEmail => Style::default(),
            InputMode::InsertingWebsite => Style::default(),
            InputMode::InsertingHour => Style::default(),
            InputMode::InsertingMinute => Style::default().fg(Color::Yellow),
            }
        )
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    match app.index {
        0 => f.render_widget(render_home(), chunks1[1]),
        1 => {
            let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]).split(chunks1[1]);
            
            let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Percentage(25),
            ]).split(main_chunks[0]);

            let mid_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Percentage(25),
            ]).split(main_chunks[1]);

            f.render_widget(input_name, left_chunks[0]);
            f.render_widget(input_email, left_chunks[1]);
            f.render_widget(input_website, left_chunks[2]);

            let left_time_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(left_chunks[3]);

            f.render_stateful_widget(hr_list, left_time_chunks[0], &mut app.hr_items.state);
            f.render_stateful_widget(min_list, left_time_chunks[1], &mut app.min_items.state);

            let name_para = Paragraph::new(format!("Name : {}",app.name.as_str()))
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left);
            f.render_widget(name_para, mid_chunks[0]);

            let email_para = Paragraph::new(format!("Email : {}",app.email.as_str()))
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left);
            f.render_widget(email_para, mid_chunks[1]);

            let website_para = Paragraph::new(format!("Website : {}",app.website.as_str()))
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left);
            f.render_widget(website_para, mid_chunks[2]);

            let mut time_para = Paragraph::new(format!("\n    Notify When -> {} : {}",app.hr.as_str(),app.min.as_str()))
                .block(Block::default().title("Time").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left);
            f.render_widget(time_para, mid_chunks[3]);

            let mut messages_para = Paragraph::new("")
                .block(Block::default().title("Messages").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left);
            f.render_widget(messages_para, main_chunks[2]);
            
            let network_para = Paragraph::new("")
                .block(Block::default().title("Network").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left);
            f.render_widget(network_para, main_chunks[3]);

            match app.input_mode {
                InputMode::Normal => {
                    time_para = Paragraph::new(format!("\n    Notify When -> {} : {}",app.hr.as_str(),app.min.as_str()))
                        .block(Block::default().title("Time").borders(Borders::ALL))
                        .style(Style::default().fg(Color::White))
                        .alignment(Alignment::Left);

                    let stdout = io::stdout();
                    let backend = CrosstermBackend::new(stdout);
                    let mut terminal = Terminal::new(backend);

                    let mut message_vec = vec!["Checking Network Connection...", "Checking Network Connection...\n.", "Checking Network Connection...\n.\n.", "Checking Network Connection...\n.\n.\n."];

                    for string in &message_vec {
                        messages_para = Paragraph::new(string.to_string())
                            .block(Block::default().title("Messages").borders(Borders::ALL))
                            .style(Style::default().fg(Color::White))
                            .alignment(Alignment::Left);
                        terminal.as_mut().expect("REASON").draw(|f| {
                            f.render_widget(time_para.clone(), mid_chunks[3]);
                            f.render_widget(messages_para, main_chunks[2]);
                        }).unwrap();

                        delay_for(Duration::from_millis(1000)).await;
                    }

                    if is_internet_connected() {
                        let message_vec_ext = vec!["Checking Network Connection...\n.\n.\n.\nConnected!", 
                        "Checking Network Connection...\n.\n.\n.\nConnected!\n.\n",
                        "Checking Network Connection...\n.\n.\n.\nConnected!\n.\nSince you have use this it will\nbe close automatically.", 
                        "Checking Network Connection...\n.\n.\n.\nConnected!\n.\nSince you have use this it will\nbe close automatically.\n.\n",
                        "Checking Network Connection...\n.\n.\n.\nConnected!\n.\nSince you have use this it will\nbe close automatically.\n.\n.\n",
                        "Checking Network Connection...\n.\n.\n.\nConnected!\n.\nSince you have use this it will\nbe close automatically.\n.\n.\n.\n",
                        "Checking Network Connection...\n.\n.\n.\nConnected!\n.\nSince you have use this it will\nbe close automatically.\n.\n.\n.\nGood Bye!"];
                        
                        for string in &message_vec_ext {
                            message_vec.push(string);
                            messages_para = Paragraph::new(message_vec[message_vec.len()-1].to_string())
                                .block(Block::default().title("Messages").borders(Borders::ALL))
                                .style(Style::default().fg(Color::White))
                                .alignment(Alignment::Left);
                            terminal.as_mut().expect("REASON").draw(|f| {
                                f.render_widget(time_para.clone(), mid_chunks[3]);
                                f.render_widget(messages_para, main_chunks[2]);
                            }).unwrap();

                            delay_for(Duration::from_millis(1000)).await;
                        }
                    }
                    else {
                        message_vec.push("Checking Network Connection...\n.\n.\n.\nDisconnected!");
                        messages_para = Paragraph::new(message_vec[message_vec.len()-1].to_string())
                            .block(Block::default().title("Messages").borders(Borders::ALL))
                            .style(Style::default().fg(Color::White))
                            .alignment(Alignment::Left);
                        terminal.as_mut().expect("REASON").draw(|f| {
                            f.render_widget(time_para.clone(), mid_chunks[3]);
                            f.render_widget(messages_para, main_chunks[2]);
                        }).unwrap();
                    }

                    delay_for(Duration::from_millis(2000)).await;
                    app.ending = true;
                },
                InputMode::InsertingName => {
                    f.set_cursor(
                        left_chunks[0].x + app.cursor_position_name as u16 + 1,
                        left_chunks[0].y + 1,
                    )
                },
                InputMode::InsertingEmail => {
                    f.set_cursor(
                        left_chunks[1].x + app.cursor_position_email as u16 + 1,
                        left_chunks[1].y + 1,
                    )
                },
                InputMode::InsertingWebsite => {
                    f.set_cursor(
                        left_chunks[2].x + app.cursor_position_website as u16 + 1,
                        left_chunks[2].y + 1,
                    )
                },
                InputMode::InsertingHour => {},
                InputMode::InsertingMinute => {},
            }
        }
        2 => f.render_widget(render_about(), chunks1[1]),
        _ => {}
    }
}

fn render_home<'a>() -> Paragraph<'a> {
    let text = vec![
        Line::from("\n"),
        Line::styled("Welcome", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Line::from("\n"),
        Line::styled("To", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Line::from("\n"),
        Line::styled("NotiCheckDown", Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD | Modifier::ITALIC)),
        Line::from("\n"),
        Line::styled("Press 'c' to access main program, About this app press 'a'", Style::default().fg(Color::White)),
    ];
    Paragraph::new(text)
        .block(Block::default().title("Home").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
}

fn render_about<'a>() -> Paragraph<'a> {
    let text = vec![
        Line::from(""),
        Line::from("     \"NotiCheckDown (a tui-app) is your go-to solution for website monitoring. Choose the sites you care about, and we'll alert you if they go down. For live sites, it's also provide detailed"),
        Line::from(""),
        Line::from(" response time statistics, keeping you informed about your online assets. With a user-friendly interface and proactive monitoring, NotiCheckDown empowers you to take control of your online"),
        Line::from(""),
        Line::from(" presence and ensure uninterrupted user experiences.\"")
    
    ];
    Paragraph::new(text)
        .block(Block::default().title("About").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
}

fn is_internet_connected() -> bool {
    let client = match HttpClient::new() {
        Ok(client) => client,
        _ => {
            return false;
        }
    };

    let response = match client.get("https://www.google.com") {
        Ok(response) => response,
        _ => {
            return false;
        }
    };

    response.status() == StatusCode::OK || response.status().is_redirection()
}