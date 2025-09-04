"""Simple editor RPC adapter.

>>> send_command("ping")
'pong'
"""


def send_command(cmd: str) -> str:
    return "pong" if cmd == "ping" else "unknown"
