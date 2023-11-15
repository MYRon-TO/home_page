CREATE DATABASE yuru;

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
