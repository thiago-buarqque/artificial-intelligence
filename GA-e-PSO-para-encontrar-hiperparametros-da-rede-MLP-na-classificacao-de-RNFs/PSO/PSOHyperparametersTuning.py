from utils.utils import (
    make_log,
    train_model
)

from PSO.PSO import PSO

from PSO.PSOLogBook import PSOLogBook


class PSOHyperparametersTuning:
    def __init__(self, X, Y):
        self.logs = []
        self.all_kfolds = []
        self.X = X
        self.Y = Y

    def fitness_func(self, particle):
        x_data = self.X
        y_data = self.Y

        hidden_functions = ['sigmoid', 'tanh', 'relu', 'elu']
        optimizers = ['adam', 'rmsprop', 'adamax']

        opt_algorithm = optimizers[particle.positions[0] - 1]
        hidden_function = hidden_functions[particle.positions[1] - 1]
        hidden_size = particle.positions[2]
        learning_rate = particle.positions[3]
        dropout = particle.positions[4]
        batch_size = particle.positions[5]

        return train_model(
            x_data,
            y_data,
            opt_algorithm,
            hidden_function,
            hidden_size,
            learning_rate,
            dropout,
            batch_size,
        )

    def run(self, pool, runs=30, swarm_size=20, iterations=30):
        # 0- Optimizer ->  ['adam', 'rmsprop', 'adamax']
        # 1- Hidden_function -> ['sigmoid', 'tanh', 'relu', 'elu']
        # 2- Hidden size
        # 3- Learning_rate
        # 4- Dropout
        # 5- Batch_size
        velocities = [
            [-1, 1.5],
            [-2, 2],
            [-512, 512],
            [-0.05, 0.05],
            [-0.25, 0.25],
            [-1, 1]
        ]
        bounds = [
            [1, 3],
            [1, 4],
            [32, 1024],
            [0.001, 0.1],
            [0.1, 0.5],
            [32, 64]
        ]

        hyper_id = 0
        for i in range(runs):
            print(f'\n\n-- Hyperparameters tuning #{hyper_id} --\n\n')

            logbook = PSOLogBook()

            _pso = PSO(c1=2,
                       c2=2,
                       velocities=velocities,
                       dimension=6,
                       swarm_size=swarm_size,
                       iterations=iterations,
                       bounds=bounds,
                       fitness_function=self.fitness_func
                       )

            _pso.optimize(
                w=0.9,
                logbook=logbook,
                hyper_id=hyper_id,
                pool=pool
            )

            if len(self.all_kfolds) == 0:
                self.all_kfolds = logbook.kfolds
            else:
                self.all_kfolds.extend(logbook.kfolds)

            self.logs.extend(logbook.logs)

            self.logs.append(['-----', '-----', f'End hyper #{hyper_id}', '-----', '-----'])
            self.save_results()
            hyper_id += 1

    def save_results(self):
        # Save pso logs
        make_log(
            data=self.logs,
            dir_name='PSO',
            file_name='logs_pso',
            columns=['Ite', 'Min', 'Avg', 'Max', 'Std']
        )

        # Save all combinations found by PSO and their respective k-fold result
        make_log(
            data=self.all_kfolds,
            dir_name='PSO',
            file_name='logs_all_kfolds_pso',
            columns=['HyperId',
                     'Iteration',
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

        self.logs = []
        self.all_kfolds = []
