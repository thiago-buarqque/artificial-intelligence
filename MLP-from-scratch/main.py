import math

import pandas as pd

from MultilayerPerceptron.Layer import Layer
from MultilayerPerceptron.MLP import MLP
from MultilayerPerceptron.Neuron import Neuron
from MultilayerPerceptron.optimizers.RMSprop import RMSprop


def handle_nan_values(dataset, current_row_counter, new_row, backup_column):
    for i, row_column in enumerate(new_row):
        if math.isnan(row_column):
            sub_range = current_row_counter if i != len(
                new_row) - 1 else current_row_counter + len(new_row) - 1

            for j in reversed(range(sub_range)):
                if not math.isnan(float(dataset.iloc[[j]][backup_column])):
                    new_row[i] = float(dataset.iloc[[j]][backup_column])
                    break


def generate_dataset(dataset, desired_column, backup_column, period=5):
    j = period - 1

    result = []
    while (j + period) <= len(dataset) - 1:
        new_row = []
        for i in reversed(range(period)):
            new_row.append(float(dataset.iloc[[j - i]][desired_column]))

        new_row.append(float(dataset.iloc[[j + period]][desired_column]))

        handle_nan_values(dataset, j, new_row, backup_column)

        result.append(new_row)
        j += 1

    columns = []
    for i in range(period):
        if period - i - 1 == 0:
            columns.append("Dia(k)")
        else:
            columns.append(f"Dia(k-{period - i - 1})")

    columns.append("DiaObj")

    return pd.DataFrame(result, columns=columns)


if __name__ == '__main__':
    data = pd.read_csv('./acoes_bb_2020_2022.csv')

    period = 5
    acoes = generate_dataset(data, desired_column="Open",
                             backup_column="Close", period=period)

    real_world_data: [float] = acoes.tail((period * 2) - 1).pop(
        'DiaObj').values.tolist()

    x_train = acoes.sample(frac=0.9)
    x_test = acoes.drop(x_train.index)

    y_train = x_train.pop('DiaObj')
    y_test = x_test.pop('DiaObj')

    x_train = x_train.values.tolist()
    x_test = x_test.values.tolist()
    y_train = [[n] for n in y_train.values.tolist()]
    y_test = [[n] for n in y_test.values.tolist()]

    net = MLP(optimizer=RMSprop(lr=0.01))
    hidden_layer_1 = Layer(input_dim=len(x_train[0]), neurons=32,
                           activation_function="relu")
    output_layer = Layer(input_dim=32, neurons=1, activation_function="linear")

    net.add_layer(hidden_layer_1)
    net.add_layer(output_layer)

    net.optimize(x_train, y_train, epochs=50, metrics=['mae'])

    # Separa os últimos 9 dias para serem usados como entradas para prever os
    # próximos 5 dias
    last_n_days = []

    j = period - 1
    while j <= len(real_world_data) - 1:
        day = []
        for i in reversed(range(period)):
            day.append(float(real_world_data[j - i]))

        last_n_days.append(day)

        j += 1

    preds = net.predict(last_n_days)
    print(f"\nNetwork predictions: {preds}")
