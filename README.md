# yuru
here is the source code of my blog

**these code won't work on windows, i guess :(**

## What I used
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
    - axum <!-- web framework -->
    - sqlx <!-- database driver -->
    - serde <!-- json parser -->
    - serde_json <!-- json parser -->
    - chrono <!-- date time -->

## Quick Start
make sure you have installed node.js ,mysql and rust

### Init datebase

connect to mysql and run the following sql

#### create database
```sql
CREATE DATABASE yuru;
```

#### create tables

```sql
USE yuru;

CREATE TABLE IF NOT EXISTS serise (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE IF NOT EXISTS blog_serise (
    id INT AUTO_INCREMENT PRIMARY KEY,
    info_path VARCHAR(255) NOT NULL,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    serise_id INT,
    FOREIGN KEY (serise_id) REFERENCES serise(id)
    );

CREATE TABLE IF NOT EXISTS tag (
    blog_id INT,
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (blog_id, name),
    FOREIGN KEY (blog_id) REFERENCES blog_serise(id)
    );
```

#### create user
```sql
-- I won't show you my password :)

CREATE USER 'user_can_read'@'localhost' IDENTIFIED BY 'password';
CREATE USER 'user_can_write'@'localhost' IDENTIFIED BY 'password';

GRANT SELECT ON yuru.* TO 'user_can_read'@'localhost';
GRANT ALL PRIVILEGES ON yuru.* TO 'user_can_write'@'localhost';
```

### Init backend
```bash
cd backend
cargo build
```
