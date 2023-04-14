mod selenium;
mod manager;

use thirtyfour::prelude::WebDriverResult;


#[tokio::main]
async fn main() -> WebDriverResult<()> {

    manager::get_web_driver().await.unwrap();
    selenium::send_to_whatsapp("Holi").await?;

    Ok(())
}
