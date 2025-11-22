# Caution Build Configuration

# Container file to build (Dockerfile or Containerfile)
containerfile: docker/Dockerfile

build: ./docker/build.sh

# OCI tarball location (if build script exports to OCI instead of loading into Docker)
oci_tarball: build/oci/zebra.tar

# Path to binary after build (will be extracted from the final layer)
binary: /usr/local/bin/zebrad

# Run command for the application
run: /usr/local/bin/zebrad

# Resource requirements for the enclave
# CPU count must be full cores (1, 2, 3, etc.), leaving at least 1 for parent
cpus: 3
# Memory in MB (MiB will be calculated)
memory_mb: 4096

# Enable debug mode to see enclave console output
debug: false
