FROM mysql:latest

ENV MYSQL_ROOT_PASSWORD=1243
ENV MYSQL_DATABASE=yuru

COPY ./db/init_db.sql /docker-entrypoint-initdb.d/init_db.sql
