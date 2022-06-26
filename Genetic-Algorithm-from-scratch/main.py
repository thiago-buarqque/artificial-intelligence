import math
import numpy as np
from builder import Builder


user_func = None


def fitness_func(func_vars):
    # Func vars had a type of {"var_name": var, ...}
    func_vars['np'] = np
    func_vars['math'] = math

    return eval(user_func, func_vars)


def plot_fitness_func(x1, x2):
    # Func vars had a type of {"var_name": var, ...}
    func_vars = {'x1': x1, 'x2': x2}
    func_vars['np'] = np
    func_vars['math'] = math

    return eval(user_func, func_vars)


def mutation_func(ind):
    gene_i = np.random.randint(low=0, high=len(ind.genes))

    lower_bound = ind.complex_genes[gene_i]['lower_bound']
    upper_bound = ind.complex_genes[gene_i]['upper_bound']

    ind.genes[gene_i] = ind.complex_genes[gene_i]["gen_func"](
        low=lower_bound, high=upper_bound)


if __name__ == '__main__':
    builder = Builder()

    user_func = builder.construct_fitness_function()
    base_ind = builder.construct_base_individual()
    toolbox = builder.construct_toolbox(
        fitness_func, mutation_func, plot_fitness_func)
    ga = builder.construct_ga_instance(base_ind, toolbox)

    ga.optimize()

    while True:
        print("\n\What do you want to do?\n")

        print("0- Exit")
        print("1- Plot convergence curve")
        print("2- Plot min, avg and max of each generation")
        print("3- Plot 3d graph with the best solution found (2 dimensional functions only)")

        choose = input(": ")

        if choose == '0':
            break
        elif choose == '1':
            ga.plot_convergence_curve()
        elif choose == '2':
            ga.plot_min_avg_max()
        elif choose == '3':
            ga.plot_3d_fitness_function()
        else:
            print("\nWrong option. Try again!")
