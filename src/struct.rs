use ratatui::widgets::*;

pub enum InputMode {
    Normal,
    InsertingName,
    InsertingEmail,
    InsertingWebsite,
    InsertingHour,
    InsertingMinute,
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
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

    pub fn previous(&mut self) {
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
    pub ending_connected: bool,
    pub ending_disconnected: bool,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
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
            ending_connected: false,
            ending_disconnected: false
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