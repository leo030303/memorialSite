use diesel::prelude::*;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::videos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Video {
    pub video_id: i32,
    pub url: String,
    pub title: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::pictures)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Picture {
    pub picture_id: i32,
    pub filename: String,
    pub caption: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::condolences)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Condolence {
    pub condolence_id: i32,
    pub author: String,
    pub content: String,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, FromForm, Insertable)]
#[diesel(table_name = crate::schema::anecdotes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Anecdote {
    pub author: String,
    pub content: String,
    pub published: bool,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, FromForm)]
#[diesel(table_name = crate::schema::anecdotes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AnecdoteWithID {
    pub anecdote_id: i32,
    pub author: String,
    pub content: String,
    pub published: bool,
}
