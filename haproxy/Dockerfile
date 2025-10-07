FROM haproxy:alpine
COPY ./haproxy.cfg /usr/local/etc/haproxy/haproxy.cfg
COPY ./entrypoint.sh /usr/local/bin/
USER root
RUN apk add --no-cache bash envsubst musl-locales musl-locales-lang && chmod +x /usr/local/bin/entrypoint.sh \
    && touch /usr/local/etc/haproxy/haproxy.cfg \
    && chown haproxy:haproxy /usr/local/etc/haproxy/haproxy.cfg \
    && mkdir -p /run/haproxy \
    && chown -R haproxy:haproxy /run/haproxy
ENV LANG=C.UTF-8
ENV LC_ALL=C.UTF-8
ENV HAPROXY_CONFIG=/usr/local/etc/haproxy/haproxy.cfg
USER haproxy
ENTRYPOINT [ "entrypoint.sh" ]
CMD ["haproxy", "-f", "/usr/local/etc/haproxy/haproxy.cfg", "-p", "/run/haproxy/haproxy.pid", "-db"]