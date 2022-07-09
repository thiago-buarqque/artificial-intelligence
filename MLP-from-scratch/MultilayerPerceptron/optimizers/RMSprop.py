import math

import numpy as np

from MultilayerPerceptron.Neuron import Neuron
from MultilayerPerceptron.optimizers.Optimizer import Optimizer


class RMSprop(Optimizer):
    def __init__(self, lr: float = 0.01):
        super().__init__(requires_additional_attributes=True)

        self.lr = lr

    def update_param(self,
                     neuron: Neuron,
                     param_i: int,
                     forward_pass_input: [float]):
        last_moving_avg = neuron.moving_avg[param_i]

        if param_i <= (len(forward_pass_input) - 1):
            param_gradient = neuron.delta * forward_pass_input[param_i]
        else:
            # It's the neuron bias
            param_gradient = neuron.delta

        new_moving_avg = self.__param_moving_avg(
            last_moving_avg,
            param_gradient
        )

        neuron.moving_avg[param_i] = new_moving_avg

        step_size = self.lr / (1e-8 + math.sqrt(new_moving_avg))

        neuron.weights[param_i] -= step_size * param_gradient

    def add_optimizer_required_attributes(self, neuron: Neuron):
        neuron.register("moving_avg", np.zeros(neuron.input_dim + 1))

    def __calculate_step_size(self, step_size: float,
                              new_moving_avg: float) -> float:
        return step_size / (1e-8 + math.sqrt(new_moving_avg))

    def __param_moving_avg(self, last_moving_avg: float,
                           param_gradient: float) -> float:
        return (0.9 * last_moving_avg) + ((1 - 0.9) * (param_gradient ** 2))
