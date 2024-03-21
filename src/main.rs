use std::time::Duration;

use fantoccini::{ClientBuilder, Locator};
use gisem_rust::Config;

const URL_LOGIN: &str = "https://gisem.dei.estg.ipleiria.pt/login";
const URL_PRESENCAS: &str = "https://gisem.dei.estg.ipleiria.pt/obterAulasMarcarPresenca";

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let (duration, rate) = get_args();
    let config = Config::new(duration, rate);

    let mut caps = serde_json::map::Map::new();
    let opts = serde_json::json!({
        "args": ["-headless"],
    });
    caps.insert("moz:firefoxOptions".to_string(), opts);

    let c = ClientBuilder::rustls()
        .capabilities(caps)
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    login(&c, config.username, config.password).await?;

    let start = std::time::Instant::now();

    loop {
        eprintln!("Refreshing");
        c.refresh().await?;

        let link_result = c
            .find(Locator::Css(
                "a[href='https://gisem.dei.estg.ipleiria.pt/obterAulasMarcarPresenca']",
            ))
            .await;

        if link_result.is_err() {
            if start.elapsed().as_secs() / 60 >= config.duration as u64 {
                return c.close().await;
            }
            eprintln!("Presenças fechadas");
            tokio::time::sleep(Duration::from_secs(config.rate as u64)).await;
            continue;
        }

        link_result.unwrap().click().await?;

        let url = c.current_url().await?;
        assert_eq!(url.as_ref(), URL_PRESENCAS);

        c.find(Locator::Css("button[class='btn btn-primary col-xs-12']"))
            .await?
            .click()
            .await?;

        tokio::time::sleep(Duration::from_secs(1)).await;

        let message = c.get_alert_text().await?;

        eprintln!("{}", message);
        if message.contains("sucesso")
            || message.contains("Marcação de presença já foi efetuada anteriormente.")
        {
            return c.close().await;
        }
    }
}

async fn login(
    c: &fantoccini::Client,
    username: String,
    password: String,
) -> Result<(), fantoccini::error::CmdError> {
    // go to login page
    c.goto(URL_LOGIN).await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), URL_LOGIN);

    // fill in the login form
    c.find(Locator::Css("input[name=username]"))
        .await?
        .send_keys(username.as_str())
        .await?;
    c.find(Locator::Css("input[name=password]"))
        .await?
        .send_keys(password.as_str())
        .await?;
    c.find(Locator::Css("button[type=submit]"))
        .await?
        .click()
        .await?;

    Ok(())
}

fn get_args() -> (u32, u32) {
    let mut args = std::env::args();
    args.next();

    if args.len() != 2 {
        eprintln!("Usage: gisem_rust <duration> <rate>");
        eprintln!("\tduration - minutes");
        eprintln!("\trate - seconds");
        std::process::exit(1);
    }

    let duration = args.next().unwrap().parse::<u32>().unwrap_or_else(|_| {
        eprintln!("Invalid duration, should be a number");
        std::process::exit(1);
    });

    let rate = args.next().unwrap().parse::<u32>().unwrap_or_else(|_| {
        eprintln!("Invalid rate, should be a number");
        std::process::exit(1);
    });

    (duration, rate)
}
