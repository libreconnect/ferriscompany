# ARG RUST_VERSION=1.81.0
# ARG APP_NAME=ferriscompany_server

# ################################################################################
# # Create a stage for building the application.

# FROM rust:${RUST_VERSION}-alpine AS build
# ARG APP_NAME
# WORKDIR /app

# # Install host build dependencies.
# RUN apk add --no-cache clang lld musl-dev git

# COPY Cargo.toml Cargo.lock ./

# RUN mkdir src

# RUN --mount=type=cache,target=/usr/local/cargo/registry \
#     --mount=type=cache,target=/usr/local/cargo/git/db \
#     cargo build --release || true

# COPY ./src ./src

# RUN --mount=type=cache,target=/usr/local/cargo/registry \
#     --mount=type=cache,target=/usr/local/cargo/git/db \
#     cargo build --release && \
#     cp ./target/release/${APP_NAME} /bin/server

# ################################################################################
# # Create a new stage for running the application that contains the minimal
# # runtime dependencies for the application. This often uses a different base
# # image from the build stage where the necessary files are copied from the build
# # stage.
# #
# # The example below uses the alpine image as the foundation for running the app.
# # By specifying the "3.18" tag, it will use version 3.18 of alpine. If
# # reproducability is important, consider using a digest
# # (e.g., alpine@sha256:664888ac9cfd28068e062c991ebcff4b4c7307dc8dd4df9e728bedde5c449d91).
# FROM alpine:3.18 AS final

# # Create a non-privileged user that the app will run under.
# # See https://docs.docker.com/go/dockerfile-user-best-practices/
# ARG UID=10001
# RUN adduser \
#     --disabled-password \
#     --gecos "" \
#     --home "/nonexistent" \
#     --shell "/sbin/nologin" \
#     --no-create-home \
#     --uid "${UID}" \
#     appuser
# USER appuser

# # Copy the executable from the "build" stage.
# COPY --from=build /bin/server /bin/

# # Expose the port that the application listens on.
# EXPOSE 3333

# # What the container should run when it is started.
# CMD ["/bin/server"]


FROM rust AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . . 
RUN cargo chef prepare --recipe-path recipe.json

FROM rust AS cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust AS builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin ferriscompany_server

FROM rust AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/ferriscompany_server /usr/local/bin

CMD [ "/usr/local/bin/ferriscompany_server" ]
