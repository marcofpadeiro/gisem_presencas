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
