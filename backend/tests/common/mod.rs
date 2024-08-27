pub fn get_api_path() -> anyhow::Result<String> {
    dotenv::dotenv()?;
    let api_path = std::env::var("SERVER_FULL_ADDRESS")?;
    let with_http = format!("http://{api_path}/api");
    Ok(with_http)
}
