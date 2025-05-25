ARG BASE_VERSION
FROM buildpack-deps:${BASE_VERSION}

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt-get update && \
    apt-get install -y libapt-pkg-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

ENV CARGO_TERM_COLOR=always
WORKDIR /src
ADD . .
