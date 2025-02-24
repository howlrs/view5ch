use std::collections::HashMap;

use log::info;
use regex::Regex;
use scraper::{Html, Selector};
use serde_json::{json, Value};

pub fn handler(
    msg: Option<&str>,
    data: Option<Value>,
    op: Option<Value>,
    err: Option<&str>,
) -> Value {
    info!("request value: {:?}", msg);
    json!({
        "message": msg.unwrap_or_default(),
        "data": data.unwrap_or_default(),
        "option": op.unwrap_or_default(),
        "error": err.unwrap_or_default(),
    })
}

// htmlから最頻出のタグとその数を取得
pub fn count_tag(html_body: &str) -> Result<HashMap<String, i32>, Value> {
    // htmlをパース
    let fragment = Html::parse_fragment(html_body);
    // 全要素からタグをリスト
    let selector = Selector::parse("*").unwrap();
    // 最頻出のタグを取得
    let mut tag_count = std::collections::HashMap::new();
    for element in fragment.select(&selector) {
        let tag_name = element.value().name().to_string();
        let class_name = element.value().attr("class").unwrap_or("").to_string();
        let id_name = element.value().attr("id").unwrap_or("").to_string();

        let key = format!(
            "{}{}{}",
            tag_name,
            if !id_name.is_empty() {
                format!("#{}", id_name.replace(" ", "#"))
            } else {
                "".to_string()
            },
            if !class_name.is_empty() {
                format!(".{}", class_name.replace(" ", "."))
            } else {
                "".to_string()
            }
        );

        let count = tag_count.entry(key).or_insert(0);
        *count += 1;
    }

    Ok(tag_count)
}

pub fn clean_string(input: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();
    re.replace_all(input, " ").trim().to_string()
}
