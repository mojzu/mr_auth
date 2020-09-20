FROM sso/build-release:latest as build

# depend: docker pull node:14.10-buster
FROM node:14.10-buster

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt
COPY --from=build /build/package.json /opt/package.json
COPY --from=build /build/package-lock.json /opt/package-lock.json
COPY --from=build /build/node_modules /opt/node_modules
COPY --from=build /build/sso_test /opt/sso_test