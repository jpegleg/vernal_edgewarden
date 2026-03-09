#!/bin/ash

mark_gen() {
  touch /root/._ash_gen_.lock
  chmod 600 /root/._ash_gen_.lock
}

new_init() {
  useradd -d /home/crab -m -s /bin/ash crab
  mkdir -p /opt/kiagateway/etc
  mkdir -p /opt/kiagateway/bin
  mkdir -p /opt/local/
  mkdir -p /home/crab/logs
  mkdir -p /home/crab/storage
  mkdir -p /var/www/html/
  chown -R crab:crab /home/crab/
  chown -R crab:crab /opt/local/
  chown -R crab:crab /var/www/html/
  cp /root/config.toml /opt/kiagateway/etc/config.toml
  cp /root/kigateway /opt/kiagateway/bin/keygateway
  cp /root/kiaproxy /root/kiaproxy1
  cp /root/kiaproxy /root/kiaproxy2
  cp /root/bluejay /usr/sbin/bluejay
  cp /root/gate /usr/sbin/gate
  cp /root/proxy1 /usr/sbin/proxy1
  cp /root/proxy2 /usr/sbin/proxy2
  cp /root/bluejay /usr/sbin/bluejay
}

ls /root/._ash_gen_.lock || new_init
ls /root/._ash_gen_.lock || mark_gen
