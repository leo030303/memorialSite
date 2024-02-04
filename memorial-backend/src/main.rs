#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, NamedFile};
use std::path::{Path, PathBuf};

use rocket::form::Form;
use rocket::response::Redirect;

use memorial_backend::dao::*;
use memorial_backend::mail::send_email;
use memorial_backend::models::Anecdote;

// MAIN

#[get("/")]
async fn index() -> Redirect {
    Redirect::to("/main")
}

#[get("/")]
async fn main_route() -> Option<NamedFile> {
    NamedFile::open(Path::new("frontendBuild/index.html"))
        .await
        .ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("frontendBuild/").join(file))
        .await
        .ok()
}

// CONDOLENCE_LIST

#[get("/")]
async fn condolences() -> String {
    let condolences = get_condolences();
    serde_json::to_string(&condolences).unwrap()
}

// ANECDOTE_LIST

#[get("/")]
async fn anecdotes() -> String {
    let anecdotes = get_published_anecdotes();
    serde_json::to_string(&anecdotes).unwrap()
}

// VIDEO_LIST

#[get("/")]
async fn videos() -> String {
    let videos = get_videos();
    serde_json::to_string(&videos).unwrap()
}

// PICTURE_LIST

#[get("/")]
async fn pictures() -> String {
    let pictures = get_pictures();
    serde_json::to_string(&pictures).unwrap()
}

// NEW_ANECDOTE

#[post("/", data = "<new_anecdote>")]
async fn new_anecdote(new_anecdote: Form<Anecdote>) -> Redirect {
    let new_anecdote = new_anecdote.into_inner();
    let new_anecdote = insert_anecdote(new_anecdote);
    send_email(new_anecdote);
    Redirect::to("/main")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/main", routes![main_route, files])
        .mount("/condolenceList", routes![condolences])
        .mount("/anecdoteList", routes![anecdotes])
        .mount("/videoList", routes![videos])
        .mount("/pictureList", routes![pictures])
        .mount("/imageFiles", FileServer::from("images/"))
        .mount("/newAnecdote", routes![new_anecdote])
}
