use postgres::{Client, NoTls};
use teloxide::prelude::ChatId;

pub fn get_client()->Client{
    let client = Client::connect("postgresql://postgres:changeme@192.168.2.32/mensatest",
                                     NoTls).expect("Connection failed");
    return client;
}

pub fn insert_subscriber(id: ChatId) {
    let mut client = get_client();
    client.execute("INSERT INTO telegram_subscribers (id) VALUES ($1)", &[&id.0])
        .expect("Inserting failed. Already subscribed");
    client.close().expect("Closing connection failed");
}