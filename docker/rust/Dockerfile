FROM rust:latest

WORKDIR /app

RUN apt-get update \
	&& apt-get upgrade -y \
	&& apt-get install -y -q \
	iptables \
	iproute2 \
	git \
	wget

