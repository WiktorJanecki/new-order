use anyhow::Result;
use common::get_api_path;

mod common;

#[tokio::test]
async fn ping() -> Result<()> {
    let api_path = get_api_path()?;
    let res = reqwest::get(&format!("{api_path}/ping"))
        .await?
        .text()
        .await?;
    assert_eq!("pong", &res);

    Ok(())
}
