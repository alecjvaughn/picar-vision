FROM arm64v8/debian:bookworm-slim

WORKDIR /app

# Install system dependencies and pre-compiled Python packages via apt-get
# This completely eliminates long build times for OpenCV and Numpy
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    python3-opencv \
    python3-numpy \
    python3-smbus \
    python3-spidev \
    python3-gpiozero \
    python3-websockets \
    python3-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install remaining pure python or specific hardware packages
# --break-system-packages is required on Debian Bookworm (Python 3.11) when not using venv
COPY requirements.txt .
RUN pip install --no-cache-dir --break-system-packages -r requirements.txt

COPY . .

# Run the websocket gateway server
CMD ["python3", "gateway.py"]
