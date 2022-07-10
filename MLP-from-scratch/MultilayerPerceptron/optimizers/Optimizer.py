from MultilayerPerceptron.Neuron import Neuron


class Optimizer:
    def __init__(self, requires_additional_attributes: bool, lr: float = 0.01):
        self.lr = lr
        self.requires_additional_attributes = requires_additional_attributes

    def update_param(self,
                     forward_pass_input: [float],
                     neuron: Neuron,
                     param_i: int,
                     t: int):
        pass

    def add_optimizer_required_attributes(self, neuron: Neuron):
        pass
