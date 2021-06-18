#[macro_use]
extern crate rocket;

use dotenv;
use plaid;
use plaid::client::Client;
use reqwest;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
use rocket::{Build, Rocket};
use tokio;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct PlaidUpdate {
    error: Option<String>,
    item_id: String,
    new_transactions: i32,
    webhook_code: String,
    webhook_type: String,
}

#[post("/", format = "json", data = "<update>")]
async fn change_jar_hook(
    update: Json<PlaidUpdate>,
    client: &State<Client>,
    access_token: &State<AccessToken>,
) -> Value {
    if update.webhook_type != "TRANSACTION" {
        return json!({"status": "ignored"});
    }
    if update.webhook_code != "DEFAULT_UPDATE" {
        return json!({"status": "ignored"});
    }

    if update.new_transactions <= 0 {
        return json!({"status": "ignored"});
    }

    json!({})
}

lazy_static::lazy_static! {
    static ref PLAID_CLIENT_ID: String = {
        dotenv::dotenv().unwrap();
        dotenv::var("PLAID_CLIENT_ID").unwrap()
    };

    static ref PLAID_SECRET: String = {
        dotenv::dotenv().unwrap();
        dotenv::var("PLAID_SECRET").unwrap()
    };
}

struct AccessToken(String);

#[launch]
async fn blastoff() -> Rocket<Build> {
    let client = plaid::client::Client::new(
        PLAID_CLIENT_ID.to_string(),
        PLAID_SECRET.to_string(),
        plaid::client::Environment::Sandbox,
    );

    let public_token = client
        .create_sandbox_public_token("ins_1", &["transactions"])
        .await
        .unwrap();
    let access_token = client
        .exchange_public_token(public_token.public_token.as_str())
        .await
        .unwrap();
    let access_token = access_token.access_token;

    rocket::build()
        .mount("/", routes![change_jar_hook])
        .manage(client)
        .manage(AccessToken(access_token))
}

// async fn setup_plaid_old(client: &plaid::client::Client, access_token: &str) {
//     client
//         .update_item_webhook(access_token, "http://50.4.234.24:8000")
//         .await
//         .unwrap();
//     let response = client
//         .fire_webhook(access_token, "DEFAULT_UPDATE")
//         .await
//         .unwrap();
//
//     println!("{:#?}", response);
// }

// TODO: Look at new transactions, make sure that they're not our own (somehow)
// TODO: Check if a transfer will overdraft
// TODO: Send money
