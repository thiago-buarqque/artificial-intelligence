from sklearn.metrics import (
    mean_squared_error,
    mean_absolute_error
)

LOSS_FUNCTIONS = {
    'mse': mean_squared_error,
    'mae': mean_absolute_error
}
