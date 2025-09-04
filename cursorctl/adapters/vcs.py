"""VCS adapter.

>>> commit("init")
'committed: init'
>>> import subprocess
>>> subprocess.check_output(["bash", "cursorctl/cursorctl.sh", "hello"], text=True).strip()
'cursorctl: hello'
"""


def commit(message: str) -> str:
    return f"committed: {message}"
