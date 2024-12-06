FROM ubuntu:22.04
COPY ./target/release/my-settings-ui ./target/release/my-settings-ui
COPY ./dist ./dist
RUN chmod +x /target/release/my-settings-ui
WORKDIR /target/release/
ENTRYPOINT ["./my-settings-ui"]