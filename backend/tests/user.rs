use std::str::FromStr;

use reqwest::{
    header::{self, SET_COOKIE},
    StatusCode,
};
use serde::Serialize;
use tower_cookies::Cookie;
use tracing::{info, Instrument};

mod common;

#[derive(Serialize)]
struct LoginPayload {
    login: String,
    password: String,
}

// requires running backend with postgresql and mock_data migration on
#[tokio::test]
async fn login() -> anyhow::Result<()> {
    let api_path = common::get_api_path()?;

    let client = reqwest::Client::new();

    // user does not exist
    let payload = LoginPayload {
        login: "kazimierz".to_owned(),
        password: "123".to_owned(),
    };
    let res = client
        .post(&format!("{api_path}/login"))
        .json(&payload)
        .send()
        .await?;

    // TODO: in future login fail code can be different so check if it is correct and check error message
    assert!(res.status() != StatusCode::OK);

    // bad password
    let payload = LoginPayload {
        login: "boss".to_owned(),
        password: "123_ale_4".to_owned(),
    };
    let res = client
        .post(&format!("{api_path}/login"))
        .json(&payload)
        .send()
        .await?;
    // TODO: in future login fail code can be different so check if it is correct and check error message
    assert!(res.status() != StatusCode::OK);

    // ok
    let payload = LoginPayload {
        login: "boss".to_owned(),
        password: "123".to_owned(),
    };
    let res = client
        .post(&format!("{api_path}/login"))
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    let token = res
        .headers()
        .get(SET_COOKIE)
        .ok_or(anyhow::anyhow!("no set-cookie header"))?
        .to_str()?;

    let cookie = Cookie::from_str(token);

    assert!(cookie.is_ok());

    Ok(())
}

#[tokio::test]
async fn token() -> anyhow::Result<()> {
    let api_path = common::get_api_path()?;
    let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;

    // token without cookie
    let res = client.get(api_path.clone() + "/token").send().await?;
    assert!(res.status() != StatusCode::OK);

    // token with bad cookie
    let res = client
        .get(api_path.clone() + "/token")
        .header(header::COOKIE, "AUTH_TOKEN=failed")
        .send()
        .await?;
    assert!(res.status() != StatusCode::OK);

    // ok
    let payload = LoginPayload {
        login: "boss".to_owned(),
        password: "123".to_owned(),
    };
    let res = client
        .post(&format!("{api_path}/login"))
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let res = client.get(api_path.clone() + "/token").send().await?;
    assert_eq!(res.status(), StatusCode::OK);

    Ok(())
}
