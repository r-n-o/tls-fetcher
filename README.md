# tls-fetcher

Experimentation repo to fetch TLS content over sockets.

Right now this contains a program to fetch an HTTP response over a UNIX socket.

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
