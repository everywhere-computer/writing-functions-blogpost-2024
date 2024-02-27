import multiplication
from multiplication.imports.logging import (log, Level)

class Multiplication(multiplication.Multiplication):
    def multiply(self, a, b) -> float:
        result = a * b

        log(Level.INFO, 'guest:python:multiply', '{} * {} = {}'.format(a, b, result))

        return a * b
