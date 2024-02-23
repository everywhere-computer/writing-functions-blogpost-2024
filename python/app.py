import maths
from maths.imports.logging import (log, Level)

class Maths(maths.Maths):
    def multiply(self, a, b) -> int:
        result = a * b

        log(Level.INFO, 'guest:python:multiply', '{} * {} = {}'.format(a, b, result))

        return a * b
