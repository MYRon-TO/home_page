CREATE USER 'reader'@'localhost' IDENTIFIED BY 'read';
CREATE USER 'writer'@'localhost' IDENTIFIED BY 'write';

GRANT SELECT ON yuru.* TO 'reader'@'*';
GRANT ALL PRIVILEGES ON yuru.* TO 'writer'@'*';
