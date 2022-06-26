import multiprocessing

from nlp import nlp

from PSO.PSOHyperparametersTuning import PSOHyperparametersTuning
from GA.GAHyperparametersTuning import GAHyperparametersTuning


X, Y = nlp()
if __name__ == '__main__':
    ga_hyper = GAHyperparametersTuning(mutation_prob=0.1, population_size=30, generations=20)
    with multiprocessing.Pool(multiprocessing.cpu_count()) as pool:
        ga_hyper.run(pool, runs=30, tourn_size=2, X=X, Y=Y)

    print('\n\n---- End GA ----')

    pso_hyper = PSOHyperparametersTuning(X=X, Y=Y)
    with multiprocessing.Pool(multiprocessing.cpu_count()) as pool:
        pso_hyper.run(pool=pool, runs=30, swarm_size=20, iterations=30)
