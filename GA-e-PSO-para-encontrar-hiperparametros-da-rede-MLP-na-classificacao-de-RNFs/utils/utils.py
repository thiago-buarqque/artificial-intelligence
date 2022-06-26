import random
import numpy as np

import pandas as pd

from sklearn.model_selection import KFold
import keras.backend as k
from keras.callbacks import EarlyStopping

import tensorflow.keras
import keras
from tensorflow.keras import layers

from sklearn.metrics import f1_score

from sklearn.preprocessing import (LabelBinarizer, LabelEncoder)


def parameter_round(_min, _max, d=2):
    return round(random.uniform(_min, _max), d)


def make_log(data, dir_name, file_name, columns):
    new_data = pd.DataFrame(data=data, columns=columns)
    try:
        sheet = pd.read_excel(f'results/{dir_name}/{file_name}.xlsx', index_col=0)
        sheet = sheet.append(new_data)
    except FileNotFoundError:
        sheet = new_data

    sheet.to_csv(f'results/{dir_name}/{file_name}.csv', index=False)
    sheet.to_excel(f'results/{dir_name}/{file_name}.xlsx', sheet_name=f'{file_name}')


def train_model(
        x,
        y,
        opt_algorithm,
        hidden_function,
        hidden_size,
        learning_rate,
        dropout,
        batch_size,
):
    results = []

    le = LabelEncoder()
    lb = LabelBinarizer()

    # Label binarizer
    lb.fit(y)
    lb_Y = lb.transform(y)

    le.fit(y)
    le_Y = le.transform(y)

    kf = KFold(n_splits=10, shuffle=True)
    kf.get_n_splits(x)
    for train_index, test_index in kf.split(x):
        k.clear_session()

        fold_x_train, fold_x_valid = x[train_index], x[test_index]
        fold_y_train, fold_y_valid = lb_Y[train_index], lb_Y[test_index]
        fold_le_y_valid = le_Y[test_index]

        model = tensorflow.keras.models.Sequential()
        model.add(layers.Dense(hidden_size, input_dim=x.shape[1], activation=hidden_function))
        model.add(layers.Dropout(dropout))

        model.add(layers.Dense(11, activation='softmax'))

        callbacks_list = []

        es = EarlyStopping(monitor='val_loss', mode='min', verbose=0, patience=5, restore_best_weights=True)
        callbacks_list.append(es)

        optimizer = None
        if opt_algorithm == 'adam':
            optimizer = keras.optimizers.Adam(learning_rate=learning_rate)
        elif opt_algorithm == 'rmsprop':
            optimizer = keras.optimizers.RMSprop(learning_rate=learning_rate)
        elif opt_algorithm == 'adamax':
            optimizer = keras.optimizers.Adamax(learning_rate=learning_rate)

        model.compile(
            loss="categorical_crossentropy",
            optimizer=optimizer
        )

        model.fit(fold_x_train, fold_y_train,
                  epochs=50,
                  verbose=0,
                  batch_size=batch_size,
                  validation_split=0.111,
                  callbacks=callbacks_list)

        # predict = model.predict(x_test, verbose=0, batch_size=batch_size)
        predict = model(fold_x_valid)
        y_pred_bool = np.argmax(predict, axis=1)

        f1 = f1_score(fold_le_y_valid, y_pred_bool, zero_division=0, average='macro')
        results.append(f1)

    average = np.sum(results) / len(results)

    return [results, average]
