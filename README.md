# KV

In-memory key-value store that's a drop-in replacement for Redis.

## Usage

```
devenv shell # optional, installs redis-cli, python, etc
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
| PUSH, POP, LEN      | OK     |
| SADD, SMEMBERS      | TODO   |
| ZADD                | TODO   |
