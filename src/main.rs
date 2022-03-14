use chrono;
use dotenv::dotenv;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use std::env;

use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::Write;

mod ftp;
mod utils;

//Function to extract only the numbers out from the text
fn number_only(input: String) -> String {
    let mut info: String = String::new();
    for i in input.chars() {
        if i.is_numeric() {
            let found = i.to_string();
            info.push_str(&found);
        } else {
            ()
        }
    }
    info
}

//Save the json file
fn save_info(info: &HashMap<String, String>) {
    let dt = chrono::Local::now();
    create_dir_all("./holders").unwrap_or_else(|e| panic!("Error creating dir: {}", e));
    let filename = format!(
        "{}{}{}{}{}",
        "./holders/",
        &env::var("TOKEN_NAME").expect("TOKEN_NAME must be set"),
        "_Holders_",
        dt.format("%Y-%m-%d_%H.%M.%S"),
        ".json"
    );
    let mut writer = File::create(&filename).unwrap();
    for (k, v) in info.iter() {
        let data_to_write = format!("{}{}{}", k, ":", v);
        writeln!(
            &mut writer,
            "{}",
            &serde_json::to_string(&data_to_write).unwrap()
        )
        .unwrap();
    }
    println!("Information saved!");
}

//Scrape the website and create the hashmap
#[tokio::main]
async fn web_scrape(url: &str) -> HashMap<String, String> {
    //Getting the client from utils.rs and starting it with the set website url.
    let client = utils::get_client();
    let result = client.get(url).send().await.unwrap();

    //Getting the raw html data
    let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong"),
    };

    //Parsing the html file and set up the selecting requirments
    let document = Html::parse_document(&raw_html);
    let article_selector = Selector::parse("span").unwrap();

    let mut info: HashMap<String, String> = HashMap::new();
    //Iterating on the span elements
    for element in document.select(&article_selector) {
        let inner = element.inner_html().to_string();
        //If the element contains the snapshot taken or holders info
        if inner.contains("Holders Snapshot taken") {
            let num = number_only(inner);
            info.insert("Snapshot".to_string(), num);
        } else if inner.contains("From a total of") {
            let num = number_only(inner);
            info.insert("Holders".to_string(), num);
        } else {
            ()
        }
    }
    //Returning the hashmap with the holders info and when the snapshot taken
    info
}

fn main() {
    dotenv().expect("Failed to read .env file");
    let url = &env::var("TOKEN_LINK").expect("TOKEN_LINK must be set");
    let web_scrape_result = web_scrape(&url);
    save_info(&web_scrape_result);
    ftp::upload();
}
