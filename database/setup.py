import sqlite3

ROOT = "database"
ROOT = "."

def create_table(datatype: str):
    tablename = "{}Table".format(datatype.capitalize())
    valuetype = datatype
    
    query = f"""
        create table if not exists {tablename} (
            id integer primary key autoincrement,
            datetime text,
            parameter text,
            value {valuetype},
            flag long
        );
    """

    print(query)
    conn = sqlite3.connect(f"{ROOT}/dummy.db")
    cur = conn.cursor()
    cur.execute(query)
    

if __name__=="__main__":
    create_table("float")
    create_table("text")
    create_table("integer")

