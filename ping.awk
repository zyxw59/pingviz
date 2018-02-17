#!/usr/bin/awk

BEGIN {FS="icmp_seq=|ttl=|time=|ms"}
{print $2+$4/1000.0 " " $4; fflush()}
