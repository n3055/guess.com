FROM rust:1.95

# Install nginx
RUN apt-get update && apt-get install -y nginx && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

RUN cargo build --release

# Copy custom nginx configuration
COPY nginx.conf /etc/nginx/sites-available/default

# Expose the default port
EXPOSE 8080

CMD ["./start.sh"]
