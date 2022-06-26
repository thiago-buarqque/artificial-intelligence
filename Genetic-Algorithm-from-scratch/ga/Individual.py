class Individual:
    def __init__(self):
        self.genes = []
        self.complex_genes = []
        self.fitness = None

        self.fitness_function = None

    def evaluate(self):
        fit_func_vars_dict = {}
        for i, complex in enumerate(self.complex_genes):
            fit_func_vars_dict[complex["attr_name"]] = self.genes[i]

        self.fitness = self.fitness_function(fit_func_vars_dict)

    def register(self, attr_name, gen_function, lower_bound, upper_bound, *args):
        self.complex_genes.append({
            "attr_name": attr_name,
            "gen_func": gen_function,
            "lower_bound": lower_bound,
            "upper_bound": upper_bound,
            "args": args
        })

    def random_init(self):
        self.genes = []
        self.fitness = None

        for complex in self.complex_genes:
            self.genes.append(complex["gen_func"](
                complex["lower_bound"], complex["upper_bound"]))

    def __str__(self):
        return f"{self.genes}"
