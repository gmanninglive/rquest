FROM alpine:3.14
ENV KAFKA_VERSION=3.3.1
ENV SCALA_VERSION=2.13

WORKDIR /app

COPY ./kafka/install_kafka.sh /bin/
COPY ./kafka/start_kafka.sh /bin/

RUN apk update \
  && apk add --no-cache bash curl jq openjdk8-jre\
  && /bin/install_kafka.sh \
  && apk del curl jq

CMD [ "/bin/start_kafka.sh" ]
