#!/bin/bash

set -e

HAPROXY_CONFIG="/usr/local/etc/haproxy/haproxy.cfg"
PIDFILE="/run/haproxy/haproxy.pid"

if [[ -n "$BACKEND_HOST" ]]; then
	BACKEND_PORT=${BACKEND_PORT:-80}
	if grep -q "server dynamic *" "$HAPROXY_CONFIG"; then
		echo "updating backend $BACKEND_HOST:$BACKEND_PORT"
		sed -i "s/server dynamic .*/server dynamic $BACKEND_HOST:$BACKEND_PORT check/" "$HAPROXY_CONFIG"
	fi
	if ! grep -q "$BACKEND_HOST:$BACKEND_PORT" "$HAPROXY_CONFIG"; then
		echo "adding backend $BACKEND_HOST:$BACKEND_PORT"
		echo "    server dynamic $BACKEND_HOST:$BACKEND_PORT check" | tee -a "$HAPROXY_CONFIG"
	fi
else
	echo "BACKEND_HOST or BACKEND_PORT is not set"
fi

update_config() {
	local index=$1
	local host=$2
	local port=$3
	local name="dynamic-$index"

	if grep -q "server $name *" "$HAPROXY_CONFIG"; then
		echo "updating backend $host:$port"
		sed -i "s/server $name .*/server $name $host:$port check/" "$HAPROXY_CONFIG"
	else
		echo "adding backend $host:$port"
		echo "    server $name $host:$port check" | tee -a "$HAPROXY_CONFIG"
	fi

	echo "testing config"
	haproxy -c -V -f $HAPROXY_CONFIG
}

env | grep -Eo "BACKEND_[0-9]+_HOST" | while read -r line; do
	index=$(echo "$line" | cut -d'_' -f2)
	var_host="BACKEND_${index}_HOST"
	var_port="BACKEND_${index}_PORT"
	host=${!var_host}
	port=${!var_port:-80}
	if [[ -n "$host" ]]; then
		update_config "$index" "$host" "$port"
	fi
done

if [[ -f "$PIDFILE" ]]; then
	echo "reloading"
	haproxy -f "$HAPROXY_CONFIG" -p "$PIDFILE" -D -sf "$(cat "$PIDFILE")"
fi

exec "$@"
