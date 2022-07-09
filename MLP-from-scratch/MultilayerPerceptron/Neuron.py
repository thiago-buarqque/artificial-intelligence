from functools import partial
from typing import Callable

import numpy as np


class Neuron:
    def __init__(self, input_dim: int, activation_function: Callable[[float], any]):
        self.input_dim = input_dim
        self.activation_function = activation_function

        self.weights = np.random.uniform(-2, 2, input_dim + 1)

        self.delta = 0

    def forward_data(self, input_data):
        if len(input_data) != self.input_dim:
            raise TypeError("The input data does not match neuron input dimension")

        weighted_sum = 0
        for i, weight in enumerate(self.weights):
            if i != len(self.weights) - 1:
                weighted_sum += weight * input_data[i]
            else:
                # Adding bias
                weighted_sum += weight

        return self.activation_function(weighted_sum)

    def set_weights(self, weights):
        self.weights = weights

    def set_delta(self, delta):
        self.delta = delta

    def get_weights(self):
        return self.weights

    def register(self, attr_name, attr_value):

        if isinstance(attr_value, Callable):
            raise TypeError("Function value for Neuron attribute is not "
                            "supported.")

        setattr(self, attr_name, attr_value)
