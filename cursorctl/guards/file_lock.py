"""File lock context manager.

>>> import tempfile, os
>>> path = tempfile.mktemp()
>>> with FileLock(path):
...     os.path.exists(path)
True
"""

from contextlib import contextmanager
from pathlib import Path


@contextmanager
def FileLock(path: str):
    lock = Path(path)
    lock.touch()
    try:
        yield
    finally:
        if lock.exists():
            lock.unlink()
