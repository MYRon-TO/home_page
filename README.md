# yuru
Here is the source code of my blog

**These code won't work on windows, I guess :(**

## Quick Start
make sure you have installed [**node.js**](https://nodejs.org/en) ,[**mysql**](https://www.mysql.com/downloads/) and [**rust**](https://www.rust-lang.org/zh-CN/tools/install)

### Init Datebase

connect to mysql and run the following sql

#### - create database
```sql
CREATE DATABASE yuru;
```

#### - create user
```sql
-- I won't tell you my password :)
CREATE USER 'user_can_read'@'localhost' IDENTIFIED BY 'password';
CREATE USER 'user_can_write'@'localhost' IDENTIFIED BY 'password';

GRANT SELECT ON yuru.* TO 'user_can_read'@'localhost';
GRANT ALL PRIVILEGES ON yuru.* TO 'user_can_write'@'localhost';
```

#### - fill the config
```bash
mv .config/config.toml.example .config/config.toml
```
choose a editor you like :)
```bash
vim ./config/config.toml
```

### Build
<font size=5rem color=red>**make sure you are in the root directory of this project**</font>
```bash
cargo build --release
cargo run --bin init_db
cargo run --bin backend
```

## What I use
- apache(or lighttpd)
- mysql
- node.js
  - npm
    - typescript <!-- actually, i didn't use it -->
    - marked <!-- markdown parser -->
    - husky <!-- git hook -->
    - material design <!-- css framework -->
        - material design icons
        - material desifn lite
- rust
  - cargo
    - axum <!-- web framework -->
    - sqlx <!-- database driver -->
    - serde <!-- json parser -->
      - serde_json <!-- json parser -->
    - askama <!-- template engine -->
