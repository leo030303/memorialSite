use crate::models::Anecdote;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub fn send_email(anecdote: Anecdote) {
    let source_email = env::var("SOURCE_EMAIL").expect("SOURCE_EMAIL must be set");
    let dest_email = env::var("DEST_EMAIL").expect("DEST_EMAIL must be set");
    let source_password = env::var("SOURCE_PASSWORD").expect("SOURCE_PASSWORD must be set");
    let email = Message::builder()
        .from(format!("Memorial Site <{}>", source_email).parse().unwrap())
        .to(format!("Leo Ring <{}>", dest_email).parse().unwrap())
        .subject("New Anecdote")
        .header(ContentType::TEXT_PLAIN)
        .body(format!(
            "Author: {}\nContent: {}\n",
            anecdote.author, anecdote.content
        ))
        .unwrap();

    let creds = Credentials::new(source_email.to_owned(), source_password.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}
