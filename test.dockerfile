FROM rust:slim
MAINTAINER Ryan Faulhaber <ryf@sent.as>

COPY . /app
WORKDIR /app

RUN cargo build --tests

ENV RUST_BACKTRACE=1

ENTRYPOINT ["cargo", "test", "--test"]
CMD ["integration"]
