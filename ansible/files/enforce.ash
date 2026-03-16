#!/bin/ash
pgrep kiabluejay || service bluejay start
pgrep kiaproxy1 || service proxy1 start
pgrep redirectrix || service redirectrix start
pgrep kiagateway_https || service gate start
