FROM ubuntu:22.04

ENV PORT=9001
ENV IP=0.0.0.0

EXPOSE 9001

COPY ./target/dx/my-settings-ui/release/web /target/dx/my-settings-ui/release/web
RUN chmod +x /target/dx/my-settings-ui/release/web/server
WORKDIR /target/dx/my-settings-ui/release/web/
ENTRYPOINT ["./server" ]