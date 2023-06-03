FROM rust:1.67 AS build

WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/rustup \
    set -eux; \
    rustup install stable; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/emacs-config-generator ./emacs-config-generator

FROM debian:11.3-slim

RUN set -eux; \
		export DEBIAN_FRONTEND=noninteractive; \
	  apt update; \
		apt install --yes --no-install-recommends bind9-dnsutils iputils-ping iproute2 curl ca-certificates htop; \
		apt clean autoclean; \
		apt autoremove --yes; \
		rm -rf /var/lib/{apt,dpkg,cache,log}/; \
		echo "Installed base utils!"

WORKDIR app

COPY static/ ./static/
COPY templates/ ./templates/
COPY --from=build /app/emacs-config-generator ./emacs-config-generator

CMD ["./emacs-config-generator"]
