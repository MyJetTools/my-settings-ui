FROM myjettools/dioxus-docker:0.7.3

ENV PORT=9001
ENV IP=0.0.0.0

EXPOSE 9001

COPY ./target/dx/my-settings-ui/release/web /target/dx/my-settings-ui/release/web
RUN chmod +x /target/dx/my-settings-ui/release/web/my-settings-ui
WORKDIR /target/dx/my-settings-ui/release/web/
ENTRYPOINT ["./my-settings-ui" ]