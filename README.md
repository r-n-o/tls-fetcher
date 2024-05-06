# TLS Fetcher

Experimentation repo to fetch TLS content over Unix or VSOCK sockets.

Right now this contains a program to fetch TLS response over a UNIX socket which proxies to TCP.

## Running

To run this:

* start a socket proxy with `socat`:
  ```
  socat UNIX-LISTEN:/tmp/host.sock,reuseaddr,fork TCP:$(dig +short www.googleapis.com|tail -n1):443
  ```
  the above will proxy connections established at `/tmp/host.sock` to the host for `www.googleapis.com` on port 443 (for TLS)
* Run the Rust program:
  ```
  cargo run
  ```

Next up: write a rust program to replace `socat` and figure out how to make the host dynamic!

# Research links

* https://aws.amazon.com/blogs/database/aws-nitro-enclaves-for-running-ethereum-validators-part-2/ has a really good diagram showing what needs to happen to contact an external host from inside of a nitro enclave
  ![image](https://github.com/r-n-o/tls-fetcher/assets/104520680/51c72dfe-4dd0-4e49-92e6-a87269698241)

* `vsock_proxy`, which proxies vsock to inet, is written in Rust: https://github.com/aws/aws-nitro-enclaves-cli/tree/main/vsock_proxy
* `kmstool-enclave-cli` establishes a connection to vsock proxy to reach KMS APIs. It's open source and available, written in C: https://github.com/aws/aws-nitro-enclaves-sdk-c/blob/main/bin/kmstool-enclave-cli/main.c
* This comment has a snippet to make an HTTP request over a unix socket with `hyper`: https://github.com/seanmonstar/reqwest/issues/39#issuecomment-2063626443
* [this example](https://github.com/rustls/rustls/blob/main/examples/src/bin/simpleclient.rs) from `rustls` shows a simple client
