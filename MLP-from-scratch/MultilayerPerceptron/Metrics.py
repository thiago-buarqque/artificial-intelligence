from sklearn.metrics import (
    mean_squared_error,
    mean_absolute_error,
    accuracy_score,
    recall_score,
    f1_score,
)

METRICS = {
    'mse': mean_squared_error,
    'mae': mean_absolute_error,
    'accuracy': accuracy_score,
    'recall': recall_score,
    'f1': f1_score
}