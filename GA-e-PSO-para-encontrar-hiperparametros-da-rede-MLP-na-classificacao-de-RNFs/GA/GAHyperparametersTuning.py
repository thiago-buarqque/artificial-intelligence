import random
import numpy as np

from deap import (base, creator, tools)

from GA.ga_utils import register_toolbox, convert_population_to_list, separate_elite
from utils.utils import (
    make_log,
    train_model
)


class GAHyperparametersTuning:
    def __init__(self, crossover_prob=0.95, mutation_prob=0.01, generations=20, population_size=30):
        creator.create("FitnessMax", base.Fitness, weights=(1.0,))
        creator.create("Individual", list, fitness=creator.FitnessMax)

        self.generations = generations
        self.population_size = population_size

        # cs_pb crossover probability
        # mut_pb mutation probability
        self.cs_pb, self.mut_pb = crossover_prob, mutation_prob

        # Saves all info from all hyperparameters tuning
        self.logs_ga = []

        self.all_kfolds = []

        self.X = []
        self.Y = []

    def fitness_func(self, individual):
        X = self.X
        Y = self.Y

        opt_algorithm = individual[0]
        hidden_func = individual[1]
        hidden_size = individual[2]
        learning_rate = individual[3]
        dropout = individual[4]
        batch_size = individual[5]

        return train_model(
            X,
            Y,
            opt_algorithm,
            hidden_func,
            hidden_size,
            learning_rate,
            dropout,
            batch_size,
        )

    def run(self, pool, tourn_size, X, Y, runs=30, elite_percentage=0.1):
        self.X = X
        self.Y = Y

        for i in range(runs):
            print(f'\n\n-- Hyperparameters tuning {i + 1} --\n\n')

            toolbox = register_toolbox(fitness_function=self.fitness_func, tourn_size=tourn_size)

            pop = toolbox.population(n=self.population_size)

            # Evaluate the entire population
            # Convert chromosomes to lists (it's in Deap individual format) to use on multicore processing
            aux_pop = convert_population_to_list(pop)
            fitnesses = list(pool.map(self.fitness_func, aux_pop))

            for ind, fit in zip(pop, fitnesses):
                ind.fitness.values = [fit[1]]

                self.save_kfold(hyper_id=i + 1, iteration=0, ind=ind, fit=fit)

            stats = tools.Statistics(lambda indi: indi.fitness.values)
            stats.register("avg", np.mean)
            stats.register("std", np.std)
            stats.register("min", np.min)
            stats.register("max", np.max)

            hof = tools.HallOfFame(1)
            hof.update(population=pop)

            logbook = tools.Logbook()
            logbook.header = ['gen', 'nevals'] + (stats.fields if stats else [])

            record = stats.compile(pop) if stats else {}
            logbook.record(gen=0, nevals=len(pop), **record)
            print(logbook.stream)

            generation = 0
            while generation < self.generations:
                generation = generation + 1

                # aux_pop -> Population cromossomes minus the elite
                elite, aux_pop = separate_elite([ind.fitness.values[0] for ind in pop], pop,
                                                elite_size=int(elite_percentage * self.population_size))

                # Select the next generation individuals
                offspring = toolbox.select(aux_pop, len(aux_pop) - (int(elite_percentage * self.population_size)))

                # Clone the selected individuals
                offspring = list(map(toolbox.clone, offspring))

                # Apply crossover
                for child1, child2 in zip(offspring[::2], offspring[1::2]):
                    if random.random() < self.cs_pb:
                        toolbox.mate(child1, child2)

                        del child1.fitness.values
                        del child2.fitness.values

                # Apply mutation
                for mutant in offspring:
                    if random.random() < self.mut_pb:
                        toolbox.mutate(mutant)
                        del mutant.fitness.values

                offspring = offspring + elite

                # Evaluate the individuals with an invalid fitness
                invalid_ind = [ind for ind in offspring if not ind.fitness.valid]

                # Convert chromosomes to lists (it's in Deap individual format) to use on multicore processing
                aux_invalid_ind = convert_population_to_list(invalid_ind)

                fitnesses = pool.map(self.fitness_func, aux_invalid_ind)
                for ind, fit in zip(invalid_ind, fitnesses):
                    ind.fitness.values = [fit[1]]
                    self.save_kfold(hyper_id=i + 1, iteration=generation, ind=ind, fit=fit)

                # Update hall of fame
                hof.update(offspring)

                # Update population
                pop[:] = offspring

                record = stats.compile(pop) if stats else {}
                logbook.record(gen=generation, nevals=len(invalid_ind), **record)

                self.logs_ga.append([
                    generation,
                    logbook.select('min')[generation],
                    logbook.select('avg')[generation],
                    logbook.select('max')[generation],
                    logbook.select('std')[generation]])

                print(logbook.stream)

            self.logs_ga.append(['-----', '-----', f'End hyper #{i + 1}', '-----', '-----'])
            self.save_results()

    def save_kfold(self, hyper_id, iteration, ind, fit):
        self.all_kfolds.append(
            [hyper_id,
             iteration,
             ind[0],  # Optimizer
             ind[1],  # Hidden func
             ind[2],  # Hidden size
             ind[3],  # Lr
             ind[4],  # Dropout
             ind[5],  # Batch size
             fit[0][0],
             fit[0][1],
             fit[0][2],
             fit[0][3],
             fit[0][4],
             fit[0][5],
             fit[0][6],
             fit[0][7],
             fit[0][8],
             fit[0][9],
             fit[1]
             ]
        )

    def save_results(self):
        # Save GA logs
        make_log(
            data=self.logs_ga,
            dir_name='GA',
            file_name='logs_ga',
            columns=['Gen', 'Min', 'Avg', 'Max', 'Std']
        )

        # Save all combinations found by GA and their respective k-fold result
        make_log(
            data=self.all_kfolds,
            dir_name='GA',
            file_name='logs_all_kfolds_ga',
            columns=['hyperId',
                     'Generation',
                     'Optimizer',
                     'Hidden function',
                     'Hidden size',
                     'Learning rate',
                     'Dropout',
                     'Batch size',
                     'k-fold-1',
                     'k-fold-2',
                     'k-fold-3',
                     'k-fold-4',
                     'k-fold-5',
                     'k-fold-6',
                     'k-fold-7',
                     'k-fold-8',
                     'k-fold-9',
                     'k-fold-10',
                     'Fitness'])

        self.logs_ga = []
        self.all_kfolds = []
