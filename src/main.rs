use std::time::Duration;

use fantoccini::{ClientBuilder, Locator};

const URL_LOGIN: &str = "https://gisem.dei.estg.ipleiria.pt/login";
const URL_PRESENCAS: &str = "https://gisem.dei.estg.ipleiria.pt/obterAulasMarcarPresenca";

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let (username, password) = get_credentials();

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

    login(&c, username, password).await?;

    if c.find(Locator::Css(
        "a[href='https://gisem.dei.estg.ipleiria.pt/obterAulasMarcarPresenca']",
    ))
    .await
    .is_err()
    {
        c.close().await?;
        eprintln!("Presenças fechadas");
        std::process::exit(1);
    }

    c.find(Locator::Css(
        "a[href='https://gisem.dei.estg.ipleiria.pt/obterAulasMarcarPresenca']",
    ))
    .await?
    .click()
    .await?;

    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), URL_PRESENCAS);

    c.find(Locator::Css("button[class='btn btn-primary col-xs-12']"))
        .await?
        .click()
        .await?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    let message = c.get_alert_text().await?;

    if message.contains("sucesso") {
        eprintln!("{}", message);
    } else {
        eprintln!("{}", message);
        std::process::exit(2);
    }

    c.close().await
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

fn get_credentials() -> (String, String) {
    let file = std::fs::read_to_string("credentials.txt").expect("credentials.txt not found");
    let vec = file.split_once(':').unwrap();
    (vec.0.to_string(), vec.1.to_string())
}
