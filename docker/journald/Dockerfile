FROM docker.io/logdna/build-images:rust-bullseye-stable
STOPSIGNAL SIGRTMIN+3

RUN mkdir -p /var/log/journal

WORKDIR /work/
COPY . .

RUN chmod 777 /etc
RUN mkdir /etc/logdna
RUN chmod 777 /etc/logdna

CMD [ "/bin/systemd", "--system", "--unit=basic.target" ]
