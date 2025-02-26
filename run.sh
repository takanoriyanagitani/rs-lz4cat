#!/bin/sh

input=/usr/share/dict/words

time cat "${input}" | gzip --fast                 | wc -c
time cat "${input}" | ENV_ENCODE=true ./rs-lz4cat | wc -c
time cat "${input}" | cat                         | wc -c

time cat "${input}" | gzip --fast                 | zcat        | shasum -a 256
time cat "${input}" | ENV_ENCODE=true ./rs-lz4cat | ./rs-lz4cat | shasum -a 256
time cat "${input}" | cat                         | cat         | shasum -a 256
