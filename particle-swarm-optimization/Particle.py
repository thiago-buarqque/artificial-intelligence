from typing import Union, Callable

import numpy as np

from custom_types import Bounds


class Particle:
    def __init__(self, dimensions: int,
                 positions_bounds: Bounds,
                 velocities_bounds: Bounds,
                 fitness_function: Callable[[Union[list[int], list[float]]],
                                            Union[int, float]]):

        self.dimensions = dimensions

        self.velocities_bounds = velocities_bounds
        self.positions_bounds = positions_bounds
        self.p_best_velocities = np.zeros(dimensions)

        self.velocities = np.zeros(dimensions)
        self.positions = np.zeros(dimensions)
        self.p_best_positions = np.zeros(dimensions)

        self.p_best_fitness = 0
        self.fitness = 0
        self.fitness_function = fitness_function
        self.previous_fitnesses: Union[list[int], list[float]] = []

        self.__generate_initial_position()

    def calculate_fitness(self, maximize: bool):
        self.previous_fitnesses.append(self.fitness)

        self.fitness = self.fitness_function(self.positions)

        if self.__is_better_than_p_best(maximize):
            self.p_best_fitness = self.fitness
            self.p_best_positions = self.positions
            self.p_best_velocities = self.velocities

    def update_velocities(self, c1: float, c2: float, w: float,
                          g_best_positions: Union[list[int], list[float]]):
        for i in range(self.dimensions):
            swarm_influence = np.random.uniform() * c2 * (g_best_positions[i]
                                                          - self.positions[i])

            cognitive_influence = np.random.uniform() * c1 * (
                    self.p_best_positions[i] - self.positions[i])

            inertia = w * self.velocities[i]

            new_velocity = inertia + cognitive_influence + swarm_influence

            if new_velocity < self.velocities_bounds[i][0]:
                new_velocity = self.velocities_bounds[i][0]

            elif new_velocity > self.velocities_bounds[i][1]:
                new_velocity = self.velocities_bounds[i][1]

            self.velocities[i] = new_velocity

    def update_positions(self):
        for i in range(self.dimensions):
            new_position = self.positions[i] + self.velocities[i]

            if new_position < self.positions_bounds[i][0]:
                new_position = self.positions_bounds[i][0]

            elif new_position > self.positions_bounds[i][1]:
                new_position = self.positions_bounds[i][1]

            self.positions[i] = new_position

    def __is_better_than_p_best(self, maximize: bool):
        return (maximize is True and self.fitness > self.p_best_fitness) or \
               (maximize is False and self.fitness < self.p_best_fitness)

    def __generate_initial_position(self):
        for i, bounds in enumerate(self.positions_bounds):
            if type(self.positions_bounds) is list[list[int]]:
                self.positions[i] = np.random.randint(bounds[0], bounds[1])
            else:
                self.positions[i] = np.random.uniform(bounds[0], bounds[1])

    def __str__(self):
        return f"{self.fitness}"
