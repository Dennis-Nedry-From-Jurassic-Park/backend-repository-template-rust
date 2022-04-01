FROM rust:1.59.0 as build

WORKDIR entrypoint

COPY ./entrypoint ./entrypoint
COPY ./module1 ./module1
COPY ./module2 ./module2
COPY ./Cargo.lock Cargo.lock
COPY ./Cargo.toml Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release --bin entrypoint
RUN rm ./entrypoint/*.*
RUN rm ./module1/*.*
RUN rm ./module2/*.*

COPY ./entrypoint ./entrypoint
COPY ./module1 ./module1
COPY ./module2 ./module2
COPY ./Cargo.lock Cargo.lock
COPY ./Cargo.toml Cargo.toml

RUN rm ./target/*.*
RUN cargo build --release

FROM rust:1.59.0-slim-buster as runtime

COPY --from=build /entrypoint/target/release/entrypoint .

#RUN cargo install --path .

CMD ["./entrypoint"]
# https://github.com/denzp/cargo-wharf
# https://github.com/moby/buildkit#exploring-dockerfiles