use std::collections::LinkedList;
use chrono::{DateTime, Utc};
use postgres::{Client, NoTls, Row};
use crate::models::{HTWMainModel, MealModel};
use crate::postgres_client::get_client;


static QUERY: &str = "CREATE TABLE IF NOT EXISTS public.meal(
    id integer NOT NULL DEFAULT nextval('meal_id_seq'),
    calendar timestamp with time zone,
    counterid character varying(255),
    countername character varying(255),
    name character varying(255),
    studentprice character varying(255),
    CONSTRAINT meal_pkey PRIMARY KEY (id)
    )";


pub fn prepare_database(client: Client) {
    log::info!("Preparing database");

    init_meal_database(client);
    init_telegram_bot_subscribers();
}

fn init_telegram_bot_subscribers(){
    log::info!("Preparing telegram bot subscribers");
    let mut client = get_client();

    client.execute("CREATE TABLE IF NOT EXISTS telegram_subscribers (
                    id BIGINT PRIMARY KEY);", &[]).expect("Creating table failed");
}


fn init_meal_database(mut client: Client){
    let table_exists: bool = client.query("SELECT EXISTS (SELECT * FROM pg_class WHERE relname \
    ='meal');", &[]).unwrap_or_else(|e| {
        panic!("Error: {}", e);
    }).iter().next().unwrap().get(0);



    if !table_exists {
        log::info!("Preparing database");
        client.execute("CREATE SEQUENCE IF NOT EXISTS meal_id_seq", &[]).expect("Creating sequence \
        failed");
        client.execute(QUERY, &[]).expect("Query failed");
        client.execute("ALTER SEQUENCE meal_id_seq OWNED BY public.meal.id;", &[]).expect
        ("Creating sequence failed");
    }
    client.close().expect("Closing connection failed");
}

fn delete_if_meal_exists(client: &mut Client, calendar:DateTime<Utc>) {
    client.execute("DELETE FROM meal WHERE calendar = $1", &[&calendar]).expect("Deleting failed");
}

fn row_to_meal(x: &Row) ->MealModel{
    return MealModel{
        id: x.get(0),
        calendar: x.get(1),
        counterid: x.get(2),
        countername: x.get(3),
        name: x.get(4),
        studentprice: x.get(5),
    };
}

pub fn insert_htwmeal(meal: HTWMainModel){
    let mut client = get_client();

    // Delete all old entries
    for (_, item) in meal.days.iter().enumerate(){
        delete_if_meal_exists(&mut client, item.date);
    }

    for (_, item) in meal.days.iter().enumerate(){
        // Delete all old entries
        for (_, counter) in item.counters.iter().enumerate(){
            for (_, meal) in counter.meals.iter().enumerate(){

                let student_price: &str;
                match &meal.prices{
                    Some(p)=> {
                        log::info!("{}", p.s);
                        student_price = &p.s;
                    },
                    None=>{
                        student_price = "0";
                    }
                };

                client.execute("INSERT INTO meal (calendar, counterid, countername, name, \
                studentprice) VALUES ($1, $2, $3, $4, $5)", &[&item.date, &counter.id,
                        &counter.display_name, &meal.name, &student_price]).expect("Insert failed");
                }
            }
    }
}


pub fn extract_meals(date: DateTime<Utc>) -> LinkedList<MealModel>{
    let mut client = get_client();
    return client.query("SELECT * FROM meal WHERE calendar = $1", &[&date]).expect("Deleting \
    failed")
        .iter().map(|x| row_to_meal(x)).collect::<LinkedList<MealModel>>();

}