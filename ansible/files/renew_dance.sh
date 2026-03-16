#!/bin/sh
service redirectrix stop &&
sh /root/certbot_TEMPLATE.sh &&
service redirectrix start
