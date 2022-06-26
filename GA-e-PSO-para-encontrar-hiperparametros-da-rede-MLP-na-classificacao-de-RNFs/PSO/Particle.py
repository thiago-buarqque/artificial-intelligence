import random
from math import e


def sigmoid(velocity):
    return 1 / (1 + (e ** (-velocity)))


class Particle:
    def __init__(self):
        self.positions = []
        self.velocities = []
        self.fitness = -1
        self.fitness_pbest = -1
        self.list_pbest = []

    def update_velocity(self, g_best_positions, dimension, c1, c2, bounds, w):
        for i in range(dimension):
            e1 = random.uniform(0, 1)
            e2 = random.uniform(0, 1)
            cognitive_velocity = c1 * e1 * (self.list_pbest[i] - self.positions[i])
            social_velocity = c2 * e2 * (g_best_positions[i] - self.positions[i])

            v = (w * self.velocities[i]) + cognitive_velocity + social_velocity

            if v > bounds[i][1]:
                v = bounds[i][1]
            elif v < bounds[i][0]:
                v = bounds[i][0]

            self.velocities[i] = v

    def update_position(self, bounds, dimension):
        integer_values_indexes = [0, 1, 2, 5]

        learning_rate_idx = 3
        dropout_idx = 4
        batch_size_idx = 5
        for i in range(dimension):

            if i == batch_size_idx:
                # Kennedy and Eberhart PSO adaptation (BPSO)
                r = random.uniform(0, 1)
                if r < sigmoid(self.velocities[i]):
                    self.positions[i] = 32
                else:
                    self.positions[i] = 64
                continue
            else:
                new_position = self.positions[i] + self.velocities[i]

            if i in integer_values_indexes:
                new_position = round(new_position)
            elif i == learning_rate_idx:
                new_position = round(new_position, 3)
            elif i == dropout_idx:
                new_position = round(new_position, 2)

            if new_position > bounds[i][1]:
                new_position = bounds[i][1]

            if new_position < bounds[i][0]:
                new_position = bounds[i][0]

            self.positions[i] = new_position
