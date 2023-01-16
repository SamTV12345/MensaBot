use std::env::var;
use postgres::{Client, Error, NoTls};
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
    let res: Result<u64, Error> =client.execute("INSERT INTO telegram_subscribers (id) VALUES \
    ($1)", &[&id.0]);
    match res {
        Ok(_) => {
            log::info!("Subscriber inserted");
        }
        Err(_) => {
            log::error!("Subscriber already inserted. Skipping...");
        }
    }
    client.close().expect("Closing connection failed");
}

pub fn delete_subscriber(id: ChatId) {
    let mut client = get_client();
    let res: Result<u64, Error> =client.execute("DELETE FROM telegram_subscribers WHERE id = $1", &[&id.0]);
    match res {
        Ok(_) => {
            log::info!("Subscriber deleted");
        }
        Err(_) => {
            log::error!("Subscriber not found. Skipping...");
        }
    }
}