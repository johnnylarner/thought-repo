FROM busybox:1.37.0-musl AS shell

FROM ghcr.io/getzola/zola:v0.22.0 AS builder
COPY --from=shell /bin/busybox /bin/sh
WORKDIR /site
COPY . .
ARG BASE_URL=/
RUN /bin/zola build --base-url "$BASE_URL"

FROM nginx:1.28-alpine
COPY --from=builder /site/public /usr/share/nginx/html
RUN chmod -R a+rX /usr/share/nginx/html
EXPOSE 80
