# Simple Benchmark

## macOS

### Encode

| tool/codec             | elapsed | CPU  | rate     | ratio  | size   |
|:----------------------:|:-------:|:----:|:--------:|:------:|:------:|
| (dd)                   |  1.4s   | -    | 4.9 GB/s | -      | 7.1 GB |
| gzip(parallel)         |  3.6s   | 848% | 2.0 GB/s | -      | 3.8 GB |
| rs-lz4cat(native)      |  5.1s   |  93% | 1.2 GB/s | 1300%  | 4.1 GB |
| snappy                 |  6.0s   |  94% | 1.2 GB/s | 1100%  | 4.3 GB |
| zstd                   |  9.5s   | 165% | 0.7 GB/s |  700%  | 3.8 GB |
| rs-lz4cat(wasi/wazero) | 11.4s   |  97% | 0.6 GB/s |  600%  | 4.1 GB |
| gzip(fast)             | 65.4s   | 100% | 0.1 GB/s | (100%) | 3.7 GB |

### Decode

| tool/codec             | elapsed | CPU  | rate     | ratio  |
|:----------------------:|:-------:|:----:|:--------:|:------:|
| (dd)                   |  1.4s   | -    | 4.9 GB/s | -      |
| rs-lz4cat(native)      |  3.0s   |  84% | 2.3 GB/s |  310%  |
| zstd                   |  3.5s   | 225% | 1.9 GB/s |  270%  |
| rs-lz4cat(wasi/wazero) |  4.2s   |  92% | 1.7 GB/s |  220%  |
| snappy                 |  4.5s   |  90% | 1.6 GB/s |  210%  |
| zcat                   |  9.3s   | 100% | 0.8 GB/s | (100%) |
