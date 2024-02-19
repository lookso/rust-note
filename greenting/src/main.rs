use std::error::Error;
use reqwest;
use tokio::runtime::Runtime;

async fn get_api() -> Result<(), Box<dyn Error>> {
    let url = "https://mon.zijieapi.com/monitor_web/settings/browser-settings?bid=douyin_web&store=1";
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;

    let data = &response["errno"];
    println!("{:#?}", data);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;
    rt.block_on(get_api())
}