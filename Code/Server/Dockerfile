FROM arm64v8/python:3.11-slim

WORKDIR /app

# Install system dependencies for hardware I/O, OpenCV, and camera
RUN apt-get update && apt-get install -y \
    build-essential \
    python3-dev \
    i2c-tools \
    libglib2.0-0 \
    libsm6 \
    libxext6 \
    libxrender-dev \
    libcamera-dev \
    && rm -rf /var/lib/apt/lists/*

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

# Run the websocket gateway server
CMD ["python3", "gateway.py"]
