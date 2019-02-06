FROM rust:1.32

WORKDIR /usr/src/app
COPY . .

RUN cd data/typed-html
RUN cargo build --release
RUN cd ../../

RUN cargo build --release

EXPOSE 3001

CMD cargo run --release
