<h1>
    <a href="https://github.com/MYRon-TO/yuru" style="font-size: 2.8rem; display: flex; align-items: center;">
    <img src="./assets/images/jellyfish.png" width="70px" height="70px" alt="yuru">
            yuru
    </a>
</h1>

[Here is the source code of my blog](https://github.com/MYRon-TO/yuru)

**These code won't work on windows, I guess :(**

## Quick Start
~~make sure you have installed [**node.js**](https://nodejs.org/en) ,[**mysql**](https://www.mysql.com/downloads/) and [**rust**](https://www.rust-lang.org/zh-CN/tools/install)~~

make sure you have installed [**docker**](https://docs.docker.com/get-docker/) and [**docker-compose**](https://docs.docker.com/compose/install/)

### Init Datebase

connect to mysql and run the following sql

#### - create database
```sql
CREATE DATABASE yuru;
```

#### - create user
```sql
-- I won't show you my password :)
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
vim .config/config.toml
```

### Build
<font size=5rem color=red>**make sure you are in the root directory of this project**</font>
```bash
cargo build --release
```

### Run
```bash
cargo run --release --bin init_db
cargo run --release --bin backend
```

## What I use
- ~~apache(or lighttpd)~~
- mysql/mariadb
- node.js
  - npm
    - ~~typescript~~ <!-- actually, i didn't use it -->
    - marked <!-- markdown parser -->
    - ~~husky~~ <!-- git hook -->
    - material design <!-- css framework -->
        - material design icons
        - material desifn lite
    - sakana-widget <!-- a widget -->
- fontawesome <!-- icon font -->
- rust
  - cargo
    - axum <!-- web framework -->
    - tokio <!-- async runtime -->
    - tower <!-- service abstraction -->
    - sqlx <!-- database driver -->
    - serde <!-- json parser -->
      - serde_json <!-- json parser -->
    - toml
    - askama <!-- template engine -->
    - walkdir
