use crate::responses::utils;

use log::error;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use serde::Serialize;
use serde_json::{json, Value};
use tauri::Url;

#[derive(Debug, Serialize, Clone)]
struct TargetItem {
    title: String,
    url: String,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command(rename_all = "snake_case")]
pub fn get_list(target_url: &str) -> Result<Value, Value> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(target_url.to_string())
        .header(USER_AGENT, "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Mobile Safari/537.36")
        .send()
        .map_err(|e| {
            error!("error: {:?}", e);
            utils::handler(None, None, None,Some("error"))
        });

    let body = match response {
        Ok(res) => res.text().map_err(|e| {
            error!("error: {:?}", e);
            utils::handler(None, None, None, Some("error"))
        })?,
        Err(e) => {
            error!("error: {:?}", e);
            return Err(utils::handler(None, None, None, Some("error")));
        }
    };

    let tag_appearance = utils::count_tag(&body).unwrap();

    let fragment = Html::parse_fragment(&body);
    let selector = Selector::parse("li").unwrap();

    // target_urlからルートドメインを取得
    let root_domain = match Url::parse(target_url) {
        Ok(get_url) => {
            let host = get_url.host_str().unwrap_or_default().to_string();
            host
        }
        Err(e) => {
            error!("error: {:?}", e);
            return Err(utils::handler(None, None, None, Some("error")));
        }
    };

    let mut array = vec![];
    for element in fragment.select(&selector) {
        let url = match element.select(&Selector::parse("a").unwrap()).next() {
            Some(e) => {
                // 相対パスの場合はルートドメインを付与
                let href = e.value().attr("href").unwrap_or_default();
                if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("{}{}", root_domain, href).to_string()
                }
            }
            None => continue,
        };

        array.push(TargetItem {
            title: element.text().collect::<String>(),
            url: url.to_string(),
        });
    }

    Ok(utils::handler(
        Some("success"),
        Some(json!(array)),
        Some(json!(tag_appearance)),
        None,
    ))
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_comments(target_url: &str, tag: &str) -> Result<Value, Value> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(target_url.to_string())
        .header(USER_AGENT, "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Mobile Safari/537.36")
        .send()
        .map_err(|e| {
            error!("error: {:?}", e);
            utils::handler(None, None, None,Some(format!("error: {:?}", e).as_str()))
        });

    let body = match response {
        Ok(res) => res.text().map_err(|e| {
            error!("error: {:?}", e);
            utils::handler(None, None, None, Some(format!("error: {:?}", e).as_str()))
        })?,
        Err(e) => {
            error!("error: {:?}", e);
            return Err(utils::handler(
                None,
                None,
                None,
                Some(format!("error: {:?}", e).as_str()),
            ));
        }
    };

    let fragment = Html::parse_fragment(&body);
    // 全要素からdiv.threadview_response_bodyをリスト
    let selector = Selector::parse(tag).unwrap();

    let mut array = vec![];
    for element in fragment.select(&selector) {
        let text = element.text().collect::<String>();

        array.push(TargetItem {
            title: text,
            url: "".to_string(),
        });
    }

    Ok(utils::handler(
        Some("success"),
        Some(json!(array)),
        None,
        None,
    ))
}

#[tauri::command(rename_all = "snake_case")]
pub fn save_value(value: Value) -> Result<Value, Value> {
    let os_document_dir = dirs::document_dir().unwrap();

    let output_filepath = format!("{}/output.csv", os_document_dir.to_str().unwrap());
    let mut w = csv::Writer::from_path(output_filepath.clone()).unwrap();
    w.write_record(["title", "url", "option"]).unwrap();
    for item in value.as_array().unwrap() {
        let title = item["title"].as_str().unwrap_or_default();
        let clean_title = utils::clean_string(title);
        let url = item["url"].as_str().unwrap_or_default();
        let clean_url = utils::clean_string(url);
        let option = item["option"].as_str().unwrap_or_default();
        let clean_option = utils::clean_string(option);

        w.write_record([clean_title, clean_url, clean_option])
            .unwrap();
    }
    w.flush().unwrap();

    Ok(utils::handler(
        Some(format!("success, output filepath: {}", output_filepath).as_str()),
        Some(value),
        None,
        None,
    ))
}
