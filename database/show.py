import sqlite3
import datetime

ROOT = "database"
ROOT = "."

def dump_table(datatype: str):
    tablename = "{}Table".format(datatype.capitalize())
    
    query = f"""
        select * from {tablename};
    """

    print(query)
    conn = sqlite3.connect(f"{ROOT}/{datetime.datetime.now().strftime('%Y%m%d')}.db")
    cur = conn.cursor()
    cur.execute(query)
    data = cur.fetchall()

    print(data)

    

if __name__=="__main__":
    dump_table("float")
    dump_table("text")
    dump_table("integer")

