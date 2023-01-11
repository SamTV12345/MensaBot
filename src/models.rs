use chrono::{DateTime, Utc};

use serde::Deserialize;
use std::collections::{HashSet, LinkedList};
use std::collections::hash_map::RandomState;

#[derive(Deserialize, Debug)]
pub struct MyHashSet<T> {
    base: Vec<T>,
}

#[derive(Deserialize, Debug)]
struct OpeningHours {
    start: DateTime<Utc>,
    end: DateTime<Utc>
}

#[derive(Deserialize, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8
}

#[derive(Deserialize, Debug)]
struct PriceList{
    s: String,
    m: String,
    g: String
}

#[derive(Deserialize, Debug)]
struct Component {
    name:String,
    notices: LinkedList<String>
}

#[derive(Deserialize, Debug)]
struct Meal {
    name: String,
    notices: LinkedList<String>,
    components: LinkedList<Component>,
    prices: Option<PriceList>,
    category: Option<String>
}

#[derive(Deserialize, Debug)]
struct Feedback{
    start : DateTime<Utc>,
    end : DateTime<Utc>
}

#[derive(Deserialize, Debug)]
struct Counter{
    id: String,
    displayName: String,
    description: String,
    openingHours: OpeningHours,
    color: Color,
    feedback: Feedback,
    meals: Vec<Meal>
}


#[derive(Deserialize, Debug)]
pub struct HTWMainModel {
    days: LinkedList<days>
}

#[derive(Deserialize, Debug)]
struct days {
    date: DateTime<Utc>,
    isPast: bool,
    counters: LinkedList<Counter>
}