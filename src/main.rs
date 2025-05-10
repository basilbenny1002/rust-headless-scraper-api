use axum::{
    extract::Query,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use headless_chrome::{Browser, LaunchOptionsBuilder};
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use std::{collections::HashMap, net::SocketAddr, time::Duration, env};

#[tokio::main]
async fn main() {
    // Set the CHROME_BIN environment variable
    env::set_var("CHROME_BIN", "/usr/bin/chromium-browser");

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/scrape", get(scrape_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root_handler() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize)]
struct ScrapeParams {
    url: String,
}

async fn scrape_handler(Query(params): Query<ScrapeParams>) -> impl IntoResponse {
    match scrape_and_extract_emails(&params.url) {
        Ok(emails) => Json(json!({ "emails": emails })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

fn scrape_and_extract_emails(target_url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .build()?,
    )?;

    let tab = browser.new_tab()?;
    tab.navigate_to(target_url)?;
    tab.wait_until_navigated()?;

    let html_content = tab
        .evaluate("document.documentElement.outerHTML", false)?
        .value
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    let email_regex = Regex::new(r"[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+")?;
    let emails = email_regex
        .find_iter(&html_content)
        .map(|mat| mat.as_str().to_string())
        .collect();

    Ok(emails)
}
