mod models;
mod database;
mod postgres_client;
mod logging;
mod message_sender;


use clokwerk::{Job, Scheduler, TimeUnits};
use std::{thread};
use std::collections::LinkedList;
use std::time::Duration;
use models::HTWMainModel;
use std::env::{var};
use chrono::{Datelike, DateTime, TimeZone, Utc};
use teloxide::Bot;
use teloxide::prelude::{Message, Request, Requester};
use crate::database::{extract_meals, insert_htwmeal, prepare_database};
use crate::postgres_client::{get_client};
use teloxide::types::Recipient;
use crate::logging::init_logging;
use crate::message_sender::send_telegram_reply;
use crate::models::MealModel;

fn main() {

    // env variables
    /*let postgres_user = var("POSTGRES_USER").unwrap();
    let postgres_password = var("POSTGRES_PASSWORD").unwrap();
    let postgres_host = var("POSTGRES_HOST").unwrap();
    let postgres_port = var("POSTGRES_PORT").unwrap();
    let postgres_db = var("POSTGRES_DB").unwrap();*/

    let api_url: String = var("API_URL").unwrap();
    let init_meals = var("INIT_MEALS");

    init_logging();
    match init_meals {
        Ok(_) => {
            log::info!("Initing meals");
            query_and_insert_meals(&api_url);
        }
        Err(_) => {
           log::error!("Meals will not be initialized");
        }
    }

    let client = get_client();
    thread::spawn(||{
        log::info!("Starting telegram bot");
        init_telegram_bot();
    });
    prepare_database(client);
    let mut scheduler = Scheduler::new();

    scheduler.every(1.week()).plus(10.second()).run(move ||{
        query_and_insert_meals(&api_url);
    });

    scheduler.every(1.day()).at("06:00:00").run(move ||{
        send_daily_meal();
    });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(10000));
    }
}

fn query_and_insert_meals(api_url: &String) {
    let result: HTWMainModel = do_rest_call(&api_url);
    insert_htwmeal(result);
}

#[tokio::main]
async fn do_rest_call(x: &str) -> HTWMainModel {
    log::info!("Calling API");
    let response = reqwest::get(x).await.unwrap();

    let users:HTWMainModel = response.json().await.unwrap();

    return users;
}


#[tokio::main]
async fn init_telegram_bot() {
    log::info!("Starting telegram bot");
    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        log::info!("Received a message from {}: {}", msg.chat.id, msg.text().unwrap());

        send_telegram_reply(bot, msg.chat.id, msg).await;
        Ok(())
    }).await;
}

fn send_daily_meal(){
    log::info!("Sending daily meals.");
    let mut client = get_client();
    if let Ok(row) = client.query("SELECT id FROM telegram_subscribers;",&[]) {
        let id = row.get(0);
        match id {
            Some(id) => {
                let chat_id:i64 = id.get(0);
                send_meals_to_one_subscriber(&chat_id);
            }
            None => {
                log::info!("No id found");
            }
        }
    }
}

fn send_meals_to_one_subscriber(chat_id: &i64) {
    let current_date = Utc::now();
    let dt: DateTime<Utc> = Utc.with_ymd_and_hms(current_date.year(), current_date.month
    (), current_date.day(), 0, 0, 0).unwrap();
    let message_to_send: String = create_message(extract_meals(dt));
    send_message(&message_to_send, &chat_id.to_string());
}

#[tokio::main]
async fn send_message(message: &str, chat_id:&str){
    let bot = Bot::from_env();
    bot.send_message(Recipient::from(chat_id.to_string()), message).send()
        .await.expect("TODO: panic message");
}


fn create_message(meals_of_today: LinkedList<MealModel>) ->String{
    let mut message = String::new();
    message.push_str("Heute gibt es folgende Gerichte: \n");
    for i in meals_of_today {
        message.push_str(&format!("- {} für {}€\n", &i.name, &i.studentprice));
    }
    return message;
}

fn create_message_with_heading(meals_of_today: LinkedList<MealModel>, heading:&str) ->String{
    let mut message = String::new();
    message.push_str(heading);
    for i in meals_of_today {
        message.push_str(&format!("- {} für {}€\n", &i.name, &i.studentprice));
    }
    return message;
}