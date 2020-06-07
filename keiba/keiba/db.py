# %%
import os
from dataclasses import dataclass
from dotenv import load_dotenv
import psycopg2
import pandas as pd

load_dotenv()


@dataclass
class DB:
    USERNAME: str = os.environ["POSTGRES_USER"]
    PASSWORD: str = os.environ["POSTGRES_PASSWORD"]
    HOSTNAME: str = os.environ["POSTGRES_HOST"]
    PORT: int = int(os.environ["POSTGRES_PORT"])
    DATABASE: str = os.environ["POSTGRES_DATABASE"]

    def exec_query(self, query: str):
        with psycopg2.connect(
            user=self.USERNAME,
            password=self.PASSWORD,
            host=self.HOSTNAME,
            port=self.PORT,
            database=self.DATABASE,
        ) as conn, conn.cursor() as cur:
            cur.execute(query)
            conn.commit()

    def get_df(self, query: str) -> pd.DataFrame:
        with psycopg2.connect(
            user=self.USERNAME,
            password=self.PASSWORD,
            host=self.HOSTNAME,
            port=self.PORT,
            database=self.DATABASE,
        ) as conn:
            df = pd.read_sql(query, conn)
            return df
