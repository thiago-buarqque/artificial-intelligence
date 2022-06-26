from sklearn.metrics import log_loss
import random


def activation_function(value):
    return 1 if value >= 0 else 0


def accuracy_metric(actual, predicted):
    correct = 0
    for i in range(len(actual)):
        if actual[i] == predicted[i]:
            correct += 1
    return correct / float(len(actual)) * 100.0


class Perceptron:
    def __init__(self, input_dim=2, lr=0.1):
        self.input_dim = input_dim
        # The first weights is for the bias
        self.weights = [random.uniform(-1, 1) for n in range(input_dim + 1)]
        self.learning_rate = lr

    def train(self, iterations, x, y):
        if len(x) == 0:
            raise ValueError("There is no training data")
        elif len(x[0]) != self.input_dim:
            raise TypeError(f"Training data dim does not correspond to the input dim.")

        for i in range(iterations):
            predictions = []
            for j, sample in enumerate(x):
                sum_weights = 0

                for k, w in enumerate(self.weights):
                    if k == 0:  # Is bias
                        sum_weights += w
                    else:
                        sum_weights += w * sample[k - 1]
                output = activation_function(sum_weights)
                predictions.append(output)

                sample_loss = y[j] - output

                for k in range(len(self.weights)):
                    if k == 0:
                        self.weights[k] = self.weights[k] + (self.learning_rate * sample_loss)
                    else:
                        self.weights[k] = self.weights[k] + (self.learning_rate * sample_loss * sample[k - 1])
            print(f'Epoch={i} Loss: {log_loss(y, predictions)} Accuracy: {accuracy_metric(y, predictions)}')

    def test(self, x, y):
        if len(x) == 0:
            raise ValueError("There is no data")
        elif len(x[0]) != self.input_dim:
            raise TypeError(f"Training data dim does not correspond to the input dim.")

        predictions = []
        for sample in x:
            weights_sum = 0

            for k, w in enumerate(self.weights):
                if k == 0:
                    weights_sum += w
                else:
                    weights_sum += w * sample[k - 1]

            output = activation_function(weights_sum)
            predictions.append(output)

        print(f'Test loss: {log_loss(y, predictions)}')
        print(f'Test accuracy: {accuracy_metric(y, predictions)}')
