FROM rust:1.86.0

# Install Chromium and dependencies
RUN apt-get update && apt-get install -y \
    chromium \
    libxss1 \
    libasound2 \
    libnss3 \
    libgdk-pixbuf2.0-0 \
    libx11-xcb1 \
    fonts-liberation \
    && rm -rf /var/lib/apt/lists/*

# Set Chromium binary path for headless_chrome
ENV CHROME_BIN=/usr/bin/chromium

# Set working directory
WORKDIR /app

# Copy project files and build
COPY . .

RUN cargo build --release
RUN cargo fix --bin rust_web_scraper

# Run the binary
CMD ["r./target/release/rust_web_scraper.exe"]
