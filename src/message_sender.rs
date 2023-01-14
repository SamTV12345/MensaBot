use std::thread;
use teloxide::Bot;
use teloxide::prelude::{ChatId, Requester};
use teloxide::types::Message;
use crate::postgres_client::{delete_subscriber, insert_subscriber};
use crate::send_meals_to_one_subscriber;

pub async fn send_telegram_reply(bot:Bot, chat_id: ChatId, msg: Message) {
    match msg.text().unwrap() {
        "/subscribe" => {
            thread::spawn(move || {
                insert_subscriber(msg.chat.id);
            }).join().expect("Error deleting person");

            bot.send_message(chat_id,"You have been subscribed to the daily meal")
                .await.expect("Error sending message");
        }
        "/help"=>{
            bot.send_message(chat_id,"/subscribe - Subscribe to the daily meal\n/unsubscribe - \
            Unsubscribe from the daily meal").await.expect("Error sending subscribe message");
        }
        "/getMeal"=>{
            thread::spawn(move || {
                send_meals_to_one_subscriber(&chat_id.0);
            }).join().expect("Error sending meal to subscriber");
        }
        "/unsubscribe" => {
            thread::spawn(move || {
                delete_subscriber(msg.chat.id);
            }).join().expect("TODO: panic message");

            bot.send_message(msg.chat.id, "You have been unsubscribed to the daily meal")
                .await.expect("Unscribe failed");
        }
        _ => {
            bot.send_message(msg.chat.id, ("Unknown command")).await.expect("Error sending \
            unknown command");
        }
    }
}