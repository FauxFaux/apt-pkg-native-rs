FROM ubuntu:trusty

ARG http_proxy
RUN env http_proxy=${http_proxy} apt-get update \
    && apt-get install -y build-essential libapt-pkg-dev curl

COPY uprust.sh /root/uprust.sh

RUN bash /root/uprust.sh --default-toolchain nightly -y

WORKDIR /mnt
