# A bot for WhatsApp in Rust

## Description

This project uses selenium through the [thirtyfour](https://docs.rs/thirtyfour/latest/thirtyfour/) library to interact
with the Chrome browser and simulate the interaction of a user to send an automated message to a WhatsApp chat.

To manage the WebDriver, we use [selenium-manager](https://github.com/SeleniumHQ/selenium/tree/trunk/rust), which is a
standalone tool that automatically manages the browser infrastructure required by Selenium. It is implemented as a CLI (
Command-Line Interface) tool in Rust, so we import it as a library and use its internal functions to automate the
management of the WebDriver.

---

## Prerequisites :toolbox:

* [Rust](https://www.rust-lang.org) 1.68+

To run it from the source code, you need
to [install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### Build :hammer_and_wrench:

```
cargo build
```

### Run :rocket:

```
cargo run
```

---

## TODO :magic_wand: 

* Fix WebDriver running in background.
* include some API that provides relevant information to send in the bot message.
* Use cookies or cache to maintain the WhatsApp session, currently it is necessary to log in every time the program is
  executed.

---

## Author

* **[Sergio Rodríguez](https://github.com/SergioRt1)**

## License

This project is licensed under the Apache-2.0 License - see the [LICENSE](LICENSE) file for details