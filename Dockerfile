ARG BINARY_NAME_DEFAULT=crabhash
####################################################################################################
## Builder
####################################################################################################
FROM rust:latest 
COPY src ./src
COPY Cargo.toml .
RUN set -x && cargo build --release
RUN mkdir -p /build-out
RUN set -x ./target/release/crabhash
ENV PATH $PATH:/target/release/crabhash

# ####################################################################################################
# ## Final image
# ####################################################################################################

# # Create a minimal docker image 
# FROM alpine
# RUN apk update && apk upgrade && apk add bash
# COPY --from=0 /etc/passwd /etc/passwd
# USER dockeruser

# ARG BINARY_NAME_DEFAULT
# ENV BINARY_NAME=$BINARY_NAME_DEFAULT

# ENV RUST_LOG="error,$BINARY_NAME=info"
# COPY --from=builder /build-out/$BINARY_NAME /

# Start with an execution list (there is no sh in a scratch image)
# No shell => no variable expansion, |, <, >, etc 
# Hard coded start command
#CMD ["/crabhash"]
