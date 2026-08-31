use iced::{Element, widget::{self, text_editor}, Task, Length};
use sqlx::SqlitePool;
use comfy_table::Table;

use crate::{
    database::{init, run_query}, 
    setting::{Setting, load_setting, update_setting}, 
    style::output
};

pub struct State {
    setting: Setting,
    db_address: String,
    pool: Option<SqlitePool>,
    mode: bool,
    query: text_editor::Content,
    table_result: Table,
    str_result: String,
}

impl Default for State {
    fn default() -> Self {
        let default_setting = Setting::default();
        let setting = match load_setting(&default_setting) {
            Ok(config) => config,
            Err(_) => default_setting,
        };

        Self {
            setting: setting,
            db_address: "".to_string(),
            pool: None,
            mode: false,
            query: text_editor::Content::new(),
            table_result: Table::new(),
            str_result: String::new(),
        }
    }
}

impl State {
    pub fn is_dark(&self) -> bool {
        self.setting.theme
    }
}

#[derive(Clone)]
pub enum Message {
    ChangeTheme(bool),
    DatabaseAddress(String),
    ChangeMode(bool),
    NewQuery(text_editor::Action),
    Run,

    InitComplete(Result<SqlitePool, String>),
    QueryResult(Result<(Table, String), String>),
}

pub fn view(state: &State) -> Element<'_, Message> {
    let result = if state.mode {
        state.table_result.to_string()
    } else {
        state.str_result.clone()
    };

    let theme: Element<'_, Message> = if state.setting.theme {
        widget::button("Light").on_press(Message::ChangeTheme(false)).into()
    } else {
        widget::button("Dark").on_press(Message::ChangeTheme(true)).into()
    };
    let db_mode: Element<'_, Message> = if state.mode {
        widget::button("Fetch").on_press(Message::ChangeMode(false)).into()
    } else {
        widget::button("Execute").on_press(Message::ChangeMode(true)).into()
    };
    widget::column![
        widget::row![
            db_mode,
            widget::text_input("Database Address", &state.db_address)
                .width(Length::Fill)
                .on_input(|address| Message::DatabaseAddress(address)),
            widget::button("Run >").on_press(Message::Run),
        ]
        .spacing(10),
        widget::text_editor(&state.query)
            .on_action(Message::NewQuery)
            .height(Length::FillPortion(1)),
        widget::container(
            widget::scrollable(
                widget::text(result)
                    .font(iced::Font::MONOSPACE),
            )
            .width(Length::Fill),
        )
        .height(Length::FillPortion(1))
        .padding(5)
        .style(|theme| output(theme, state.setting.theme)),
        widget::row![
            theme,
            widget::space().width(Length::Fill),
        ]
        .spacing(10),
    ]
    .spacing(10)
    .padding(10)
    .into()
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::ChangeTheme(other_theme) => {
            state.setting.theme = other_theme;
            if let Err(error) = update_setting(&state.setting) {
                state.str_result = error.to_string();
            }
            Task::none()
        }
        Message::DatabaseAddress(address) => {
            state.db_address = address;
            Task::none()
        }
        Message::ChangeMode(new_mode) => {
            state.mode = new_mode; 
            Task::none()  
        }
        Message::NewQuery(action) => {
            state.query.perform(action);
            Task::none()
        }  
        Message::Run => {
            let address = state.db_address.clone();
            Task::perform(
                init(address),
                |result| Message::InitComplete(
                    result.map_err(|err| err.to_string())
                ),
            )
        }
        Message::InitComplete(result) => {
            match result {
                Ok(pool) => {
                    state.pool = Some(pool.clone());

                    let query = state.query.text();
                    Task::perform(
                        run_query(pool, query, state.mode), 
                        |result| Message::QueryResult(
                            result.map_err(|err| err.to_string())
                        ),
                    )
                }
                Err(error) => {
                    state.str_result = format!("(Err): {}", error);
                    Task::none()
                }
            }
        }
        Message::QueryResult(result) => {
            match result {
                Ok((table, outcome)) => {
                    state.table_result = table;
                    state.str_result = outcome;
                }
                Err(error) => state.str_result = format!("(Err): {}", error),
            }
            Task::none()
        }
    }
}