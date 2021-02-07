FROM ubuntu

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y git && \
    apt-get install -y curl && \
    apt-get install -y build-essential && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    mkdir /app && git clone https://github.com/Arcahub/Test.git /app

COPY . /epine/
RUN cd /epine && ~/.cargo/bin/cargo build --release && cp ./target/release/epine /app

WORKDIR /app
CMD ./epine && make
