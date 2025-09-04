"""Execute shell commands.

>>> run_command("echo hi")
'hi'
"""

from subprocess import check_output


def run_command(cmd: str) -> str:
    return check_output(cmd, shell=True, text=True).strip()
