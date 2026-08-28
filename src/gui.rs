use iced::{Element, widget::{self, text_editor}, Task, Length};
use sqlx::SqlitePool;

use crate::database::{init, run_query};

pub struct State {
    pool: Option<SqlitePool>,
    mode: bool,
    query: text_editor::Content,
    result: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            pool: None,
            mode: false,
            query: text_editor::Content::new(),
            result: String::new(),
        }
    }
}

#[derive(Clone)]
pub enum Message {
    ChangeMode(bool),
    NewQuery(text_editor::Action),
    Run,

    InitComplete(Result<SqlitePool, String>),
    QueryResult(Result<String, String>),
}

pub fn view(state: &State) -> Element<'_, Message> {
    let mode: Element<'_, Message> = if state.mode {
        widget::button("Fetch").on_press(Message::ChangeMode(false)).into()
    } else {
        widget::button("Execute").on_press(Message::ChangeMode(true)).into()
    };
    widget::column![
        widget::row![
            mode,
            widget::space().width(Length::Fill),
            widget::button("Run >").on_press(Message::Run),
        ],
        widget::text_editor(&state.query)
            .on_action(Message::NewQuery)
            .height(Length::FillPortion(1)),
        widget::container(
            widget::scrollable(
                widget::text(&state.result),
            )
            .width(Length::Fill),
        )
        .height(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(10)
    .into()
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::ChangeMode(new_mode) => {
            state.mode = new_mode; 
            Task::none()  
        }
        Message::NewQuery(action) => {
            state.query.perform(action);
            Task::none()
        }  
        Message::Run => {
            Task::perform(
                init(),
                |result| Message::InitComplete(
                    result.map_err(|err| err.to_string())
                ),
            )
        }
        Message::InitComplete(result) => {
            match result {
                Ok(pool) => state.pool = Some(pool),
                Err(error) => state.result = format!("(Err): {}", error),
            }
            let pool = state.pool.clone().unwrap();
            let query = state.query.text();
            Task::perform(
                run_query(pool, query, state.mode),
                |result| Message::QueryResult(
                    result.map_err(|err| err.to_string())
                ),
            )
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