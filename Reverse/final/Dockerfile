FROM ubuntu:18.04

RUN apt-get update && \
    apt-get install -y socat && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY challenge /
COPY run.sh /
COPY flag /

RUN chmod 444 /flag && \
    chmod 500 /run.sh && \
    chown -R root:root /challenge

EXPOSE 1337

ENTRYPOINT ["/run.sh"]
