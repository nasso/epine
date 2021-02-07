FROM epitechcontent/epitest-docker

RUN mkdir /app && git clone https://github.com/Arcahub/Test.git /app

COPY . /epine/
RUN cd /epine && cargo build --release && cp ./target/release/epine /app

WORKDIR /app
CMD ./epine && make
