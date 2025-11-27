# Custom build process (uses oci_tarball)
build: ./docker/build.sh
oci_tarball: build/oci/zebra.tar

# Required
binary: /usr/local/bin/zebrad
run: /usr/local/bin/zebrad

# For reproducible verification
source: https://github.com/antonleviathan/zebra/archive/deterministic-bootstrapped-build.tar.gz

# Resource requirements (defaults are 2 CPUs, 512MB)
cpus: 3
memory_mb: 4096
