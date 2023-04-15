FROM rust:latest

WORKDIR /code

RUN echo 'alias run_rust="cargo test -- -Z unstable-options --format json | tee $1"' > .bashrc

COPY . .

ENTRYPOINT ["/bin/bash"]
