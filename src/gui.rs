use iced::{Element, widget::{self, text_editor}, Task, Length};
use sqlx::SqlitePool;

use crate::{database::{init, run_query}, style::output};

pub struct State {
    theme: bool,
    db_address: String,
    pool: Option<SqlitePool>,
    mode: bool,
    query: text_editor::Content,
    result: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            theme: false,
            db_address: "".to_string(),
            pool: None,
            mode: false,
            query: text_editor::Content::new(),
            result: String::new(),
        }
    }
}

impl State {
    pub fn is_dark(&self) -> bool {
        self.theme
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
    QueryResult(Result<String, String>),
}

pub fn view(state: &State) -> Element<'_, Message> {
    let theme: Element<'_, Message> = if state.theme {
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
                widget::text(&state.result),
            )
            .width(Length::Fill),
        )
        .height(Length::FillPortion(1))
        .padding(5)
        .style(|theme| output(theme, state.theme)),
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
            state.theme = other_theme;
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
                    state.result = format!("(Err): {}", error);
                    Task::none()
                }
            }
        }
        Message::QueryResult(result) => {
            match result {
                Ok(outcome) => state.result = outcome,
                Err(error) => state.result = format!("(Err): {}", error),
            }
            Task::none()
        }
    }
}