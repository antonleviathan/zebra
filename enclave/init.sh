#!/bin/sh

echo "=== Caution Enclave Startup ==="

echo "Setting up network loopback..."
/bin/busybox ip addr add 127.0.0.1/8 dev lo
/bin/busybox ip link set dev lo up
/bin/busybox ip link show lo

echo "127.0.0.1   localhost" > /etc/hosts

echo "nameserver 127.0.0.1" > /etc/resolv.conf

echo "Network loopback configured"

echo "Loading NSM kernel module..."
if [ -f /nsm.ko ]; then
    insmod /nsm.ko && echo "NSM module loaded successfully" || echo "Failed to load NSM module"
else
    echo "WARNING: NSM module not found at /nsm.ko"
fi

export SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

echo "Starting Attestation Service on port 5000..."
/attestation-service &

echo "Starting VSOCK-to-TCP proxies..."
/bin/socat VSOCK-LISTEN:5000,reuseaddr,fork TCP:localhost:5000 &
/bin/socat VSOCK-LISTEN:8080,reuseaddr,fork TCP:localhost:8080 &

/bin/busybox sleep 2

echo "Looking for user application in /app..."
cd /app || exit 1

echo "Contents of /app:"
/bin/busybox ls -la /app || echo "Could not list /app"

for exe in $(/bin/busybox find /app -type f -executable 2>/dev/null); do
    echo "Executing: $exe"
    exec "$exe"
done

echo "ERROR: No executable found in /app"
exit 1
