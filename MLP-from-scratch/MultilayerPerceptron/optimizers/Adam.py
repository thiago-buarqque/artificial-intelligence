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
                     forward_pass_input: [float],
                     neuron: Neuron,
                     param_i: int,
                     t: int):
        last_momentum = neuron.momentum[param_i]
        last_moving_avg = neuron.moving_avg[param_i]

        if param_i <= (len(forward_pass_input) - 1):
            param_gradient = neuron.delta * forward_pass_input[param_i]
        else:
            # It's the neuron bias
            param_gradient = neuron.delta

        mt = (self.b1 * last_momentum) + ((1 - self.b1) * param_gradient)

        vt = (self.b2 * last_moving_avg) + ((1 - self.b2) * param_gradient ** 2)

        neuron.momentum[param_i] = mt

        neuron.moving_avg[param_i] = vt

        mt_hat = mt / (1 - (math.pow(self.b1, t + 1)))

        vt_hat = vt / (1 - (math.pow(self.b2, t + 1)))

        neuron.weights[param_i] -= (self.lr / (
            math.sqrt(vt_hat) + 1e-8)) * mt_hat

    def add_optimizer_required_attributes(self, neuron: Neuron):
        neuron.register("moving_avg", np.zeros(neuron.input_dim + 1))
        neuron.register("momentum", np.zeros(neuron.input_dim + 1))
