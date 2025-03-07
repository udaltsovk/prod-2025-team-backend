FROM alpine:3.21

WORKDIR /opt

RUN apk add curl

RUN curl -fsSL https://raw.githubusercontent.com/pressly/goose/master/install.sh | sh

COPY /services/go/coworking/db/migrations ./migrations

CMD ["goose", "up"]