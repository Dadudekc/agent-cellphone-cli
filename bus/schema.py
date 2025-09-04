"""Message schema for bus.

>>> Message("alice", "hello").to_dict()
{'sender': 'alice', 'payload': 'hello'}
"""

from dataclasses import dataclass, asdict


@dataclass
class Message:
    sender: str
    payload: str

    def to_dict(self) -> dict:
        return asdict(self)
