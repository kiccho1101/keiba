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

# %%
