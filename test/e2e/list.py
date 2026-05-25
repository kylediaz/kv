import redis
from common import key

r = redis.Redis(host="localhost", port=6379, db=0, decode_responses=True)


def test_lpop_empty():
    assert r.lpop(key("test_lpop_empty")) is None


def test_rpop_empty():
    assert r.rpop(key("test_rpop_empty")) is None


def test_lops():
    k = key("test_lops")
    for i in range(1, 10):
        assert r.lpush(k, i) == i
    for i in reversed(range(1, 10)):
        assert r.lpop(k) == str(i)


def test_rops():
    k = key("test_rops")
    for i in range(1, 10):
        assert r.rpush(k, i) == i
    for i in reversed(range(1, 10)):
        assert r.rpop(k) == str(i)
