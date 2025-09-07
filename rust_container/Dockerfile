FROM rust:1.67

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["asl_rust_app"]

# docker build -t asl-rust-app . (BUILD CONTAINER)
# docker run --rm asl-rust-app  (RUN CONTAINER)