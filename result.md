#### glommio


> ./wrk -t12 -c400 -d30s http://127.0.0.1:8080
Running 30s test @ http://127.0.0.1:8080
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    43.32ms    2.95ms  61.90ms   79.91%
    Req/Sec   759.83     49.66     1.28k    67.11%
  272396 requests in 30.04s, 18.96MB read
  Socket errors: connect 0, read 176804, write 95589, timeout 0
Requests/sec:   9068.35
Transfer/sec:    646.47KB

> ./wrk -t12 -c400 -d30s http://127.0.0.1:8080
Running 30s test @ http://127.0.0.1:8080
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    43.11ms    3.06ms  64.15ms   79.02%
    Req/Sec   763.52     52.70     1.00k    65.11%
  273708 requests in 30.06s, 19.06MB read
  Socket errors: connect 0, read 176803, write 96902, timeout 0
Requests/sec:   9105.10
Transfer/sec:    649.09KB

#### monoio

> ./wrk -t12 -c400 -d30s http://127.0.0.1:8080
Running 30s test @ http://127.0.0.1:8080
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    47.00ms    3.73ms  73.17ms   83.61%
    Req/Sec   699.15     51.01     1.89k    85.33%
  250618 requests in 30.06s, 17.45MB read
  Socket errors: connect 0, read 233954, write 16662, timeout 0
Requests/sec:   8337.83
Transfer/sec:    594.40KB
> ./wrk -t12 -c400 -d30s http://127.0.0.1:8080
Running 30s test @ http://127.0.0.1:8080
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    47.52ms    3.29ms  60.34ms   77.96%
    Req/Sec   691.43     40.42     0.99k    75.81%
  247859 requests in 30.04s, 17.26MB read
  Socket errors: connect 0, read 230943, write 16915, timeout 0
Requests/sec:   8250.15
Transfer/sec:    588.15KB

#### tokio

> ./wrk -t12 -c400 -d30s http://127.0.0.1:8080
Running 30s test @ http://127.0.0.1:8080
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    47.59ms    3.26ms  64.83ms   79.24%
    Req/Sec   691.86     41.66     0.88k    74.47%
  248034 requests in 30.05s, 17.27MB read
  Socket errors: connect 0, read 174428, write 73605, timeout 0
Requests/sec:   8255.21
Transfer/sec:    588.51KB
> ./wrk -t12 -c400 -d30s http://127.0.0.1:8080
Running 30s test @ http://127.0.0.1:8080
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    47.67ms    3.33ms  72.74ms   81.39%
    Req/Sec   690.55     39.08     0.94k    75.44%
  247606 requests in 30.07s, 17.24MB read
  Socket errors: connect 0, read 171346, write 76256, timeout 0
Requests/sec:   8235.61
Transfer/sec:    587.11KB
~/checkout/wrk master 

#### tokio-uring

> ./wrk -t12 -c400 -d30s http://127.0.0.1:8080
Running 30s test @ http://127.0.0.1:8080
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    50.38ms    3.58ms  72.22ms   82.00%
    Req/Sec   653.40     39.53     1.40k    77.42%
  234255 requests in 30.05s, 16.31MB read
  Socket errors: connect 0, read 187824, write 46429, timeout 0
Requests/sec:   7795.99
Transfer/sec:    555.77KB
> ./wrk -t12 -c400 -d30s http://127.0.0.1:8080
Running 30s test @ http://127.0.0.1:8080
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    50.16ms    3.42ms  62.86ms   80.19%
    Req/Sec   656.42     38.02     0.91k    74.75%
  235340 requests in 30.04s, 16.38MB read
  Socket errors: connect 0, read 187507, write 47831, timeout 0
Requests/sec:   7834.52
Transfer/sec:    558.52KB
