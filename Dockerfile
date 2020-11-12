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


# Layer Run
# It makes possible to execute commands inside the Docker image
# Is like a shell script that contains the desired behaviour,
# similar a Shell Script



RUN echo $alpine_repo:/main >> /etc/apk/repositories
RUN echo $alpine_repo:/community >> /etc/apk/repositories
RUN apk -U upgrade -a 


RUN for i in openssl-dev gcc musl-dev rust cargo; do apk add "$i"; done

USER root
RUN addgroup -g 1000 -S cbgroup && adduser -u 1000 -S cbuser -G cbgroup -D -g ''   -h /home/cbuser -s /sbin/nologin
RUN chown -Rv cbuser:cbgroup /home/cbuser/
WORKDIR /home/cbuser


# Layer Copy
# It makes possible to copy files inside Docker image.
# It's similar to ADD layer, but more secure.
# Syntax COPY origin destination
COPY . /home/cbuser/CloudBanking
RUN chown -Rv cbuser:cbgroup /home/cbuser/
# RUN cargo install --force cargo-make





# Layer Expose
# Expose. It makes a port available to the outside
# Under this port, CloudBankingApp will be available, and 
# container will provide only this port on the internal 
# Docker network. Access from outside will be only possible
# if Expose is defined correctly.
# We should indicate the port to redirect when we build it
# docker run -p INTERNAL_PORT_EXPOSED:DESIRED_PORT CloudBanking
EXPOSE 8001



USER cbuser
WORKDIR /home/cbuser/CloudBanking
# RUN cargo build --release --bin CloudBanking


# Layer CMD
# It makes posible to run process inside Docker Container.
# It defines what happens when Docker container has started.
# CMD ["/home/cbuser/CloudBanking/target/release/CloudBanking"]
CMD /bin/sh