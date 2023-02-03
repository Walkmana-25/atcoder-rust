FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye
RUN cargo install cargo-compete
WORKDIR /workspaces/atcoder-rust/
RUN rustup install 1.42.0
COPY pass.txt /tmp/
RUN cat /tmp/pass.txt | cargo compete login atcoder

CMD ["sleep","infinity"]