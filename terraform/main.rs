resource "vultr_ssh_key" "enre" {
  name = "enre"
  ssh_key = "YOURPUBLICSSHKEY"
}

resource "vultr_instance" "warden_south_us" {
    hostname = "TEMPLATENAME"
    plan = "vc2-1c-1gb"
    region = "atl"
    os_id = 2076
    ssh_key_ids = ["${vultr_ssh_key.enre.id}"]
    label = "edgewarden"
}
