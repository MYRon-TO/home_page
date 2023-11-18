use std::{fs::File, io::Read};
use serde::Deserialize;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

#[derive(Debug, Deserialize)]
pub struct Config {
  pub database: DataBase,
}

impl Config {
  pub fn new() -> Self {
    let mut file = File::open(".config/config.toml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = toml::from_str(&contents).unwrap();
    config
  }
}

#[derive(Debug, Deserialize)]
pub struct DataBase {
  user: String,
  password: String,
  host: String,
  database: String,
}

  // pub fn get(&self) -> String {
impl DataBase {
  pub fn get(&self) -> String {
    let before = format!("mysql://{}:{}@{}/{}", self.user, self.password, self.host, self.database);
    const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>').add(b'`');
    utf8_percent_encode(&before, &QUERY_ENCODE_SET).to_string()
  }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::new();
        assert_eq!(config.database.get(), "mysql://BlueBird:%23ff0000Berry@localhost/yuru");
    }
}
