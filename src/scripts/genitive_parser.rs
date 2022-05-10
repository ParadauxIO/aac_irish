use std::collections::HashMap;
use std::error::Error;

async fn genitive_parser(text: String) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    let client = reqwest::Client::new();

    let mut request_url: String = "https://phoneticsrv3.lcs.tcd.ie/gramsrv/api/grammar?text=".to_owned();
    request_url.push_str(&text.to_owned());
    request_url.push_str("&check=genitive");

    // client.get(request_url).send().await?
    //     .json::<serde_json::Value>()
    //     .await?
    return map;
}


fn apply_changes(text: &str, rule_id: &str, message: &str) {
    if rule_id == "GENITIVE" {

    }
}

fn process_errors(text: &str, errors: Vec<&str>) {

}

fn test_parser() {
    // fear mór an post

    // hata an fear

    // fear an post
}