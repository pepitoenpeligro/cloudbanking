# It provides containerisation to CloudBanking proyect.
# Containersitation it's a way of build a machine 
# and human readable script (similar to makefile)
# for creating a virtual infraestructure of data processing.

# Our infrastructure will be the result of abstracting and packaging 
# the language compiler/interpreter, the application-dependent
# programs and the system libraries needed to run a workflow.


# From Layer
FROM rust:1.47.0


LABEL pepitoenpeligro.CloudBanking.version="0.6.0"
LABEL pepitoenpeligro.CloudBanking.release-date="2021-01-09"
LABEL mantainer="https://github.com/pepitoenpeligro"
LABEL pepitoenpeligro.label-CloudBanking.usage="https://github.com/pepitoenpeligro/cloudbanking"
LABEL pepitoenpeligro.CloudBanking.url="https://hub.docker.com/r/pepitoenpeligro/cloudbanking/"
LABEL pepitoenpeligro.CloudBanking.description="Docker Alpine with CloudBanking"
LABEL org.opencontainers.image.source https://github.com/pepitoenpeligro/cloudbanking


RUN mkdir -p /app/test \
    && chmod -R o+xwr /app \
    && chmod -R o+xwr /app/test \
    && cargo install --force cargo-make

# Changing our work directory
WORKDIR /app/test


ENTRYPOINT ["cargo make --makefile make.toml test"]

# docker build  --no-cache -t pepitoenpeligro/cloudbanking .
# docker run -it -v `pwd`:/app/test -t pepitoenpeligro/cloudbanking
# docker run -t -v `pwd`:/app/test -t pepitoenpeligro/cloudbanking