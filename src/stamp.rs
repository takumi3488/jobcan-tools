use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use scraper::Html;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::login::login;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct StampRequest {
    pub mode: Mode,
}

#[derive(Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Mode {
    Start,
    End,
}

#[utoipa::path(
    post,
    path = "/api/stamp",
    request_body=StampRequest,
    responses(
        (status = 204, description = "Stamped successfully"),
        (status = 409, description = "Conflict", body = String)
    )
)]
pub async fn stamp(Json(stamp_request): Json<StampRequest>) -> impl IntoResponse {
    let client = login().await;
    let mobile_page_res_body = client
        .get("https://ssl.jobcan.jp/m/work/accessrecord?_m=adit")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let current_state = Html::parse_document(&mobile_page_res_body)
        .select(&scraper::Selector::parse(r#"#current_time + div"#).unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<String>();
    if let Err(e) = check_current_state(&current_state, stamp_request.mode) {
        return (StatusCode::CONFLICT, e).into_response();
    }
    StatusCode::NO_CONTENT.into_response()
}

/// current_stateとmodeが矛盾していないかチェックする
fn check_current_state(current_state: &str, mode: Mode) -> Result<(), String> {
    if mode == Mode::Start {
        if current_state == "勤務中" {
            return Err("Already started".to_string());
        }
    } else {
        if vec!["退出中", "未出勤"].contains(&current_state) {
            return Err("Already ended".to_string());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_current_state() {
        let table = vec![
            (Mode::Start, "勤務中", Err("Already started".to_string())),
            (Mode::Start, "退出中", Ok(())),
            (Mode::Start, "未出勤", Ok(())),
            (Mode::End, "勤務中", Ok(())),
            (Mode::End, "退出中", Err("Already ended".to_string())),
            (Mode::End, "未出勤", Err("Already ended".to_string())),
        ];
        for (mode, current_state, expected) in table {
            assert_eq!(check_current_state(current_state, mode), expected);
        }
    }
}
