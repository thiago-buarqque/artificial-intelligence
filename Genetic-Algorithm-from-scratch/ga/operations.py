import copy
import numpy as np
import random
from operator import attrgetter


class Operations:
    def sl_tournament(self, pop, n, tourn_size=2):
        new_population = []
        for i in range(n):
            selected_individuals = np.random.choice(
                pop, size=tourn_size, replace=False)

            winner = max(selected_individuals,
                         key=lambda ind: ind.fitness if ind.fitness > ind.fitness else ind.fitness)
            new_population.append(copy.deepcopy(winner))

        return new_population

    def sl_roulette_wheel(self, pop, n):
        s_inds = sorted(pop, key=attrgetter("fitness"), reverse=True)
        sum_fits = sum(getattr(ind, "fitness") for ind in pop)
        chosen = []
        for i in range(n):
            u = random.random() * sum_fits
            sum_ = 0
            for ind in s_inds:
                sum_ += getattr(ind, "fitness")
                if sum_ > u:
                    chosen.append(copy.deepcopy(ind))
                    break

        return chosen

    def cs_one_point(self, ind1, ind2):
        cs_point = random.randint(1, len(ind1) - 1)

        ind1[cs_point:], ind2[cs_point:] = ind2[cs_point:], ind1[cs_point:]

        return ind1, ind2

    def cs_two_point(self, ind1, ind2):
        cs_point1 = random.randint(0, len(ind1.genes))
        cs_point2 = random.randint(0, len(ind1.genes) - 1)

        if cs_point2 >= cs_point1:
            cs_point2 += 1
        else:
            cs_point1, cs_point2 = cs_point2, cs_point1

        ind1.genes[cs_point1:cs_point2], ind2.genes[cs_point1:cs_point2] \
            = ind2.genes[cs_point1:cs_point2], ind1.genes[cs_point1:cs_point2]

        return ind1, ind2

    def cs_uniform(self, ind1, ind2):
        for i in range(len(ind1.genes)):
            if random.random() < 0.5:
                temp = ind1.genes[i]
                ind1.genes[i] = ind2.genes[i]
                ind2.genes[i] = temp
        return ind1, ind2
