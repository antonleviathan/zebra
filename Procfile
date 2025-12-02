binary: /usr/local/bin/zebrad
build: ./docker/build.sh
run: export ZEBRA_HEALTH__LISTEN_ADDR=0.0.0.0:8080 ZEBRA_HEALTH__MIN_CONNECTED_PEERS=0 ZEBRA_HEALTH__READY_MAX_BLOCKS_BEHIND=100 ZEBRA_NETWORK__LISTEN_ADDR=0.0.0.0:8233 && /usr/local/bin/zebrad start
oci_tarball: build/oci/zebra.tar
source: https://github.com/antonleviathan/zebra/archive/${COMMIT}.tar.gz
memory_mb: 4096
cpus: 3
debug: true
ports: 8232, 18232, 8233, 18233
