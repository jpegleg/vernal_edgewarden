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
  cp /root/kigateway_https /opt/kiagateway/bin/keygateway_https
  cp /root/gate /etc/init.d/gate
  cp /root/proxy1 /etc/init.d/proxy1
  cp /root/redirectrix /etc/init.d/redirectrix
  cp /root/bluejay /etc/init.d/bluejay
}

ls /root/._ash_gen_.lock || new_init
ls /root/._ash_gen_.lock || mark_gen
