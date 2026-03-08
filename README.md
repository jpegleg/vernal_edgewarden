![cdlogo](https://carefuldata.com/images/cdlogo.png)

# Vernal Edgewarden

Alpine + Rust + Javascript/HTML/CSS

This build is a minimal build focused on performance, simplicity, and low resource consumption.

It deploy s the kiastack with [kiabluejay](https://github.com/jpegleg/kiabluejay) behind [kiaproxy](https://github.com/jpegleg/kiaproxy) and [kiagateway](https://github.com/jpegleg/kiagateway).

```
( internet ingress ) -> ( kiagateway )
                           |      |
                     (kiaproxy1) (kiaproxy2)     there is a kiaproxy for HTTP and one for HTTPS
                            \    /
                             \ /
                              |
                          (kiabluejay)           handles both HTTP and HTTPS listeners, redirecfting to HTTPS

```
_Note there could be failovers added to each kiaproxy instance, but here the purpose is to allow a restart of kiabluejay to not drop any traffic as kiaproxy will hold the client side of the connection until kiabluejay comes back up unless the connection retries of kiaproxy are exceeded._


This project includes precompiled static pie musl libc binaries for those Rust services, as well as ASH scripts for service management and certbot PKI.

This build is the evolution from https://github.com/jpegleg/serotinous-cone/ since OCI containers on new Alpine builds are not doing well with resource consumption and bugs.
