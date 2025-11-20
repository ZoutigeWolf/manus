use dotenv::dotenv;
use futures::stream::{FuturesUnordered, StreamExt};

use axum::{routing::get, Extension, Router};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::{
    net::TcpListener,
    sync::RwLock,
    time::{interval, Duration},
};

mod date;
mod models;
mod request;
mod server;

const TOKEN_REFRESH_INTERVAL: Duration = Duration::from_hours(24);

fn load_credentials_from_env() -> Vec<(String, String)> {
    println!("Loading credentials from env file...");

    dotenv().ok();

    let raw = env::var("MANUS_USERS").expect("MANUS_USERS not set");

    let credentials: Vec<(String, String)> = raw
        .split(',')
        .map(|s| {
            let mut parts = s.splitn(2, ':');
            (
                parts.next().unwrap().to_string(),
                parts.next().unwrap().to_string(),
            )
        })
        .collect();

    println!("Found {} credentials", credentials.len());

    credentials
}

async fn init_accounts(credentials: &Vec<(String, String)>) -> Vec<models::Account> {
    println!("Initializing accounts...");

    let mut tasks = FuturesUnordered::new();

    for (username, password) in credentials {
        let username = username.clone();
        let password = password.clone();

        tasks.push(async move {
            let token = request::get_token(&username, &password).await.ok()?;
            let me = request::get_me(&token).await.ok()?;

            Some(models::Account {
                username,
                password,
                token,
                me,
            })
        });
    }

    let mut accounts = Vec::new();
    while let Some(account) = tasks.next().await {
        if let Some(acc) = account {
            accounts.push(acc);
        }
    }

    println!("Initialized {} accounts", accounts.len());

    accounts
}

async fn refresh_accounts(accounts: &mut Vec<models::Account>) {
    println!("Refreshing accounts...");

    for acc in accounts.iter_mut() {
        if let Ok(token) = request::get_token(&acc.username, &acc.password).await {
            acc.token = token;
            if let Ok(me) = request::get_me(&acc.token).await {
                acc.me = me;
            }
        }
    }

    println!("Refreshed {} accounts", accounts.len());
}

async fn token_refresh_loop(accounts: Arc<RwLock<Vec<models::Account>>>) {
    println!(
        "Running token refresh loop every {}s",
        TOKEN_REFRESH_INTERVAL.as_secs()
    );

    let mut interval = interval(TOKEN_REFRESH_INTERVAL);

    interval.tick().await;

    loop {
        interval.tick().await;
        let mut accounts_lock = accounts.write().await;
        refresh_accounts(&mut accounts_lock).await;
    }
}

async fn start_webserver(accounts: Arc<RwLock<Vec<models::Account>>>) {
    println!("Starting webserver...");

    let app = Router::new()
        .route("/{username}", get(server::get_calendar_for_user))
        .layer(Extension(Arc::clone(&accounts)));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3069));
    let listener = TcpListener::bind(addr).await.unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    println!("Webserver started on {}:{}", addr.ip(), addr.port());
}

#[tokio::main]
async fn main() {
    let credentials = load_credentials_from_env();
    let accounts = init_accounts(&credentials).await;
    let accounts = Arc::new(RwLock::new(accounts));

    tokio::spawn(token_refresh_loop(Arc::clone(&accounts)));

    start_webserver(Arc::clone(&accounts)).await;

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
