use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use tokio::time::sleep as delay_for;
use ratatui::widgets::*;
use std::time::Duration;

use ratatui::prelude::*;
use std::io;

mod helpers;
use helpers::*;

mod r#struct;
use r#struct::*;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if app.ending_connected == true { 
            make_env(app.name, app.email);
            let res = check_res(app.website, app.hr, app.min);
            
            if res == true {
                let _ = send_email(true);
                return Ok(()) 
            }
            else {
                let _ = send_email(false);
                return Ok(()) 
            }
        }
        else if app.ending_disconnected == true { return Ok(()) }

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
pub async fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
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
                Constraint::Percentage(40),
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
                        app.ending_connected = true;
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
                        app.ending_disconnected = true;
                    }

                    delay_for(Duration::from_millis(2000)).await;
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

use std::error::Error;

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

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
