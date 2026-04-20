# Stage 1: Build
FROM rust:1.81-slim AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Dioxus CLI
RUN cargo install dioxus-cli --version 0.6.3 --locked

# Setup WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

# Argument for the backend URL injected at build time
ARG BACKEND_URL="https://rh-back-production-c7cf.up.railway.app/api"
ENV BACKEND_URL=$BACKEND_URL

# Build the frontend for web
RUN dx build --release

# Stage 2: Serve
FROM nginx:alpine

# Copy the built assets to nginx
COPY --from=builder /app/target/dx/rhexiom-frontend/release/web/public /usr/share/nginx/html

# Custom Nginx config template to handle SPA routing (dynamic port will be injected at runtime)
RUN echo 'server { \
    listen ${PORT}; \
    location / { \
        root /usr/share/nginx/html; \
        index index.html; \
        try_files $uri $uri/ /index.html; \
    } \
}' > /etc/nginx/conf.d/default.conf.template

# Use sh to substitute the port at runtime before starting nginx
CMD ["/bin/sh", "-c", "envsubst '${PORT}' < /etc/nginx/conf.d/default.conf.template > /etc/nginx/conf.d/default.conf && nginx -g 'daemon off;'"]
