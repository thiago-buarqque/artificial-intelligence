import copy
from functools import partial


class Toolbox:
    # Ok
    def generate_population(self, base_ind, pop_size, eval_func):
        pop = []

        for i in range(pop_size):
            new_ind = copy.deepcopy(base_ind)
            new_ind.random_init()
            new_ind.fitness_function = eval_func

            pop.append(new_ind)

        return pop

    # Ok
    def register(self, attr_name, function, *args, **kargs):
        pfunc = partial(function, *args, **kargs)
        pfunc.__name__ = attr_name
        pfunc.__doc__ = function.__doc__

        if hasattr(function, "__dict__") and not isinstance(function, type):
            pfunc.__dict__.update(function.__dict__.copy())

        setattr(self, attr_name, pfunc)
