# Rust + Redis Client

```bash
cargo build --release
target/release/qraft
```
```text
❯ wrk -t12 -c200 -d30s http://localhost:8080/
Running 30s test @ http://localhost:8080/
  12 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     3.51ms    1.36ms  44.69ms   98.05%
    Req/Sec     4.68k   448.36    14.52k    92.01%
  1677165 requests in 30.10s, 127.96MB read
Requests/sec:  55712.09
Transfer/sec:      4.25MB
```

with my m1x


```bash
cargo run
```

```text
❯ wrk -t12 -c200 -d30s http://localhost:8080/
Running 30s test @ http://localhost:8080/
  12 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.91ms   11.62ms 216.16ms   96.06%
    Req/Sec     2.22k   149.03     3.08k    83.67%
  797191 requests in 30.02s, 60.82MB read
Requests/sec:  26558.54
Transfer/sec:      2.03MB
```