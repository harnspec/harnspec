# Stage 1: pull the platform binary and UI dist from npm
FROM node:20-slim AS builder
ARG NPM_TAG=latest
ARG PLATFORM_PKG=http-linux-x64
RUN npm install -g @harnspec/${PLATFORM_PKG}@${NPM_TAG} @harnspec/ui@${NPM_TAG}

# Stage 2: minimal runtime — no Node, just the Rust binary + static files
FROM debian:bookworm-slim

ARG PLATFORM_PKG=http-linux-x64

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/lib/node_modules/@harnspec/${PLATFORM_PKG}/harnspec-http /usr/local/bin/harnspec-http
COPY --from=builder /usr/local/lib/node_modules/@harnspec/ui/dist /app/ui-dist

# Non-root user
RUN groupadd -r harnspec && useradd -r -g harnspec -m -d /home/harnspec harnspec

ENV harnspec_UI_DIST=/app/ui-dist
ENV HOME=/home/harnspec
ENV harnspec_LOG_FORMAT=json

EXPOSE 3000

USER harnspec

# --no-open: skip browser launch (not available in containers)
# --host 0.0.0.0: bind all interfaces so Docker port mapping works
ENTRYPOINT ["harnspec-http", "--no-open", "--host", "0.0.0.0"]
