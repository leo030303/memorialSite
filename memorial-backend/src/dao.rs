use crate::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_condolences() -> Vec<Condolence> {
    use crate::schema::condolences::dsl::*;

    let connection = &mut establish_connection();
    condolences
        .select(Condolence::as_select())
        .load(connection)
        .expect("Error loading condolences")
}

pub fn get_published_anecdotes() -> Vec<Anecdote> {
    use crate::schema::anecdotes::dsl::*;

    let connection = &mut establish_connection();
    anecdotes
        .filter(published.eq(true))
        .select(Anecdote::as_select())
        .load(connection)
        .expect("Error loading anecdotes")
}
pub fn get_pictures() -> Vec<Picture> {
    use crate::schema::pictures::dsl::*;

    let connection = &mut establish_connection();
    pictures
        .select(Picture::as_select())
        .load(connection)
        .expect("Error loading pictures")
}

pub fn get_videos() -> Vec<Video> {
    use crate::schema::videos::dsl::*;

    let connection = &mut establish_connection();
    videos
        .select(Video::as_select())
        .load(connection)
        .expect("Error loading videos")
}

pub fn insert_anecdote(new_anecdote: Anecdote) -> Anecdote {
    use crate::schema::anecdotes;

    let connection = &mut establish_connection();
    diesel::insert_into(anecdotes::table)
        .values(&new_anecdote)
        .returning(Anecdote::as_returning())
        .get_result(connection)
        .expect("Error saving new anecdote")
}
