FROM debian:sid

COPY sources.list /etc/apt/sources.list

ARG http_proxy
RUN env http_proxy=${http_proxy} apt-get update

WORKDIR /mnt

