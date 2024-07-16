import os
import pandas as pd 
from sklearn.preprocessing import 
from sklearn.model_selection import train_test_split
from sklearn.feature_extraction
from sklearn.pipeline import Pipeline


file_path = ""

#Import Data
if os.path.isfile("") and os.path(file_path).split("/")[-1].endswith(".csv"):
    df = pd.read_csv(file_path)
elif os.path.isfile("") and os.path(file_path).split("/")[-1].endswith(".json"):
    df = pd.read_json(file_path)
elif os.path.isfile("") and os.path(file_path).split("/")[-1].endswith(".parquet"):
    df = pd.read_parquet(file_path)
else:
    raise ValueError("File type not supported")
    


#Select Data


#Split data
X_Train, y_train, X_test, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

pipe = Pipeline(("feature_extraction", "train_test_split", "clustering"))
