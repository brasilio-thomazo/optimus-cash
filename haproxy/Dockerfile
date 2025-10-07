FROM haproxy:alpine
COPY ./haproxy.cfg /usr/local/etc/haproxy/haproxy.cfg.template
COPY ./entrypoint.sh /usr/local/bin/
USER root
RUN apk add --no-cache bash envsubst musl-locales musl-locales-lang && chmod +x /usr/local/bin/entrypoint.sh \
    && touch /usr/local/etc/haproxy/haproxy.cfg \
    && chown haproxy:haproxy /usr/local/etc/haproxy/haproxy.cfg
ENV LANG=C.UTF-8
ENV LC_ALL=C.UTF-8
USER haproxy
ENTRYPOINT [ "entrypoint.sh" ]
CMD ["haproxy", "-f", "/usr/local/etc/haproxy/haproxy.cfg", "-db"]