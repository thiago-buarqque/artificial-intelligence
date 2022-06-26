import math
import numpy


def linear(x):
    return x


def linear_derivative(x):
    return 1


def sigmoid(x):
    return 1 / (1 + math.exp(-x))


def sigmoid_derivative(x):
    return x * (1.0 - x)


def relu(x):
    return numpy.maximum(x, 0)


def relu_derivative(x):
    return 0 if x < 0 else 1


def tanh(x):
    return numpy.tanh(x)


def tanh_derivative(x):
    return 1 - (numpy.tanh(x) ** 2)


ACTIVATION_FUNCTIONS = {
    'sigmoid': sigmoid,
    'relu': relu,
    'tanh': tanh,
    'linear': linear
}

ACTIVATION_FUNCTIONS_DERIVATIVES = {
    'sigmoid': sigmoid_derivative,
    'relu': relu_derivative,
    'tanh': tanh_derivative,
    'linear': linear_derivative
}
