use std::{env, sync::Arc};

use reqwest::{cookie::Jar, Client};
use scraper::{Html, Selector};

pub async fn login() -> Client {
    // 環境変数の読み込み
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID is not set");
    let email = env::var("EMAIL").expect("EMAIL is not set");
    let password = env::var("PASSWORD").expect("PASSWORD is not set");

    // クッキーストアの作成
    let cookie_store = Arc::new(Jar::default());
    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(cookie_store.clone())
        .build()
        .unwrap();

    // CSRFトークンの取得
    let login_page_res_body = client
        .get("https://ssl.jobcan.jp/login/mb-employee?client_id=ChatWork&lang_code=ja")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let token = Html::parse_document(&login_page_res_body)
        .select(&Selector::parse(r#"input[name="token"]"#).unwrap())
        .next()
        .unwrap()
        .value()
        .attr("value")
        .unwrap()
        .to_string();

    // ログイン
    let login_res = client
        .post("https://ssl.jobcan.jp/login/mb-employee")
        .form(&[
            ("client_id", client_id.as_str()),
            ("lang_code", "ja"),
            ("token", token.as_str()),
            ("email", email.as_str()),
            ("password", password.as_str()),
        ])
        .send()
        .await
        .unwrap();

    if !login_res.status().is_success() {
        panic!("Failed to login");
    }

    client
}
