FROM rust:1-stretch as builder
# Choose a workdir
WORKDIR /usr/src/app

# This is is you want to compile all rust docs
#RUN cargo build --release

# This is is you want to run all tests
#RUN cargo test

#FROM debian:stretch-slim
# Copy bin from builder to this new image
#COPY --from=builder /usr/src/app/target/release/rust-build /bin/
# Default command, run app
#CMD rust_build

RUN apt-get update && \
    apt-get install -y graphviz && \
    apt-get install -y nano
	
WORKDIR /source
