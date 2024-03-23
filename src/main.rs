use gisem_presencas::{log, login, main_loop, setup_client, Config};

const PORT: u16 = 4444;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let (duration, rate) = get_args();
    let config = Config::new(duration, rate);

    let client = setup_client(PORT).await.unwrap_or_else(|e| {
        log!("Failed to create client: {}", e);
        std::process::exit(1);
    });

    if let Ok(()) = login(&client, &config).await {
        log!("Login as {} performed successfully!", config.username);
    }

    let res = main_loop(client, &config).await;
    log!("");
    res
}

fn get_args() -> (u32, u32) {
    let mut args = std::env::args();
    args.next();

    if args.len() != 2 {
        eprintln!("Usage: gisem_presencas <duration> <rate>");
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
