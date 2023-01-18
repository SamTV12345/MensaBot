use std::thread;
use chrono::{Datelike, DateTime, NaiveDate, TimeZone, Utc};
use regex::Regex;
use teloxide::Bot;
use teloxide::prelude::{ChatId, Requester};
use teloxide::types::Message;
use crate::postgres_client::{delete_subscriber, insert_subscriber};
use crate::{create_message_with_heading, send_meals_to_one_subscriber, send_message};
use crate::database::extract_meals;

pub async fn send_telegram_reply(bot:Bot, chat_id: ChatId, msg: Message) {
    let command: &str = msg.text().unwrap().split([' ']).collect::<Vec<&str>>()[0];
    match command {
        "/subscribe" => {
            thread::spawn(move || {
                insert_subscriber(msg.chat.id);
            }).join().expect("Error deleting person");

            bot.send_message(chat_id,"You have been subscribed to the daily meal")
                .await.expect("Error sending message");
        }
        "/help"=>{
            bot.send_message(chat_id,"/subscribe - Subscribe to the daily meal\
            \n/unsubscribe - Unsubscribe from the daily meal.\
            \n/getMeal tomorrow - Get the meal for tomorrow.\
            \n/getMeal dd.mm.yyyy - Get the meal for the given date if it exists.\
            \n/getMeal - Get today's meal.").await.expect("Error sending subscribe message");
        }
        "/getMeal"=>{
            thread::spawn(move || {
                check_if_string_contains_date(msg.text().unwrap(), chat_id);
            });
        }
        "/unsubscribe" => {
            thread::spawn(move || {
                delete_subscriber(msg.chat.id);
            }).join().expect("TODO: panic message");

            bot.send_message(msg.chat.id, "You have been unsubscribed to the daily meal")
                .await.expect("Unscribe failed");
        }
        _ => {
            bot.send_message(msg.chat.id, "Unknown command").await.expect("Error sending \
            unknown command");
        }
    }
}

fn check_if_string_contains_date(str_to_check:&str, chat_id: ChatId){
    if check_if_contains_tommorow(str_to_check) {
        send_meal_for_tomorrow(&chat_id);
        return;
    }
    let date_regex = Regex::new(r"\d{2}.\d{2}.\d{4}").unwrap();
    let date = date_regex.find(str_to_check);
    match date {
        Some(date) => {
            let date = date.as_str();
            println!("{}", date);
            let date = NaiveDate::parse_from_str(date, "%d.%m.%Y").unwrap();
            let heading = format!("Am {} gibt es folgende Gerichte: \n", date.format("%d.%m.%Y"));
            let dt: DateTime<Utc> = Utc.with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0).unwrap();
            let message_to_send: String = create_message_with_heading(extract_meals(dt), &heading);
            send_message(&message_to_send, &chat_id.to_string());
        }
        None => {
            thread::spawn(move || {
                send_meals_to_one_subscriber(&chat_id.0);
            }).join().expect("Error sending meal to subscriber");
        }
    }
}

fn send_meal_for_tomorrow(chat_id: &ChatId) {
    let current_date = Utc::now();
    let dt: DateTime<Utc> = Utc.with_ymd_and_hms(current_date.year(), current_date.month(),
                                                 current_date.day() + 1, 0, 0, 0).unwrap();
    let message_to_send: String = create_message_with_heading(extract_meals(dt), "Morgen gibt es folgende Gerichte: \n");
    send_message(&message_to_send, &chat_id.to_string());
}

fn check_if_contains_tommorow(msg: &str)->bool{
    return msg.contains("tomorrow")||msg.contains("morgen");
}