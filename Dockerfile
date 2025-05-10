FROM rust:1.75

# Install Chromium and dependencies
RUN apt-get update && apt-get install -y \
    chromium-browser \
    libxss1 \
    libasound2 \
    libnss3 \
    libgdk-pixbuf2.0-0 \
    libx11-xcb1 \
    fonts-liberation \
    && rm -rf /var/lib/apt/lists/*

# Set environment variable so headless_chrome can find Chromium
ENV CHROME_BIN=/usr/bin/chromium-browser

# Create app directory
WORKDIR /app

# Copy source and build
COPY . .

RUN cargo build --release

# Run the binary
CMD ["r./target/release/rust_web_scraper.exe"]
