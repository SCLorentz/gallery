FROM debian:bookworm-slim

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    RUST_VERSION=1.83.0

COPY cmd/ /root/

RUN rm -rf ~/.bashrc \
    && mv ~/.bash ~/.bashrc

RUN apt-get update && apt-get install -f -y --no-install-recommends wget xz-utils ca-certificates libssl-dev net-tools git build-essential gcc

RUN BIN="/usr/local/bin" \
    && UPX="${BIN}/upx-4.2.4-arm64_linux.tar.xz" \
    && wget -O ${UPX} --no-check-certificate https://github.com/upx/upx/releases/download/v4.2.4/upx-4.2.4-arm64_linux.tar.xz \
    && tar xf ${UPX} -C ${BIN}

RUN apt-get purge -y wget xz-utils \
    && apt-get autoremove -y \
    && rm -rf /var/lib/apt/lists/* ${UPX}

RUN chmod +x ~/rustup-init \
    && ~/rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host aarch64-unknown-linux-gnu \
    && rm -rf ~/rustup-init \
    && chmod -R a+w $RUSTUP_HOME $CARGO_HOME

ENV FLASK_APP=zagware
EXPOSE 8000

CMD ["/bin/bash", "-l", "-c", "flask run --host 0.0.0.0 --port 8000"]