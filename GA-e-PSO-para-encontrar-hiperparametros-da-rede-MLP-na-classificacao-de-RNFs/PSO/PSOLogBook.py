class PSOLogBook:
    def __init__(self):
        self.min = []
        self.avg = []
        self.max = []
        self.std = []

        self.logs = []
        self.logs_best_params = []

        self.kfolds = []

    def update_logs(self, i, _min, avg, _max, std):
        self.min.append(_min)
        self.avg.append(avg)
        self.max.append(_max)
        self.std.append(std)

        self.logs.append([
            i,
            _min,
            avg,
            _max,
            std
        ])

    def register_kfold(self, hyper_id, iteration, particle, kfold_data):
        self.kfolds.append(
            [
                hyper_id,
                iteration,
                particle.positions[0],
                particle.positions[1],
                particle.positions[2],
                particle.positions[3],
                particle.positions[4],
                particle.positions[5],
                kfold_data[0],
                kfold_data[1],
                kfold_data[2],
                kfold_data[3],
                kfold_data[4],
                kfold_data[5],
                kfold_data[6],
                kfold_data[7],
                kfold_data[8],
                kfold_data[9],
                particle.fitness
            ]
        )
