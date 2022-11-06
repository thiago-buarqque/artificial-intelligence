from typing import Callable, Union

Bounds = Union[list[list[float]], list[list[int]]]
FitnessFunction = Callable[[Union[list[int], list[float]]], Union[int, float]]