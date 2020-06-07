# %%
import sys
import os

sys.path.append(os.getcwd() + "/..")

from keiba.utils import timer
from keiba.db import DB

db = DB()

df = db.get_df("SELECT * FROM race_results;")
