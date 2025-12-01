# Caution Procfile
# To learn more visit https://git.distrust.co/public/caution

# ==============================================================================
# REQUIRED
# ==============================================================================

# Path to the binary to extract from the container and run
binary: /usr/local/bin/zebrad

# ==============================================================================
# OPTIONAL - Build Configuration
# ==============================================================================

# Build command (default: docker build -t app .)
build: ./docker/build.sh
oci_tarball: build/oci/zebra.tar

# Dockerfile location (default: Dockerfile)
# containerfile: Dockerfile

# For custom build scripts that produce OCI tarballs:
# build: ./docker/build.sh
# oci_tarball: build/oci/myapp.tar

# ==============================================================================
# OPTIONAL - Runtime Configuration
# ==============================================================================

# Run command (default: same as binary)
# Use this to pass arguments, e.g.: /app/myapp serve --config /etc/config.toml
run: export ZEBRA_RPC__LISTEN_ADDR=0.0.0.0:8232 && /usr/local/bin/zebrad start

# ==============================================================================
# OPTIONAL - Reproducibility
# ==============================================================================

# Source URL for reproducible verification (${COMMIT} is replaced with commit SHA)
source: https://github.com/antonleviathan/zebra/archive/${COMMIT}.tar.gz

# Metadata string embedded in the manifest (for versioning, etc.)
# metadata: v1.0.0

# ==============================================================================
# OPTIONAL - Resources
# ==============================================================================

# Memory allocation in MB (default: 512)
memory_mb: 1024

# CPU count (default: 2)
cpus: 3

# Debug mode - enables console output (default: false, reduces security)
debug: true

ports: 8232, 18232, 8233, 18233
nocache: true
