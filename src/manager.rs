use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

use tokio::time::sleep;
use tokio::process::Command;

use selenium_manager;
use selenium_manager::logger::Logger;


const BROWSER_NAME: &str = selenium_manager::chrome::CHROME_NAME;
const LOGGER_LEVEL: &str = "INFO";
const DEBUG: bool = false;
const TIMEOUT: u64 = 1000;

#[derive(Debug)]
pub enum DriverError {
    InvalidBrowser(String),
    ErrorGettingDriver(Box<dyn Error>),
}

pub async fn get_web_driver() -> Result<(), DriverError> {
    let driver_path = tokio::task::spawn_blocking(|| {
        return download_driver().unwrap();
    }).await.unwrap();

    run_driver(driver_path).await;

    Ok(())
}

fn download_driver() -> Result<PathBuf, DriverError> {
    let mut manager = selenium_manager::get_manager_by_browser(BROWSER_NAME.to_string()).map_err(|err| DriverError::InvalidBrowser(err))?;

    let logger = Logger::create(LOGGER_LEVEL.to_string(), DEBUG, false);
    manager.set_logger(logger);


    manager.set_timeout(TIMEOUT)
        .and_then(|_| manager.resolve_driver())
        .map(|path| {
            let log = manager.get_logger();
            log.info(path.display().to_string());
            flush_log(0, log);
        })
        .map_err(|err| {
            let log = manager.get_logger();
            log.error(err.to_string());
            flush_log(1, log);
            DriverError::ErrorGettingDriver(err)
        })?;

    Ok(manager.get_driver_path_in_cache())
}

async fn run_driver(program_path: PathBuf) {
    // TODO do not leave the thread hanging indefinitely
    Command::new(program_path).status();
    sleep(Duration::from_millis(1000)).await;
}


fn flush_log(code: i32, log: &Logger) {
    log.set_code(code);
    log.flush();
}