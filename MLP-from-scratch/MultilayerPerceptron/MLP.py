import math

import numpy as np

from MultilayerPerceptron.Layer import Layer
from MultilayerPerceptron.Neuron import Neuron

from MultilayerPerceptron.LossFunctions import LOSS_FUNCTIONS
from MultilayerPerceptron.Metrics import METRICS


class MLP:
    def __init__(self, lr=0.01, classify_function=None, loss="mse"):
        self.input_dim = 0

        self.layers = []
        self.lr = lr

        """
        Used for binary classification. Returns 0 or 1 based on a user condition
        
        Example:
        def classify_function(net_output):
            if net_output > 0.5:
                return 1
            return 0
        """
        self.classify_function = classify_function

        if loss not in LOSS_FUNCTIONS.keys():
            raise ValueError(f"'{loss}' is not a valid loss function.")

        self.loss = LOSS_FUNCTIONS[loss]

    def add_layer(self, layer):
        if len(self.layers) == 0:
            self.input_dim = layer.input_dim
        
        self.layers.append(layer)

        for i in range(len(self.layers) - 1):
            if self.layers[i].next_layer is None:
                self.layers[i].set_next_layer(self.layers[i + 1])
            
            self.layers[i].layer_name = f"{i}"
        self.layers[-1].layer_name = f"{len(self.layers) - 1}"

    def get_layers(self):
        return self.layers

    # RMSProp moving average
    def _param_moving_avg(self, last_step_size, param_gradient):
        return (0.9 * last_step_size) + ((1 - 0.9) * (param_gradient ** 2))

    def _backward_propagate_error(self, expected_output):
        # https://mattmazur.com/2015/03/17/a-step-by-step-backpropagation-example/
        for i in range(len(self.layers) - 1, -1, -1):
            layer = self.layers[i]
            layer_neurons: [Neuron] = layer.get_neurons()

            # It's a hidden layer
            if i != len(self.layers) - 1:
                next_layer: Layer = self.layers[i + 1]

                layer_output = layer.layer_output
                for j, neuron in enumerate(layer_neurons):
                    # The neuron output relative error
                    next_layer_relative_error = 0

                    next_layer_neurons: [Neuron] = next_layer.get_neurons()

                    for k, _neuron in enumerate(next_layer_neurons):
                        next_layer_relative_error += _neuron.delta * _neuron.weights[j]

                    # How much the output of h_i change with respect the neuron input
                    neuron_input_delta = layer.activation_derivative(layer_output[j])

                    # Calculate weight delta

                    # The "How much the total neuron input changes with respect to w_i" value
                    # is calculated when updating the parameter. This is just the output
                    # from previous layer related to w_i
                    neuron.delta = neuron_input_delta * next_layer_relative_error
            else:
                layer_output = layer.layer_output
                for j, neuron in enumerate(layer_neurons):
                    # How much the error change with respect to the output
                    output_delta = layer_output[j] - expected_output[j]

                    # How much the output of o_i change with respect the neuron input
                    neuron_input_delta = layer.activation_derivative(layer_output[j])

                    # Calculate neuron delta

                    # The "How much the total neuron input changes with respect to w_i" value
                    # is calculated when updating the parameter. This is just the output
                    # from previous layer related to w_i
                    neuron.set_delta(output_delta * neuron_input_delta)

    def _update_params(self):
        for i in reversed(range(len(self.layers))):
            layer: Layer = self.layers[i]

            forward_pass_input = layer.get_forward_pass_input()

            neurons: [Neuron] = layer.get_neurons()

            for j, neuron in enumerate(neurons):
                for k in range(len(neuron.weights)):
                    last_moving_avg = neuron.moving_avg[k]

                    if k <= len(forward_pass_input) - 1:
                        # forward_pass_input[j] is "How much the total neuron input changes with respect to w_i"
                        param_gradient = neuron.delta * forward_pass_input[k]
                    else:
                        # It's the neuron bias
                        param_gradient = neuron.delta

                    new_moving_avg = self._param_moving_avg(
                        last_moving_avg,
                        param_gradient
                    )

                    neuron.moving_avg[k] = new_moving_avg

                    step_size = self.lr / (1e-8 + math.sqrt(new_moving_avg))

                    neuron.weights[k] -= step_size * param_gradient

    def optimize(self, x, y, epochs, metrics=None):
        if len(x) == 0:
            raise ValueError("No data provided.")
        elif len(x[0]) != self.input_dim:
            raise TypeError("Data does not have the same input dimension as the network.")

        if metrics is not None:
            self._validate_metrics(metrics)

        for i in range(epochs):
            predictions = []
            raw_predictions = []
            for j, sample in enumerate(x):
                raw_output = self.predict([sample])[0]

                if self.classify_function is not None:
                    output = self.classify_function(raw_output)
                    predictions.append(output)

                raw_predictions.append(raw_output)

                self._backward_propagate_error(y[j])
                self._update_params()

            log = f"Epoch={i} Loss (train): {self.loss(np.array(y).ravel(), np.array(raw_predictions).ravel())}"

            for metric in metrics:
                log += f" {metric} (train): {METRICS[metric](y, raw_predictions)}"

            print(log)

    def predict(self, x):
        if len(x[0]) != self.input_dim:
            raise TypeError("Data does not have the same input dimension as the network.")

        predicts = []
        for sample in x:
            output = sample
            for k, layer in enumerate(self.layers):
                output = layer.feed_layer(output)

            predicts.append(output)

        return predicts

    def evaluate(self, x):
        """
        Make a prediction and classify it using the classify function
        :param x:
        :return: The proper class for the network prediction
        """
        prediction = self.evaluate(x)

        return self.classify_function(prediction)

    def _validate_metrics(self, metrics: [str]):
        for metric in metrics:
            if metric not in METRICS.keys():
                raise ValueError(f"'{metric}' is not a valid metric.")
