from typing import Callable

import numpy as np

from MultilayerPerceptron.Layer import Layer
from MultilayerPerceptron.LossFunctions import LOSS_FUNCTIONS
from MultilayerPerceptron.Metrics import METRICS
from MultilayerPerceptron.Neuron import Neuron
from MultilayerPerceptron.optimizers.Optimizer import Optimizer
from MultilayerPerceptron.optimizers.RMSprop import RMSprop


class MLP:
    def __init__(self, loss: str = "mse", optimizer: Optimizer = RMSprop):
        self.input_dim = 0

        self.layers = []

        if loss not in LOSS_FUNCTIONS.keys():
            raise ValueError(f"'{loss}' is not a valid loss function.")

        self.loss = LOSS_FUNCTIONS[loss]

        self.optimizer = optimizer

    def add_layer(self, layer):
        if len(self.layers) == 0:
            self.input_dim = layer.input_dim

        self.layers.append(layer)

        for i in range(len(self.layers) - 1):
            if self.layers[i].next_layer is None:
                self.layers[i].set_next_layer(self.layers[i + 1])

            self.layers[i].layer_name = f"{i}"
        self.layers[-1].layer_name = f"{len(self.layers) - 1}"

        if self.optimizer.requires_additional_attributes:
            layer.add_optimizer_required_attributes(self.optimizer)

    def get_layers(self):
        return self.layers

    def __backward_propagate_error(self, expected_output):
        for i in reversed(range(len(self.layers))):
            layer = self.layers[i]
            layer_neurons: list[Neuron] = layer.get_neurons()

            if i != len(self.layers) - 1:
                next_layer: Layer = self.layers[i + 1]

                layer_output = layer.layer_output
                for j, neuron in enumerate(layer_neurons):
                    # The neuron output relative error
                    next_layer_relative_error = 0

                    next_layer_neurons: list[Neuron] = next_layer.get_neurons()

                    for k, _neuron in enumerate(next_layer_neurons):
                        next_layer_relative_error += _neuron.delta * \
                                                     _neuron.weights[j]

                    # How much the output of h_i changes with respect to the
                    # neuron input
                    neuron_input_delta = \
                        layer.activation_derivative(layer_output[j])

                    # Calculate weight delta

                    # The "How much the total neuron input changes with
                    # respect to w_i" value is calculated when updating the
                    # parameter. This is just the output from previous layer
                    # related to w_i
                    neuron.delta = neuron_input_delta * next_layer_relative_error
            else:
                layer_output = layer.layer_output
                for j, neuron in enumerate(layer_neurons):
                    # How much the error change with respect to the output
                    error = layer_output[j] - expected_output[j]

                    # How much the output of o_i change with respect the
                    # neuron input
                    neuron_input_delta = \
                        layer.activation_derivative(layer_output[j])

                    # Calculate neuron delta

                    # The "How much the total neuron input changes with
                    # respect to w_i" value is calculated when updating the
                    # parameter. This is just the output from previous layer
                    # related to w_i
                    neuron.set_delta(error * neuron_input_delta)

    def __update_params(self, epoch: int):
        for i in reversed(range(len(self.layers))):
            layer: Layer = self.layers[i]

            forward_pass_input = layer.get_forward_pass_input()

            neurons: list[Neuron] = layer.get_neurons()

            for j, neuron in enumerate(neurons):
                for k in range(len(neuron.weights)):
                    self.optimizer.update_param(neuron=neuron, param_index=k,
                                                forward_pass_input=
                                                forward_pass_input, t=epoch)

    def optimize(self, x, y, epochs, metrics=None):
        if len(x) == 0:
            raise ValueError("No data provided.")
        elif len(x[0]) != self.input_dim:
            raise TypeError(
                "Data does not have the same input dimension as the network.")

        if metrics is not None:
            self.__validate_metrics(metrics)

        for epoch in range(epochs):
            raw_predictions = []

            for j, sample in enumerate(x):
                raw_output = self.predict([sample])[0]

                raw_predictions.append(raw_output)

                self.__backward_propagate_error(y[j])
                self.__update_params(epoch=epoch)

            log = f"Epoch={epoch} Loss (train): \
                {self.loss(np.array(y).ravel(), np.array(raw_predictions).ravel())} "

            for metric in metrics:
                log += f" {metric} (train): {METRICS[metric](y, raw_predictions)}"

            print(log)

    def predict(self, x):
        if len(x[0]) != self.input_dim:
            raise TypeError(
                "Data does not have the same input dimension as the network.")

        predicts = []
        for sample in x:
            output = sample
            for layer in self.layers:
                output = layer.feed_layer(output)

            predicts.append(output)

        return predicts

    def __validate_metrics(self, metrics: list[str]):
        for metric in metrics:
            if metric not in METRICS.keys():
                raise ValueError(f"'{metric}' is not a valid metric.")
