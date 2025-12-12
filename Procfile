build: ./docker/build.sh
run: /usr/local/bin/zebrad -c /home/zebra/.config/zebrad.toml start
oci_tarball: build/oci/zebra.tar
source: https://github.com/antonleviathan/zebra/archive/${COMMIT}.tar.gz
memory_mb: 7800
cpus: 4
debug: true
ports: 8232, 8233
nocache: true
