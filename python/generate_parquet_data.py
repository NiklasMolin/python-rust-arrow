""" This script generates a very basic parquet file
"""
import numpy as np
import pandas as pd
from faker.providers.person.en import Provider

def generate_file(output_file):
    df = pd.DataFrame()
    df['first_name'] = np.random.choice(getattr(Provider, "first_names"), size=1000000)
    df['last_name'] = np.random.choice(getattr(Provider, "last_names"), size=1000000)
    df['number'] = np.random.default_rng().random(size=1000000)
    df.to_parquet(output_file)

