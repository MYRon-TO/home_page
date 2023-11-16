# What I need
- python3
  - mysql-connector-python
- php
  - composer
    - psalm
    - toml
- node.js
  - npm
    - typescript <!-- actually, i didn't use it -->
    - marked

# What to do

## Init datebase
### create database
```sql
CREATE DATABASE yuru;
```

### create tables

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

### create user
```sql
CREATE USER 'user_can_read'@'localhost' IDENTIFIED BY 'password';
CREATE USER 'user_can_write'@'localhost' IDENTIFIED BY 'password';

GRANT SELECT ON yuru.* TO 'user_can_read'@'localhost';
GRANT ALL PRIVILEGES ON yuru.* TO 'user_can_write'@'localhost';
```
** remember to put the info of the user into the /yuru/.config/config.toml **
