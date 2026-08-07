#!/bin/bash

# Wait a bit for NetworkManager to attempt connections
sleep 15

# Check if we are connected to a wifi network
# if ! nmcli -t -f STATE,TYPE connection show --active | grep -q "wifi"; then
#    echo "No active WiFi connection found. Initializing AP Fallback mode..."
    
    # Get last 4 characters of MAC address for uniqueness
    MAC=$(cat /sys/class/net/wlan0/address | sed 's/://g' | tail -c 5)
    SSID="Picar-Vision-$MAC"
    PASSWORD="picar-vision"

    # Remove existing hotspot if any
    nmcli connection show hotspot >/dev/null 2>&1 && nmcli connection delete hotspot
    
    # Create and start the hotspot
    echo "Starting hotspot with SSID: $SSID"
    nmcli device wifi hotspot ifname wlan0 ssid "$SSID" password "$PASSWORD" connection.id hotspot
# else
#    echo "Active WiFi connection found. Skipping AP Fallback."
# fi
