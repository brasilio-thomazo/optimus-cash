#!/bin/bash
set -e
cat /usr/local/etc/haproxy/haproxy.cfg.template | envsubst >/usr/local/etc/haproxy/haproxy.cfg
IFS="," read -r -a services <<<"$SERVICES"
for service in "${services[@]}"; do
	echo "server $service $service:4000 check" >>/usr/local/etc/haproxy/haproxy.cfg
	ping -c 2 "$service"
done
cat /usr/local/etc/haproxy/haproxy.cfg

haproxy -c -V -f /usr/local/etc/haproxy/haproxy.cfg

exec "$@"
