import toml

TEST_NAME="Boundary test"

def get_config(path):
    cfg = toml.load(path)
    return cfg

def get_level_config(config, level):
    cfg = config[f"level_{level}"]["module"]
    for ele in cfg:
        tmp = ele['name']
        if TEST_NAME==tmp:
            return ele

def run(level, datetime, data):
    print("Python: Hello Rust!")
    print(f"level: {level}, datetime: {datetime}, data: {data + 1}")
    return {
        # "res": 10,
        "res": False,
    }


if __name__ == "__main__":
    # tmp1 = pathlib.Path(__file__)
    # tmp2 = pathlib.Path("./module/boundary.py").absolute()
    # print(tmp1==tmp2)

    cfg = get_config("config/temperature.toml")

    # cfg = cfg[f"level_0"]["module"]
    cfg = get_level_config(cfg, 0)
    print(cfg)