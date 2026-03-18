In most cases use [redirectrix2](https://github.com/jpegleg/vernal_edgewarden/tree/main/redirectrix2) instead of this original version of redirectrix.

However, if you don't need to do ACME HTTP-01, then this version is _much_ smaller and lighter and can then be used.

Redirectrix2 supports ACME HTTP-01 challenges by serving .well-known with Actix and redirecting everything else to HTTPS.

This original redirectrix only does HTTPS redirects, nothing else.
