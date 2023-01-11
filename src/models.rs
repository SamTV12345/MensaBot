use chrono::{DateTime, Utc};

use serde::Deserialize;
use std::collections::{LinkedList};

#[derive(Deserialize, Debug)]
pub struct MyHashSet<T> {
    pub base: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct OpeningHours {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>
}

#[derive(Deserialize, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Deserialize, Debug)]
pub struct PriceList{
    pub s: String,
    pub m: String,
    pub g: String
}

#[derive(Deserialize, Debug)]
pub struct Component {
    pub name:String,
    pub notices: LinkedList<String>
}

#[derive(Deserialize, Debug)]
pub struct Meal {
    pub name: String,
    pub notices: LinkedList<String>,
    pub components: LinkedList<Component>,
    pub prices: Option<PriceList>,
    pub category: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Feedback{
    pub start : DateTime<Utc>,
    pub end : DateTime<Utc>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Counter{
    pub id: String,
    pub display_name: String,
    pub description: String,
    pub opening_hours: OpeningHours,
    pub color: Color,
    pub feedback: Feedback,
    pub meals: Vec<Meal>
}


#[derive(Deserialize, Debug)]
pub struct HTWMainModel {
    pub days: LinkedList<Days>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Days {
    pub date: DateTime<Utc>,
    pub is_past: bool,
    pub counters: LinkedList<Counter>
}


/**
    Database model
 */

pub struct MealModel {
    pub id: i32,
    pub calendar: DateTime<Utc>,
    pub counterid: String,
    pub countername: String,
    pub name: String,
    pub studentprice: String
}