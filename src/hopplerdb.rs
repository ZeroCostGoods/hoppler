use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use serde_yaml;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct ServerConfig {
    pub db_url: String,
}

// get_config read the config file to find the db url
fn get_config() -> Result<ServerConfig, Error> {
    let mut config_file = File::open("config/config.yaml")?;

    let mut contents = String::new();
    if let Err(err) = config_file.read_to_string(&mut contents) {
        return Err(err);
    }

    let config: ServerConfig = serde_yaml::from_str(contents.as_str()).unwrap();

    Ok(config)
}

// start the database connection
// FIXME(digant) -- introduce connection pooling
pub fn establish_connection() -> MysqlConnection {
    let config = get_config().expect("Could not load config");

    MysqlConnection::establish(&config.db_url)
        .expect(&format!("Error connecting to {}", &config.db_url))
}