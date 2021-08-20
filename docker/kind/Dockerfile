FROM docker.io/logdna/build-images:rust-bullseye-stable
STOPSIGNAL SIGRTMIN+3

RUN mkdir -p /var/cache/sccache

WORKDIR /work/
COPY . .

CMD [ "echo", "Starting..." ]
