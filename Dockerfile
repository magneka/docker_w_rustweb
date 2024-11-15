# FIRST PART - COMPILE BINARIES
# -----------------------------
# Use a Rust base image with Cargo installed
FROM rust:1.82.0 AS rustbuilder2

# Set the working directory inside the container
WORKDIR /usr/src

# Copy the rust project
COPY ./source/rouille/. .

# Build the dependencies without the actual source code to cache dependencies separately
RUN cargo build --release

# SECOND PART - CREATE RUN ENVIRONMENT
# ------------------------------------
# Start a new stage to create a smaller image without unnecessary build dependencies
# genererer 76 GB image
#FROM debian:stable-slim
#FROM bitnami/minideb:latest
# genererer 24 GB image
#FROM gcr.io/distroless/cc
# genererer 22 GB image
##FROM gcr.io/distroless/static fungerer ikke
FROM gcr.io/distroless/cc-debian12

# Set the working directory
WORKDIR /usr/src/app

# Copy the built binary from the previous stage
COPY --from=rustbuilder2 /usr/src/target/release/rouilleserver /usr/src/app/.

EXPOSE 80

# Command to run the application
CMD ["./rouilleserver"]