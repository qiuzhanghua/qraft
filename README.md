# Rust + Redis Client + MySQL

## Test Redis
```bash
cargo run --release
```
with m1x
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

```bash
cargo run
```
with old mac
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

## Test MySQL
```bash
cargo run
```
with m1x

```text
❯ wrk -t12 -c200 -d30s http://localhost:8080/db
Running 30s test @ http://localhost:8080/db
  12 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    15.27ms  441.17us  28.76ms   76.80%
    Req/Sec     1.05k    30.68     1.13k    69.81%
  377284 requests in 30.04s, 29.14MB read
Requests/sec:  12560.64
Transfer/sec:      0.97MB
```

```bash
cargo run --release
```
m1x with release

```text
❯ wrk -t12 -c200 -d30s http://localhost:8080/db
Running 30s test @ http://localhost:8080/db
  12 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.64ms  399.45us  21.05ms   97.37%
    Req/Sec     2.85k   106.35     2.94k    95.81%
  1022376 requests in 30.01s, 78.98MB read
Requests/sec:  34062.93
Transfer/sec:      2.63MB
```
