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

    # Create a virtual AP interface if it doesn't exist
    if ! iw dev | grep -q "uap0"; then
        iw dev wlan0 interface add uap0 type __ap
    fi
    ip link set uap0 up

    # Remove existing hotspot if any
    nmcli connection show "Hotspot" >/dev/null 2>&1 && nmcli connection delete "Hotspot"
    nmcli connection show "$SSID" >/dev/null 2>&1 && nmcli connection delete "$SSID"
    
    # Create and start the hotspot robustly on uap0
    echo "Starting hotspot with SSID: $SSID on uap0"
    nmcli con add type wifi ifname uap0 con-name "$SSID" autoconnect no ssid "$SSID"
    nmcli con modify "$SSID" 802-11-wireless.mode ap 802-11-wireless.band bg ipv4.method shared
    nmcli con modify "$SSID" wifi-sec.key-mgmt wpa-psk
    nmcli con modify "$SSID" wifi-sec.psk "$PASSWORD"
    nmcli con up "$SSID"
# else
#    echo "Active WiFi connection found. Skipping AP Fallback."
# fi
