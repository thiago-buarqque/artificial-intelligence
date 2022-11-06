import math

import numpy as np

from MultilayerPerceptron.Neuron import Neuron
from MultilayerPerceptron.optimizers.Optimizer import Optimizer


class Adam(Optimizer):
    def __init__(self, b1: float = 0.9, b2: float = 0.999, lr: float = 0.01):
        super().__init__(requires_additional_attributes=True, lr=lr)

        self.b1 = b1
        self.b2 = b2

    def update_param(self,
                     forward_pass_input: list[float],
                     neuron: Neuron,
                     param_index: int,
                     t: int):
                     
        last_momentum = neuron.momentum[param_index]

        last_moving_avg = neuron.moving_avg[param_index]

        param_gradient = self.get_param_gradient(
            forward_pass_input, neuron, param_index
        )

        momentum = (self.b1 * last_momentum) + ((1 - self.b1) * param_gradient)

        past_gradients = \
            (self.b2 * last_moving_avg) + ((1 - self.b2) * param_gradient ** 2)

        neuron.momentum[param_index] = momentum

        neuron.moving_avg[param_index] = past_gradients

        momentum_corrected = momentum / (1 - (math.pow(self.b1, t + 1)))

        past_gradients_corrected = \
            past_gradients / (1 - (math.pow(self.b2, t + 1)))

        neuron.weights[param_index] -= (self.lr / \
            (math.sqrt(past_gradients_corrected) + 1e-8)) * momentum_corrected

    def add_required_attributes(self, neuron: Neuron):
        neuron.register("moving_avg", np.zeros(neuron.input_dim + 1))
        neuron.register("momentum", np.zeros(neuron.input_dim + 1))
