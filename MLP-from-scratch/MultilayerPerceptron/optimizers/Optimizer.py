from abc import ABC, abstractmethod

from MultilayerPerceptron.Neuron import Neuron


class Optimizer(ABC):
    def __init__(self, requires_additional_attributes: bool, lr: float = 0.01):
        self.lr = lr
        self.requires_additional_attributes = requires_additional_attributes
        
        super().__init__()

    @abstractmethod
    def update_param(self,
                     forward_pass_input: list[float],
                     neuron: Neuron,
                     param_index: int,
                     t: int):
        pass

    @abstractmethod
    def add_required_attributes(self, neuron: Neuron):
        pass

    def get_param_gradient(self,
                           forward_pass_input: list[float],
                           neuron: Neuron,
                           param_index: int):

        if param_index <= (len(forward_pass_input) - 1):
            return neuron.delta * forward_pass_input[param_index]
        
        # It's the neuron bias
        return neuron.delta
