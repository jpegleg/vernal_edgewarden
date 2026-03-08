![cdlogo](https://carefuldata.com/images/cdlogo.png)

# Vernal Edgewarden

Alpine + Rust microservices/monoliths/apps + Javascript/HTML/CSS

This build is a minimal build focused on performance, simplicity, and low resource consumption.

It deploy s the kiastack with [kiabluejay](https://github.com/jpegleg/kiabluejay) behind [kiaparoxy](https://github.com/jpegleg/kiaproxy) and [kiagateway](https://github.com/jpegleg/kiagateway).

This project includes precompiled statci pie musl libc binaries for those Rust services, as well as ASH scripts for service management and certbot PKI.

This build is the evolution from https://github.com/jpegleg/serotinous-cone/ since OCI containers on new Alpine builds are not doing well with resource consumption and bugs.

