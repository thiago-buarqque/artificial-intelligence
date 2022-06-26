import numpy as np
import random

import statistics

from PSO.Particle import Particle


class PSO:
    def __init__(self,
                 c1,
                 c2,
                 velocities,
                 dimension,
                 swarm_size,
                 iterations,
                 bounds,
                 fitness_function):
        self.w = np.nan
        self.c1 = c1
        self.c2 = c2

        self.bounds = bounds
        self.velocities = velocities
        self.dimension = dimension
        self.swarm_size = swarm_size
        self.iterations = iterations
        self.g_best = []
        self.g_best_fitness = -1

        self.swarm = []

        self.fitness_function = fitness_function

    def initialize_swarm(self):
        integer_values_indexes = [0, 1, 2, 5]

        learning_rate_idx = 3
        dropout_idx = 4
        batch_size_idx = 5
        for i in range(self.swarm_size):
            p = Particle()
            for j in range(self.dimension):
                position = None

                if j == batch_size_idx:
                    position = random.choice(self.bounds[j])
                elif j in integer_values_indexes:
                    position = random.randint(self.bounds[j][0], self.bounds[j][1])
                elif j == learning_rate_idx:
                    position = random.uniform(self.bounds[j][0], self.bounds[j][1])
                    position = round(position, 3)
                elif j == dropout_idx:
                    position = random.uniform(self.bounds[j][0], self.bounds[j][1])
                    position = round(position, 2)

                p.positions.append(position)
                p.velocities.append(
                    random.uniform(self.velocities[j][0], self.velocities[j][1])
                )
            self.swarm.append(p)

    def linear_decay(self, current_iteration):
        w_max = 0.9
        w_min = 0.4
        return (w_max - w_min) * ((self.iterations - current_iteration) / self.iterations) + w_min

    def optimize(self, pool, w, logbook, hyper_id):
        self.swarm = []
        self.initialize_swarm()
        self.w = w

        for i in range(self.iterations):
            iteration_fitnesses = []

            fitnesses = list(pool.map(self.fitness_function, self.swarm))
            # Calculate fitness
            for j, particle_result in enumerate(fitnesses):
                self.swarm[j].fitness = particle_result[1]
                if particle_result[1] > self.swarm[j].fitness_pbest:
                    self.swarm[j].fitness_pbest = particle_result[1]
                    self.swarm[j].list_pbest = list(self.swarm[j].positions)

                if particle_result[1] > self.g_best_fitness:
                    self.g_best_fitness = particle_result[1]
                    self.g_best = list(self.swarm[j].positions)

                iteration_fitnesses.append(particle_result[1])
                logbook.register_kfold(
                    hyper_id=hyper_id,
                    iteration=i + 1,
                    particle=self.swarm[j],
                    kfold_data=particle_result[0]
                )

            # Calculate velocity and position
            for j in range(self.swarm_size):
                self.w = self.linear_decay(i)

                self.swarm[j].update_velocity(
                    g_best_positions=self.g_best,
                    dimension=self.dimension,
                    c1=self.c1, c2=self.c2,
                    bounds=self.bounds, w=self.w
                )
                self.swarm[j].update_position(self.bounds, self.dimension)

            standard_deviation = statistics.pstdev(iteration_fitnesses)

            _min = np.min(iteration_fitnesses)
            _avg = np.average(iteration_fitnesses)
            _max = np.max(iteration_fitnesses)

            logbook.update_logs(
                i + 1,
                _min=_min,
                avg=_avg,
                _max=self.g_best_fitness,
                std=standard_deviation
            )

            print(f'Iteration {i + 1} --'
                  f' Min: {_min} --'
                  f' Avg: {_avg} --'
                  f' Max: {self.g_best_fitness} --'
                  f' Std: {standard_deviation} --'
                  f' Gbest: {self.g_best}')
