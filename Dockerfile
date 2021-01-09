# It provides containerisation to CloudBanking proyect.
# Containersitation it's a way of build a machine 
# and human readable script (similar to makefile)
# for creating a virtual infraestructure of data processing.

# Our infrastructure will be the result of abstracting and packaging 
# the language compiler/interpreter, the application-dependent
# programs and the system libraries needed to run a workflow.


# From Layer
# Should be first, if not: failed to solve with frontend dockerfile.v0: failed to create LLB definition: No build stage in current context
# Select image base {alpine, fedora, centos}
# We use Alpine image because is a complete Linux
# In only 5 MB package
FROM alpine:3.12.1


LABEL pepitoenpeligro.CloudBanking.version="0.4.1"
LABEL pepitoenpeligro.CloudBanking.release-date="2020-11-09"
LABEL mantainer="https://github.com/pepitoenpeligro"
LABEL pepitoenpeligro.label-CloudBanking.usage="https://github.com/pepitoenpeligro/cloudbanking"
LABEL pepitoenpeligro.CloudBanking.url="https://hub.docker.com/r/pepitoenpeligro/cloudbanking/"
LABEL pepitoenpeligro.CloudBanking.description="Docker Alpine with CloudBanking"
LABEL org.opencontainers.image.source https://github.com/pepitoenpeligro/cloudbanking

# Repo for wget, openssl and openssl-dev
ENV alpine_repo=https://alpine.global.ssl.fastly.net/alpine/v3.12

# Environment Variables to indicate a global installation of rustup, and cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo

# We made accesible all cargo binaries (like cargo, cargo-doc, etc)
ENV PATH=/usr/local/cargo/bin:$PATH

# Environment Variables to indicate a global installation and configuration of OpenSSL
# Necessary for our project due to warp, parking_lot and tokio {Cargo.toml} and for cargo-make binary
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
ENV OPENSSL_LIB_DIR=/usr/lib


# Explicit use of root user don't needed
#USER root

# Layer Run
# It makes possible to execute commands inside the Docker image
# Is like a shell script that contains the desired behaviour,
# similar a Shell Script
RUN mkdir -p /app/test \
    && chmod -R o+xwr /app \
    && chmod -R o+xwr /app/test \
    && addgroup -g 1000 -S cbgroup  \ 
    && adduser -u 1000 -S cbuser -G cbgroup -D -g ''   -h /app/test -s /bin/sh \
    && chown -R cbuser:cbgroup /app/test \
    && echo $alpine_repo:/main >> /etc/apk/repositories \
    && echo $alpine_repo:/community >> /etc/apk/repositories \
    && apk -U upgrade -a -q \
    && for i in openssl openssl-dev wget ca-certificates gcc musl-dev pkgconfig; do apk add -q --no-cache "$i" ; done \
    && wget https://static.rust-lang.org/rustup/archive/1.22.1/x86_64-unknown-linux-musl/rustup-init \
    && chmod +x rustup-init \
    && ./rustup-init -y --no-modify-path --profile minimal --default-toolchain 1.48.0 --default-host x86_64-unknown-linux-musl \
    && chmod -R a+w $RUSTUP_HOME $CARGO_HOME \
    && rm -f rustup-init \
    && wget https://github.com/sagiegurari/cargo-make/releases/download/0.32.9/cargo-make-v0.32.9-x86_64-unknown-linux-musl.zip \
    && unzip cargo-make-v0.32.9-x86_64-unknown-linux-musl.zip \
    && cp cargo-make-v0.32.9-x86_64-unknown-linux-musl/cargo-make /usr/local/cargo/bin \
    && rm -rf cargo-make-v0.32.9-x86_64-unknown-linux-musl* \
    && apk del wget

# Changing our work directory
WORKDIR /app/test

# Layer CMD
# It makes posible to run process inside Docker Container.
# It defines what happens when Docker container has started.

# 1. (As root) we change ownership of /ap/test to cbuser
# 2. (As root) we change user to cbuser
# 3. (As cbuser - non superprivileges) we run our test target (with task runner)
CMD chown -R cbuser:cbgroup /app/test && su cbuser -c 'cargo make --makefile make.toml test'


# docker build  --no-cache -t pepitoenpeligro/cloudbanking .
# docker run -it -v `pwd`:/app/test -t pepitoenpeligro/cloudbanking
# docker run -t -v `pwd`:/app/test -t pepitoenpeligro/cloudbanking