import numpy as np
import random
import copy

from deap import (base, creator, tools)

from utils.utils import (
    parameter_round,
)


def convert_population_to_list(pop):
    aux = []
    for c in pop:
        c_aux = []
        for p in c:
            c_aux.append(p)
        aux.append(c_aux)

    return aux


def separate_elite(fitnesses, pop, elite_size):
    elite = []
    max_indexes = np.array(fitnesses)
    indices = (-max_indexes).argsort()[:elite_size]
    indices = sorted(indices, reverse=True)
    aux_pop = copy.deepcopy(pop)

    for i in indices:
        elite.append(pop[i])
        # del aux_pop[i]

    return elite, aux_pop


def register_toolbox(fitness_function, tourn_size=2):
    # Hyperparameters range
    hidden_func = ['sigmoid', 'tanh', 'relu', 'elu']
    lower_nodes_hidden_layer1, upper_nodes_hidden_layer1 = 32, 1024
    lower_lr, upper_lr = 0.001, 0.1
    lower_dropout, upper_dropout = 0.1, 0.5
    opt_algorithm = ['adam', 'rmsprop', 'adamax']
    batch_size = [32, 64]

    def mutate(individual):
        gene = random.randint(0, 5)  # select which parameter to mutate

        if gene == 0:
            algorithm = random.choice(opt_algorithm)
            while algorithm == individual[0]:
                algorithm = random.choice(opt_algorithm)
            individual[0] = algorithm
        elif gene == 1:
            func = random.choice(hidden_func)
            while func == individual[1]:
                func = random.choice(hidden_func)
            individual[1] = func
        elif gene == 2:
            individual[2] = random.randint(lower_nodes_hidden_layer1, upper_nodes_hidden_layer1)
        elif gene == 3:
            individual[3] = round(random.uniform(lower_lr, upper_lr), 3)
        elif gene == 4:
            individual[4] = round(random.uniform(lower_dropout, upper_dropout), 2)
        elif gene == 5:
            if individual[5] == 32:
                individual[5] = 64
            else:
                individual[5] = 32

        return individual

    # Register hyperparamers and their ranges
    toolbox = base.Toolbox()
    toolbox.register("optimizer", random.choice, opt_algorithm)
    toolbox.register("hidden_func", random.choice, hidden_func)
    toolbox.register("hidden_size", random.randint, lower_nodes_hidden_layer1, upper_nodes_hidden_layer1)
    toolbox.register("learning_rate", parameter_round, lower_lr, upper_lr, d=3)
    toolbox.register("dropout", parameter_round, lower_dropout, upper_dropout, d=2)
    toolbox.register("batch_size", random.choice, batch_size)

    # Chromosome structure
    toolbox.register("individual", tools.initCycle, creator.Individual,
                     (toolbox.optimizer,
                      toolbox.hidden_func,
                      toolbox.hidden_size,
                      toolbox.learning_rate,
                      toolbox.dropout,
                      toolbox.batch_size), n=1)

    toolbox.register("population", tools.initRepeat, list, toolbox.individual)

    # Register crossover function
    toolbox.register("mate", tools.cxTwoPoint)
    # Register mutation function
    toolbox.register("mutate", mutate)
    # Register selection function
    toolbox.register("select", tools.selTournament, tournsize=tourn_size)
    # Register evaluation (fitness) function
    toolbox.register("evaluate", fitness_function)

    return toolbox
