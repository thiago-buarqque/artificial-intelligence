import numpy as np

from MultilayerPerceptron.ActivationFunctions import (
    ACTIVATION_FUNCTIONS,
    ACTIVATION_FUNCTIONS_DERIVATIVES
)
from MultilayerPerceptron.Neuron import Neuron
from MultilayerPerceptron.optimizers.Optimizer import Optimizer


class Layer:
    def __init__(self,
                 neurons: int,
                 input_dim: int,
                 activation_function: str = 'linear'
                 ):
        self.input_dim = input_dim
        self.output_dim = neurons

        self.forward_pass_input: [float] = []
        self.layer_output = None

        if activation_function not in ACTIVATION_FUNCTIONS.keys():
            raise ValueError("Invalid activation function.")

        self.neurons = [Neuron(
            input_dim,
            ACTIVATION_FUNCTIONS[activation_function]
        ) for _ in range(neurons)]

        self.activation_derivative = ACTIVATION_FUNCTIONS_DERIVATIVES[activation_function]

        self.next_layer = None
        self.layer_name = ''

    def set_next_layer(self, next_layer):
        self.next_layer = next_layer

    def feed_layer(self, input_data: [float]):
        if len(input_data) != self.input_dim:
            raise TypeError(
                f"Input data does not have the same dimension as layer. ({len(input_data), self.input_dim})")

        self.forward_pass_input = input_data

        layer_output = np.zeros(self.output_dim)
        for i, weight in enumerate(self.neurons):
            layer_output[i] = weight.forward_data(input_data)

        self.layer_output = layer_output

        return layer_output

    def get_forward_pass_input(self):
        return self.forward_pass_input

    def get_neurons(self) -> [Neuron]:
        return self.neurons

    def add_optimizer_required_attributes(self, optimizer: Optimizer):
        for neuron in self.neurons:
            optimizer.add_optimizer_required_attributes(neuron)

    def __str__(self):
        return f"Layer #{self.layer_name} ({self.input_dim}, {self.output_dim})"
