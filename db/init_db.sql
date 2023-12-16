CREATE USER 'BlueBird'@'%' IDENTIFIED BY '#0000ffBerry';
CREATE USER 'RedBird'@'%' IDENTIFIED BY '#ff0000Berry';

GRANT SELECT ON yuru.* TO 'BlueBird'@'%';
GRANT ALL PRIVILEGES ON yuru.* TO 'RedBird'@'%';
