FROM ubuntu:22.04

COPY ./target/release/my-settings-ui ./target/release/my-settings-ui
COPY ./dist ./dist
ENTRYPOINT ["./target/release/my-settings-ui"]