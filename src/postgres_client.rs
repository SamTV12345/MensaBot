use std::env::var;
use std::fmt::format;
use postgres::{Client, NoTls};
use teloxide::prelude::ChatId;

pub fn get_client()->Client{

    let postgres_user = var("POSTGRES_USER").unwrap();
    let postgres_password = var("POSTGRES_PASSWORD").unwrap();
    let postgres_host = var("POSTGRES_HOST").unwrap();
    let postgres_port = var("POSTGRES_PORT").unwrap();
    let postgres_db = var("POSTGRES_DB").unwrap();

    let client = Client::connect(&format!("postgresql://{}:{}@{}:{}/{}",
                                         postgres_user, postgres_password,postgres_host,
                                          postgres_port, postgres_db),
                                     NoTls).expect("Connection failed");
    return client;
}

pub fn insert_subscriber(id: ChatId) {
    let mut client = get_client();
    client.execute("INSERT INTO telegram_subscribers (id) VALUES ($1)", &[&id.0])
        .expect("Inserting failed. Already subscribed");
    client.close().expect("Closing connection failed");
}