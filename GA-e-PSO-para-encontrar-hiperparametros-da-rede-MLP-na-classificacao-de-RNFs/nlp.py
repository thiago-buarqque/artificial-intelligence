import pandas as pd
import re

# NLTK
from nltk.stem import WordNetLemmatizer
from nltk.corpus import stopwords

from keras.preprocessing.text import Tokenizer


def nlp():
    data = pd.read_csv('PROMISE_exp.csv')
    data.drop(data[data.Class == 'F'].index, inplace=True)

    # Convert text to lowercase
    data['RequirementText'] = data['RequirementText'].apply(lambda x: " ".join(x.lower() for x in x.split()))

    # Remove symbols
    data['RequirementText'] = data['RequirementText'].str.replace(r"[^\w\s]", " ")
    # Remove numbers
    data['RequirementText'] = data['RequirementText'].apply(lambda x: "".join(re.sub(r'\d+|', '', x)))

    # Remove stopwords
    stop = stopwords.words('english')
    data['RequirementText'] = data['RequirementText'].apply(lambda x: " ".join(x for x in x.split() if x not in stop))

    # Apply Lemmatization
    wordnet_lemmatizer = WordNetLemmatizer()
    data['RequirementText'] = data['RequirementText'].apply(
        lambda x: " ".join([wordnet_lemmatizer.lemmatize(word) for word in x.split()]))

    tokenizer = Tokenizer()

    tokenizer.fit_on_texts(data['RequirementText'])
    X = tokenizer.texts_to_matrix(data['RequirementText'], mode='tfidf')

    return X, data['Class']
