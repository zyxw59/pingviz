#!/bin/sh

sed -r 's/^.* icmp_seq=([0-9]+).* time=([0-9]*(\.[0-9]+)?) ms$/\1 \2/'
