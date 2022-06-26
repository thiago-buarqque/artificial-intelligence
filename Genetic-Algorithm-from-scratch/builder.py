import numpy as np

from ga.Individual import Individual
from ga.ga import GA
from ga.operations import Operations
from ga.toolbox import Toolbox


class Builder:
    def construct_ga_instance(self, base_ind, toolbox):
        print("\n**Now let's choose the parameters**\n")

        crossover_rate = float(input("Crossover rate: "))
        mutation_rate = float(input("Mutation rate: "))
        population_size = int(input("Population size: "))
        max_generations = int(input("Generations: "))
        elite_size = int(input("Elite size (0 if you doesn't want it): "))

        use_early_stop = input("Do you want to use early stopping (y/n): ")

        early_stop_size = None
        if use_early_stop == 'y':
            early_stop_size = int(input("How many generation: "))

        return GA(
            base_ind=base_ind,
            crossover_rate=crossover_rate,
            mutation_rate=mutation_rate,
            population_size=population_size,
            toolbox=toolbox,
            max_generations=max_generations,
            elite_size=elite_size,
            early_stop_strg=[early_stop_size, None]
        )

    def construct_toolbox(self, fitness_func, mutation_func, plot_fitness_func):
        operations = Operations()
        toolbox = Toolbox()

        print("\n**Now let's choose the genetic operations**\n")

        print("Which selection strategy do you want to use? ")
        print("1- Roulette wheel")
        print("2- Tournament of 'n' individuals")
        selection_strg = input(": ")

        if selection_strg == '2':
            tourn_size = int(
                input("\nHow many individual in each tournament? (default: 2): "))
            toolbox.register("select", operations.sl_tournament,
                             tourn_size=tourn_size)
        else:
            toolbox.register("select", operations.sl_roulette_wheel)

        print("\n\nWhich crossover strategy do you want to use? (default: one point): ")
        print("1- One point")
        print("2- Two points")
        print("3- Uniform")
        crossover_strg = input(": ")

        crossover_func = operations.cs_one_point
        if crossover_strg == '2':
            crossover_func = operations.cs_two_point
        elif crossover_strg == '3':
            crossover_func = operations.cs_uniform

        toolbox.register("crossover", crossover_func)
        toolbox.register("evaluate", fitness_func)
        toolbox.register("mutate", mutation_func)
        toolbox.register("plot_eval_func", plot_fitness_func)

        return toolbox

    def construct_base_individual(self):
        base_ind = Individual()

        print("\n\n**Let's define the function's variables**." +
              "\n\nReminder: remember to check the math so your don't accidentally divide by 0.\n")

        while(True):
            print(
                "\nType 'continue' any time when you finish adding all function's variables\n")
            var_name = input("Type the var name: ")
            if var_name == 'continue':
                break

            var_lower_bound = input("Type the var lower bound: ")
            if var_lower_bound == 'continue':
                break

            var_upper_bound = input("Type the var upper bound: ")
            if var_upper_bound == 'continue':
                break

            var_type = input("Is this var decimal or integer? ")
            if var_type == 'continue':
                break

            gen_func = None
            if var_type == 'integer':
                gen_func = np.random.randint
                var_lower_bound = int(var_lower_bound)
                var_upper_bound = int(var_upper_bound)
            else:
                gen_func = np.random.uniform
                var_lower_bound = float(var_lower_bound)
                var_upper_bound = float(var_upper_bound)

            base_ind.register(var_name, gen_func,
                              var_lower_bound, var_upper_bound)

        return base_ind

    def construct_fitness_function(self):
        eggholder_func = "-((x2 + 47) * math.sin(math.sqrt(abs((x1 / 2) + (x2 + 47))))) - (x1 * math.sin(math.sqrt(abs(x1 - (x2 + 47)))))"
        thiagos_func = "1 - (abs((math.sqrt(x1) * math.sin(x2**(math.sqrt(x3)))))) - ((x5**2)/x4) + (1/math.sqrt(x5))"
        valley_func = "((x1 + 2*x2 - 7)**2) + ((2*x1 + x2 - 5)**2)"

        print("**Let's begin bulding/choosing the fitness function**\n\n")

        print("You can type your own math function or use one of the following\n")
        print("1- Eggholder function: -((x2 + 47) * math.sin(math.sqrt(abs((x1 / 2) + (x2 + 47))))) - (x1 * math.sin(math.sqrt(abs(x1 - (x2 + 47)))))")
        print("2- Thiago's function: 1 - (abs((math.sqrt(x1) * math.sin(x2**(math.sqrt(x3)))))) - ((x5**2)/x4) + (1/math.sqrt(x5))")
        print("3- Valley function: ((x1 + 2*x2 - 7)**2) + ((2*x1 + x2 - 5)**2)\n")

        print("Type your own function or choose one of the above.")
        print("For your own function you can use Python built-in function (like abs()), math and numpy (np).")

        in_func = input(
            ": ")

        if in_func == '1':
            in_func = compile(eggholder_func, 'input', 'eval')
        elif in_func == '2':
            in_func = compile(thiagos_func, 'input', 'eval')
        elif in_func == '3':
            in_func = compile(valley_func, 'input', 'eval')
        else:
            in_func = compile(in_func, 'input', 'eval')

        return in_func
