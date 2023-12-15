# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/engine/reference/builder/

################################################################################
# Create a stage for building the application.

ARG RUST_VERSION=1.73.0
ARG APP_NAME=backend


###############################################################################
# INSTALL PACKAGES FOR NPM
FROM node:latest AS build_node
WORKDIR /app

# COPY --from=build_rust /app/server /app/server
COPY package.json package-lock.json ./
RUN npm install


FROM rust:${RUST_VERSION}-slim-bullseye AS build_rust
ARG APP_NAME
WORKDIR /app

COPY --from=build_node /app/node_modules ./doc/node_modules

COPY . .

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies and a cache mount to /app/target/ for 
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=doc,target=doc \
    --mount=type=bind,source=tests,target=tests \
    --mount=type=bind,source=assets,target=assets \
    --mount=type=bind,source=.config,target=.config \
# askama
    --mount=type=bind,source=askama.toml,target=askama.toml \
# Cargo
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cargo test
cp ./target/release/$APP_NAME ./server
cp ./target/release/init_db ./init_db
cp ./target/release/test ./test
EOF

RUN cargo clean

###############################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
#
# The example below uses the debian bullseye image as the foundation for running the app.
# By specifying the "bullseye-slim" tag, it will also use whatever happens to be the
# most recent version of that tag when you build your Dockerfile. If
# reproducability is important, consider using a digest
# (e.g., debian@sha256:ac707220fbd7b67fc19b112cee8170b41a9e97f703f588b2cdbbcdcecdd8af57).
# FROM debian:bullseye-slim AS final

# # Create a non-privileged user that the app will run under.
# # See https://docs.docker.com/go/dockerfile-user-best-practices/

# # COPY --from=build_rust /app/. /app
# WORKDIR /app
# COPY --from=build_rust /app/ .


# RUN apt-get update && apt-get install -y iputils-ping


# FROM ubuntu/mysql:latest AS final
FROM ubuntu:latest AS final
# RUN apt-get update && apt-get install -y iputils-ping

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

USER appuser

# ENV MYSQL_ROOT_PASSWORD=1243
# ENV MYSQL_DATABASE=yuru
# ENV TZ=Asia/Shanghai

WORKDIR /app
COPY --from=build_rust /app/ .
# COPY ./docker_env/mysqld.cnf /etc/mysql/mysql.conf.d/mysqld.cnf

# # Copy the executable from the "build" stage.
# COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 3000
# RUN ./init_db

# What the container should run when it is started.
# CMD ["/app/server"]
# CMD ["/app/test"]
# CMD ["/app/init_db"]
