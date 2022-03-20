#!/bin/sh

# apt-get iptables in order to get ip command
ip tuntap \
	add \
	mode tap \
	name tap-rust \
	user root

ip link set tap-rust up
ip addr add 192.168.42.100/24 dev tap-rust

# apt-get iptables in order to get iptables
iptables \
	-t nat \
	-A POSTROUTING \
	-s 192.168.42.0/24 \
	-j MASQUERADE

sysctl net.ipv4.ip_forward=1

