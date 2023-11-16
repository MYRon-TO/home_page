CREATE DATABASE yuru;

USE yuru;

CREATE TABLE IF NOT EXISTS serise (
    name VARCHAR(255) PRIMARY KEY,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    info_path VARCHAR(255) NOT NULL
    );

CREATE TABLE IF NOT EXISTS blog_serise (
    id INT AUTO_INCREMENT PRIMARY KEY,
    info_path VARCHAR(255) NOT NULL,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    serise VARCHAR(255),
    FOREIGN KEY (serise) REFERENCES serise(name)
    );

CREATE TABLE IF NOT EXISTS tag (
    blog_id INT AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    PRIMARY KEY (blog_id, name),
    FOREIGN KEY (blog_id) REFERENCES blog_serise(id)
    );
