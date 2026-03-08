#!/bin/ash
pgrep kiabluejay || bluejay start
pgrep kiaproxy1 || proxy1 start
pgrep kiaproxy2 || proxy2 start
pgrep kiagateway || gate start
