# %%
import sys
import os

sys.path.append(os.getcwd() + "/..")

from keiba.utils import timer
from keiba.db import DB

db = DB()

# %%
df = db.get_df(
    """
        SELECT
            results.*,
            races.*
        FROM
            race_results results
        JOIN
            races
        ON
            results.id = races.id;
    """
)
df["time_sec"] = (
    df["time"]
    .str.split(":", expand=True)
    .apply(lambda row: 60 * int(row[0]) + float(row[1]), axis=1)
)
df["time_sec_per_meter"] = df["time_sec"] / df["meter"]

# %%
import seaborn as sns

df[(df["meter"] == 2000)].sort_values("time_sec")[
    [
        "name",
        "race_type",
        "meter",
        "horse_name",
        "jockey_name",
        "time_sec",
        "time_sec_per_meter",
    ]
]
