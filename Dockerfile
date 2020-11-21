# It provides containerisation to CloudBanking proyect.
# Containersitation it's a way of build a machine 
# and human readable script (similar to makefile)
# for creating a virtual infraestructure of data processing.

# Our infrastructure will be the result of abstracting and packaging 
# the language compiler/interpreter, the application-dependent
# programs and the system libraries needed to run a workflow.


# From Layer
# Should be first, if not: failed to solve with frontend dockerfile.v0: failed to create LLB definition: No build stage in current context
# Select image base {alpine, fedora, centos}
# We use Alpine image because is a complete Linux
# In only 5 MB package
FROM alpine:3.12.1


LABEL pepitoenpeligro.CloudBanking.version="0.4.0"
LABEL pepitoenpeligro.CloudBanking.release-date="2020-11-09"
LABEL mantainer="https://github.com/pepitoenpeligro"
LABEL pepitoenpeligro.label-CloudBanking.usage="https://pepitoenpeligro.github.io/CloudBanking/"
LABEL pepitoenpeligro.CloudBanking.url="https://hub.docker.com/r/pepitoenpeligro/cloudbanking/"
LABEL pepitoenpeligro.CloudBanking.description="Docker Alpine with CloudBanking"

ENV alpine_repo=https://alpine.global.ssl.fastly.net/alpine/v3.12
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV CARGO_MAKE_HOME=/usr/local/cargo_make
ENV PATH=/usr/local/cargo/bin:$PATH
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
ENV OPENSSL_LIB_DIR=/usr/lib64

# Layer Run
# It makes possible to execute commands inside the Docker image
# Is like a shell script that contains the desired behaviour,
# similar a Shell Script


USER root


RUN mkdir -p /app/test
RUN chmod -R o+xwr /app
RUN chmod -R o+xwr /app/test
RUN addgroup -g 1000 -S cbgroup && adduser -u 1000 -S cbuser -G cbgroup -D -g ''   -h /app/test -s /bin/sh
RUN chown -R cbuser:cbgroup /app/test
RUN chown -R cbuser:cbgroup /app




RUN echo $alpine_repo:/main >> /etc/apk/repositories
RUN echo $alpine_repo:/community >> /etc/apk/repositories
RUN apk -U upgrade -a -q
RUN for i in openssl openssl-dev wget ca-certificates gcc musl-dev pkgconfig; do apk add -q --no-cache "$i" ; done






RUN wget https://static.rust-lang.org/rustup/archive/1.22.1/x86_64-unknown-linux-musl/rustup-init
RUN chmod +x rustup-init
RUN ./rustup-init -y --no-modify-path --profile minimal --default-toolchain 1.48.0 --default-host x86_64-unknown-linux-musl
RUN chmod -R a+w $RUSTUP_HOME $CARGO_HOME
RUN rm -f rustup-init

RUN wget https://github.com/sagiegurari/cargo-make/releases/download/0.32.9/cargo-make-v0.32.9-x86_64-unknown-linux-musl.zip
RUN unzip cargo-make-v0.32.9-x86_64-unknown-linux-musl.zip
RUN cp cargo-make-v0.32.9-x86_64-unknown-linux-musl/cargo-make /usr/local/cargo/bin
RUN rm -rf cargo-make-v0.32.9-x86_64-unknown-linux-musl*
USER cbuser
WORKDIR /app/test

# We don't need
# ENV $HOME/.cargo/bin


# Layer CMD
# It makes posible to run process inside Docker Container.
# It defines what happens when Docker container has started.
# CMD ["/home/cbuser/CloudBanking/target/release/CloudBanking"]
# CMD cargo test

# CMD cargo make --makefile make.toml test

CMD chown -R cbuser:cbgroup /app/test
CMD chown -R cbuser:cbgroup /app
CMD cargo make --makefile make.toml test