import math

import numpy as np

from MultilayerPerceptron.Neuron import Neuron
from MultilayerPerceptron.optimizers.Optimizer import Optimizer


class RMSprop(Optimizer):
    def __init__(self, lr: float = 0.01):
        super().__init__(requires_additional_attributes=True, lr=lr)

    def update_param(self,
                     forward_pass_input: list[float],
                     neuron: Neuron,
                     param_index: int,
                     t: int):
        
        last_moving_avg = neuron["moving_avg"][param_index]

        param_gradient = self.get_param_gradient(
            forward_pass_input, neuron, param_index
        )

        new_moving_avg = self.__calculate_moving_avg(
            last_moving_avg,
            param_gradient
        )

        neuron.moving_avg[param_index] = new_moving_avg

        step_size = self.__calculate_step_size(new_moving_avg)

        neuron.weights[param_index] -= step_size * param_gradient

    def add_required_attributes(self, neuron: Neuron):
        neuron.register("moving_avg", np.zeros(neuron.input_dim + 1))

    def __calculate_step_size(self, moving_avg: float) -> float:
        return self.lr / (1e-8 + math.sqrt(moving_avg))

    def __calculate_moving_avg(self, 
                     last_moving_avg: float,
                     param_gradient: float) -> float:
        
        return (0.9 * last_moving_avg) + ((1 - 0.9) * (param_gradient ** 2))
