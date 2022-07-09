from MultilayerPerceptron.Neuron import Neuron


class Optimizer:
    def __init__(self, requires_additional_attributes):
        self.requires_additional_attributes = requires_additional_attributes

    def update_param(self,
                     neuron: Neuron,
                     param_i: int,
                     forward_pass_input: [float]):
        pass

    def add_optimizer_required_attributes(self, neuron: Neuron):
        pass
