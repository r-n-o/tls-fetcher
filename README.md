# tls-fetcher

Experimentation repo to fetch TLS content over sockets.

Right now this contains a program to fetch an HTTP (**non**-TLS) response over a UNIX socket.

To run it:
* start a socket proxy with `socat`:
  ```
  socat UNIX-LISTEN:/tmp/host.sock,reuseaddr,fork TCP:$(dig +short www.dest-unreach.org):80
  ```
  the above will proxy connections established at `/tmp/host.sock` to the host for `www.dest-unreach.org` on port 80
* then run the run program:
  ```
  cargo run
  ```

Next up: write a rust program to replace `socat` and figure out how to make the host dynamic. Also: use HTTPS instead of barebone http.

# Research links

* https://aws.amazon.com/blogs/database/aws-nitro-enclaves-for-running-ethereum-validators-part-2/ has a really good diagram showing what needs to happen to contact an external host from inside of a nitro enclave
* `vsock_proxy`, which proxies vsock to inet, is written in Rust: https://github.com/aws/aws-nitro-enclaves-cli/tree/main/vsock_proxy
* `kmstool-enclave-cli` establishes a connection to vsock proxy to reach KMS APIs. It's open source and available, written in C: https://github.com/aws/aws-nitro-enclaves-sdk-c/blob/main/bin/kmstool-enclave-cli/main.c
* This comment has a snippet to make an HTTP request over a unix socket with `hyper`: https://github.com/seanmonstar/reqwest/issues/39#issuecomment-2063626443
