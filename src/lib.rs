use std::time::Duration;

use fantoccini::{ClientBuilder, Locator};

const URL_LOGIN: &str = "https://gisem.dei.estg.ipleiria.pt/login";
const URL_PRESENCAS: &str = "https://gisem.dei.estg.ipleiria.pt/obterAulasMarcarPresenca";

pub struct Config {
    pub username: String,
    pub password: String,
    pub duration: u32, // in minutes
    pub rate: u32,     // in seconds
}

impl Config {
    pub fn new(duration: u32, rate: u32) -> Self {
        let file = std::fs::read_to_string("credentials.txt").expect("credentials.txt not found");
        let vec = file.split_once(':').unwrap();
        Self {
            username: vec.0.to_string(),
            password: vec.1.to_string(),
            duration,
            rate,
        }
    }
}

pub async fn setup_client(
    port: u16,
) -> Result<fantoccini::Client, fantoccini::error::NewSessionError> {
    let mut caps = serde_json::map::Map::new();
    let opts = serde_json::json!({
        "args": ["-headless"],
    });
    caps.insert("moz:firefoxOptions".to_string(), opts);

    ClientBuilder::rustls()
        .capabilities(caps)
        .connect(format!("http://localhost:{}", port).as_str())
        .await
}

pub async fn login(
    c: &fantoccini::Client,
    config: &Config,
) -> Result<(), fantoccini::error::CmdError> {
    // go to login page
    c.goto(URL_LOGIN).await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), URL_LOGIN);

    // fill in the login form
    c.find(Locator::Css("input[name=username]"))
        .await?
        .send_keys(config.username.as_str())
        .await?;
    c.find(Locator::Css("input[name=password]"))
        .await?
        .send_keys(config.password.as_str())
        .await?;
    c.find(Locator::Css("button[type=submit]"))
        .await?
        .click()
        .await?;

    Ok(())
}

pub async fn main_loop(
    client: fantoccini::Client,
    config: &Config,
) -> Result<(), fantoccini::error::CmdError> {
    let start = std::time::Instant::now();

    // stops if the duration is reached or if the presences were already marked successfully
    loop {
        log!("Refreshing");
        client.refresh().await?;

        let link_result = client
            .find(Locator::Css(
                "a[href='https://gisem.dei.estg.ipleiria.pt/obterAulasMarcarPresenca']",
            ))
            .await;

        if link_result.is_err() {
            if start.elapsed().as_secs() / 60 >= config.duration as u64 {
                return client.close().await;
            }
            log!("Presenças fechadas");

            tokio::time::sleep(Duration::from_secs(config.rate as u64)).await;
            continue;
        }

        link_result.unwrap().click().await?;

        let url = client.current_url().await?;
        assert_eq!(url.as_ref(), URL_PRESENCAS);

        client
            .find(Locator::Css("button[class='btn btn-primary col-xs-12']"))
            .await?
            .click()
            .await?;

        tokio::time::sleep(Duration::from_secs(1)).await;

        let message = client.get_alert_text().await?;

        log!("{}", message);
        if message.contains("sucesso")
            || message.contains("Marcação de presença já foi efetuada anteriormente.")
        {
            return client.close().await;
        }
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        use chrono::Local;
        use std::fs::OpenOptions;
        use std::io::Write;
        use std::fs;

        let now = Local::now();
        let msg = format!("{} {}\n", now.format("[%Y-%m-%d %H:%M:%S]"), format_args!($($arg)*));
        eprintln!("{}", msg);

        fs::create_dir_all("logs").unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(format!("logs/{}", now.format("%Y-%m-%d.log")))
            .unwrap();

        file.write_all(msg.as_bytes()).unwrap();
    }};
}
