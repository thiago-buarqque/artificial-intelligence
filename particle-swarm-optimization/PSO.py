import copy
from typing import Callable, Union

from Particle import Particle
from custom_types import Bounds, FitnessFunction


PlotFunction = Callable[[list[Particle], int], any]


class PSO:
    def __init__(self, use_weight_decay: bool, swarm_size: int,
                 positions_bounds: Bounds,
                 velocities_bounds: Bounds,
                 fitness_function: FitnessFunction,
                 c1: float = 2.0, c2: float = 2.0, w: float = 0.9):
        
        self.c1 = c1
        self.c2 = c2

        self.use_weight_decay = use_weight_decay
        self.w = w

        self.swarm: list[Particle] = []
        self.swarm_size = swarm_size

        self.g_best: Union[None, Particle] = None
        self.g_best_history: list[Particle] = []

        self.__generate_initial_swarm(
            positions_bounds, velocities_bounds, fitness_function
        )

    def optimize(self, 
                 iterations: int,
                 maximize: bool,
                 plot_function: Union[PlotFunction, None]):

        if plot_function is not None:
            plot_function(self.swarm, 0)

        for t in range(iterations):
            for particle in self.swarm:
                particle.calculate_fitness(maximize)

                if self.g_best is None or \
                        self.__is_better_than_g_best(maximize, particle):
                    self.g_best = copy.deepcopy(particle)

            for particle in self.swarm:
                inertia = self.__linear_decay(t, iterations) if \
                    self.use_weight_decay else self.w

                particle.update_velocities(c1=self.c1, c2=self.c2, w=inertia,
                                           g_best_positions=self.g_best.positions)
                particle.update_positions()

            self.g_best_history.append(self.g_best)

            print(f"{t}: Best fitness: {self.g_best.fitness} - best "
                  f"positions: {self.g_best.positions}")

            if plot_function is not None:
                plot_function(self.swarm, t+1)

    def __is_better_than_g_best(self, maximize: bool, particle: Particle):
        return (maximize is True and particle.fitness > self.g_best.fitness) \
               or (maximize is False and particle.fitness < self.g_best.fitness)

    def __generate_initial_swarm(self, 
                                 positions_bounds: Bounds,
                                 velocities_bounds: Bounds,
                                 fitness_function: FitnessFunction):
        
        for i in range(self.swarm_size):
            self.swarm.append(
                Particle(
                    dimensions=len(positions_bounds),
                    positions_bounds=positions_bounds,
                    velocities_bounds=velocities_bounds,
                    fitness_function=fitness_function
                )
            )

    def __linear_decay(self, current_iteration: int, total_iterations: int):
        w_max = 0.9
        w_min = 0.4

        return (w_max - w_min) * \
            ((total_iterations - current_iteration) / total_iterations) + w_min
