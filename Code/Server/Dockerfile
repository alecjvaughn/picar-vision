FROM arm64v8/debian:bookworm-slim

WORKDIR /app
ENV PYTHONUNBUFFERED=1

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl gnupg \
    && curl -fsSL https://archive.raspberrypi.org/debian/raspberrypi.gpg.key | gpg --dearmor -o /usr/share/keyrings/raspberrypi-archive-keyring.gpg \
    && echo "deb [signed-by=/usr/share/keyrings/raspberrypi-archive-keyring.gpg] http://archive.raspberrypi.org/debian/ bookworm main" > /etc/apt/sources.list.d/raspi.list \
    && rm -rf /var/lib/apt/lists/*

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
    python3-lgpio \
    python3-rpi-lgpio \
    libcamera-tools \
    gstreamer1.0-libcamera \
    python3-picamera2 \
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
