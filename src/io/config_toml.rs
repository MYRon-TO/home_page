use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DataBase,
    pub database_user: DataBaseUser,
    pub database_manager: DataBaseUser,
}

impl Config {
    pub fn new() -> Self {
        let mut file = File::open(".config/config.toml").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let config: Config = toml::from_str(&contents).unwrap();
        config
    }

    pub fn get(&self) -> String {
        let before = format!(
            "mysql://{}:{}@{}/{}",
            self.database_user.get_user(),
            self.database_user.get_password(),
            self.database.get_host(),
            self.database.get_database()
        );
        const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS
            .add(b' ')
            .add(b'"')
            .add(b'#')
            .add(b'<')
            .add(b'>')
            .add(b'`');
        utf8_percent_encode(&before, &QUERY_ENCODE_SET).to_string()
    }

    pub fn get_manager(&self) -> String {
        let before = format!(
            "mysql://{}:{}@{}/{}",
            self.database_manager.get_user(),
            self.database_manager.get_password(),
            self.database.get_host(),
            self.database.get_database()
        );
        const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS
            .add(b' ')
            .add(b'"')
            .add(b'#')
            .add(b'<')
            .add(b'>')
            .add(b'`');
        utf8_percent_encode(&before, &QUERY_ENCODE_SET).to_string()
    }
}

#[derive(Debug, Deserialize)]
pub struct DataBase {
    host: String,
    database: String,
}

#[derive(Debug, Deserialize)]
pub struct DataBaseUser {
    user: String,
    password: String,
}

impl DataBaseUser {
    pub fn get_user(&self) -> String {
        self.user.clone()
    }
    pub fn get_password(&self) -> String {
        self.password.clone()
    }
}

impl DataBase {
    pub fn get_host(&self) -> String {
        self.host.clone()
    }
    pub fn get_database(&self) -> String {
        self.database.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::new();
        assert_eq!(
            config.get(),
            "mysql://BlueBird:%23ff0000Berry@localhost/yuru"
        );
    }
}
