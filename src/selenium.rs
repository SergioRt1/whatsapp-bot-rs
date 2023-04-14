use std::time::Duration;

use tokio::time::sleep;

use thirtyfour::prelude::{DesiredCapabilities, WebDriver, WebDriverResult, By, ElementQueryable, ElementWaitable, Key};

const WEB_DRIVER_URL: &str = "http://localhost:9515";
const WHATSAPP_URL: &str = "https://web.whatsapp.com";
const CHAT_NAME: &str = "Basura";

const SEARCH_BAR: &str = r#"//*[@id="side"]/div[1]/div/div/div[2]/div/div[1]/p"#;
const TEXT_BAR: &str = r#"//*[@id="main"]/footer/div[1]/div/span[2]/div/div[2]/div[1]/div/div[1]/p"#;


pub async fn send_to_whatsapp(message: &str) -> WebDriverResult<()> {
    let driver = init_driver().await?;
    load_page(&driver).await?;
    open_chat(&driver).await?;
    send_message(&driver, message).await?;
    driver.quit().await?;

    Ok(())
}

async fn init_driver() -> WebDriverResult<WebDriver> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(WEB_DRIVER_URL, caps).await?;

    Ok(driver)
}

async fn load_page(driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto(WHATSAPP_URL).await?;

    let delay = Duration::new(30, 0);
    driver.set_implicit_wait_timeout(delay).await?;
    // Wait for the user to scan the QR code and for the chat window to load
    driver.query(By::XPath(SEARCH_BAR))
        .first().await?
        .wait_until().displayed().await?;

    Ok(())
}

async fn open_chat(driver: &WebDriver) -> WebDriverResult<()> {
    let search_bar = driver.find(By::XPath(SEARCH_BAR)).await?;

    search_bar.send_keys(CHAT_NAME).await?;
    let enter = get_key_as_string(Key::Enter);
    search_bar.send_keys(enter).await?;

    // Wait for the chat to load
    driver.query(By::Css("#main > header > div:nth-child(2) > div > span"))
        .first().await?
        .wait_until().displayed().await?;

    Ok(())
}

async fn send_message(driver: &WebDriver, message: &str) -> WebDriverResult<()> {
    let input_box = driver.find(By::XPath(TEXT_BAR)).await?;

    input_box.send_keys(message).await?;

    let enter = get_key_as_string(Key::Enter);
    input_box.send_keys(enter).await?;

    sleep(Duration::from_millis(500)).await;

    Ok(())
}

fn get_key_as_string(key: Key) -> String {
    return char::from(key).to_string();
}