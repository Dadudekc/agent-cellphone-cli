"""Base agent defining interface.

>>> BaseAgent().run()
'base'
"""


class BaseAgent:
    def run(self) -> str:
        return "base"
