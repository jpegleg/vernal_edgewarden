#!/bin/ash
apk add certbot
ufw allow 443/tcp
ufw allow 80/tcp
ufw reload
