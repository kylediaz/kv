import uuid


def key(prefix: str) -> str:
    return f"{prefix}:{uuid.uuid4().hex[:8]}"
