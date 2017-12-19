FROM rust:latest

RUN apt-get update && apt-get install -y \
  sqlite3

ADD db/carolus.db /
ADD target/release/carolus /

RUN chmod +x ./carolus

EXPOSE 8000

ENTRYPOINT ["./carolus"]
