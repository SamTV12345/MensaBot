mod models;


use clokwerk::{Job, Scheduler, TimeUnits};
use std::thread;
use std::time::Duration;
use serde::Deserialize;
use reqwest::{Error, Response};
use models::HTWMainModel;
use tokio::runtime::Runtime;
use std::env::{var_os};
use std::ffi::OsString;

fn main() {
    // env variables
    const API_URL: OsString = var_os("API_URL").unwrap_or("unset".into());



    let mut scheduler = Scheduler::new();

    println!("Hello, world!");
    scheduler.every(30.second()).run(move ||{
        println!("Hello, world2!");
        do_rest_call(API_URL.to_str().unwrap());
    });



    // Manually run the scheduler in an event loop
    loop {
        scheduler.run_pending();
        println!("Waiting for next job");
        thread::sleep(Duration::from_millis(1000));
    }


}

#[tokio::main]
async fn do_rest_call(x: &str) {
    println!("Calling api");
    let response = reqwest::get(x).await.unwrap();

    let users:HTWMainModel = response.json().await.unwrap();
    println!("{:?}", users);
}