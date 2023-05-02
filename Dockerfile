FROM rust:latest

WORKDIR /code

COPY . .

ENTRYPOINT ["/bin/bash"]
