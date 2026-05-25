import redis
from common import key

r = redis.Redis(host="localhost", port=6379, db=0)


def test_set_get_int():
    k = key("test_set_get_int")
    assert r.get(k) is None
    assert r.set(k, 3) is True
    assert r.get(k) == b"3"


def test_set_get_string():
    k = key("test_set_get_string")
    assert r.get(k) is None
    assert r.set(k, "my_val") is True
    assert r.get(k) == b"my_val"
