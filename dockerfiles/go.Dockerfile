ARG SERVICE

FROM golang:1.24-alpine AS builder

ARG SERVICE

WORKDIR /usr/src/t_lounge
COPY go.* ./

RUN go mod download && go mod verify

# Uncomment and adjust if you need to copy libs
COPY libs/go/ libs/go/
COPY protos protos
COPY services/go/$SERVICE services/go/$SERVICE/

RUN go build -o ./bin/$SERVICE ./services/go/$SERVICE/cmd

FROM alpine:3.21

ARG SERVICE
ENV BIN=$SERVICE

RUN apk add --no-cache ca-certificates

WORKDIR /t_lounge

RUN adduser -DH t_lounge
USER t_lounge

COPY --from=builder /usr/src/t_lounge/bin/$SERVICE $SERVICE

CMD /t_lounge/$BIN
