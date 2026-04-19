# Stage 1: Build
FROM rust:1.94-slim AS builder

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
ARG BACKEND_URL
ENV BACKEND_URL=$BACKEND_URL

# Build the frontend for web
RUN dx build --release

# Stage 2: Serve
FROM nginx:alpine

# Copy the built assets to nginx
COPY --from=builder /app/target/dx/rhexiom-frontend/release/web/public /usr/share/nginx/html

# Custom Nginx config to handle SPA routing (redirect all to index.html)
RUN echo 'server { \
    listen 80; \
    location / { \
        root /usr/share/nginx/html; \
        index index.html; \
        try_files $uri $uri/ /index.html; \
    } \
}' > /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
