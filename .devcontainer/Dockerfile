FROM ubuntu:20.04

RUN apt-get update && apt-get install -y \
    vim \
    curl \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Rust install
ENV RUST_HOME /usr/local/lib/rust
ENV RUSTUP_HOME ${RUST_HOME}/rustup
ENV CARGO_HOME ${RUST_HOME}/cargo
RUN mkdir /usr/local/lib/rust && \
    chmod 0755 $RUST_HOME
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > ${RUST_HOME}/rustup.sh \
    && chmod +x ${RUST_HOME}/rustup.sh \
    && ${RUST_HOME}/rustup.sh -y  --no-modify-path
ENV PATH $PATH:$CARGO_HOME/bin
RUN ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
RUN echo Asia/Tokyo > /etc/timezone
RUN apt-get update && apt-get install git lldb build-essential libssl-dev pkg-config -y
WORKDIR /workspaces/atcoder-rust/
RUN rustup install 1.42.0
RUN rustup update
COPY pass.txt /tmp/
RUN cargo install cargo-compete
RUN cat /tmp/pass.txt | cargo compete login atcoder

CMD ["sleep","infinity"]