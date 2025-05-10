use axum::{
    routing::get,
    response::Json,
    Router,
};
use headless_chrome::{Browser, LaunchOptionsBuilder};
use regex::Regex;
use serde_json::json;
use std::net::SocketAddr;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new().route("/", get(scrape_handler));

    // Define the address to run our server on
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);

    // Run our server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler for the root path
async fn scrape_handler() -> Json<serde_json::Value> {
    // Perform the scraping and email extraction
    match scrape_and_extract_emails() {
        Ok(emails) => Json(json!({ "emails": emails })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}
fn scrape_and_extract_emails() -> Result<Vec<String>, failure::Error> {
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .build()
            .unwrap(),
    )?;

    let tab = browser.new_tab()?;
    tab.navigate_to("https://twitter.com/phnixhamsta")?;
    tab.wait_until_navigated()?;

    std::thread::sleep(Duration::from_secs(10));

    let html_content = tab.evaluate("document.documentElement.outerHTML", false)?
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
