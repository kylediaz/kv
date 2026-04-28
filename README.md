# KV

In-memory key-value store that's a drop-in replacement for Redis.

Implements the RESP v2 protocol (and enough of v1 to be compatible with the cli)

## Usage

```
flox activate # optional, installs redis-cli, python, etc
cargo bench   # runs benchmarks
cargo run     # runs server
redis-cli     # connect to REPL
```

## Coverage

| Command             | Status |
| ------------------- | ------ |
| SET, MSET           | OK     |
| GET, MGET           | OK     |
| DEL                 | OK     |
| EXISTS              | TODO   |
| EXPIRES, TTL        | TODO   |
| INCR                | OK     |
| HSET, HGET, HGETALL | TODO   |
| PUSH, POP           | OK     |
| SADD, SMEMBERS      | TODO   |
| ZADD                | TODO   |
