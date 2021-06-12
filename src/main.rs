use dotenv;
use plaid;
use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    dotenv::dotenv().unwrap();

    let client = plaid::client::Client::new(
        dotenv::var("PLAID_CLIENT_ID").unwrap(),
        dotenv::var("PLAID_SECRET").unwrap(),
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

    client
        .update_item_webhook(&access_token, "https://en7tryxhr10q4y5.m.pipedream.net")
        .await
        .unwrap();
    client
        .fire_webhook(access_token.as_str(), "DEFAULT_UPDATE")
        .await
        .unwrap();
}

// TODO: Webhook server.
// TODO: respond to "DEFAULT_UPDATE" transaction webhook calls.
// TODO: Look at new transactions, make sure that they're not our own (somehow)
// TODO: Check if a transfer will overdraft
// TODO: Send money
