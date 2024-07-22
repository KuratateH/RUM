# FROM alpine:3.16 AS builder

# ARG VERSION=0.1.0
# ARG TARGETPLATFORM
# ARG PLATFORM=${TARGETPLATFORM#linux/}

# WORKDIR /home/rum

# RUN apk add --no-cache curl tar gzip \
#  && curl -LO https://github.com/kuratateh/rum/releases/download/v${VERSION}/rum-${VERSION}_linux_${PLATFORM}.tar.gz \
#  && tar xvfz rum-${VERSION}_linux_${PLATFORM}.tar.gz 

# FROM alpine:3.16

# ARG VERSION=0.1.0

# LABEL org.opencontainers.image.source https://github.com/KuratateH/rum

# RUN  apk add --no-cache libgcc musl-dev \
#   && adduser -D nonroot \
#   && mkdir -p /workdir

# COPY --from=builder /home/rum/rum-${VERSION}/rum /opt/rum/rum

# WORKDIR /workdir
# USER nonroot

# ENTRYPOINT [ "/opt/rum/rum" ]


FROM alpine:3.16 AS builder

ARG VERSION=0.1.0
ARG TARGETPLATFORM
ARG PLATFORM=${TARGETPLATFORM#linux/}

WORKDIR /home/rum

RUN apk add --no-cache curl tar gzip \
 && curl -LO https://github.com/kuratateh/rum/releases/download/v${VERSION}/rum-${VERSION}_linux_${PLATFORM}.tar.gz \
 && tar xvfz rum-${VERSION}_linux_${PLATFORM}.tar.gz 

FROM alpine:3.16

ARG VERSION=0.1.0

LABEL org.opencontainers.image.source https://github.com/kuratateh/rum

RUN  apk add --no-cache libgcc musl-dev \
  && adduser -D nonroot \
  && mkdir -p /workdir

COPY --from=builder /home/rum/rum-${VERSION}/rum /opt/rum/rum

WORKDIR /workdir
USER nonroot

ENTRYPOINT [ "/opt/rum/rum" ]