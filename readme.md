# Usage
build test 2

## Add QC config
[Global]
max_level = 1   // setup maximun level, start from 0

[options]       // add to each QC level
boundary = { min = 0.0, max = 50.0 }
consist = [{interval = 1, unit = "min", difference = 1.0}]

```
cd ~/config
touch {parameter}.toml
echo << EOF

[Global]
max_level = 1

[level_0]
boundary = { min = 0.0, max = 50.0 }

[level_1]
consist = [{interval = 1, unit = "min", difference = 1.0}]
EOF
```

## Start server
```
cargo run datetime,#{parameters_list}
```

## Client 
```
cargo run --bin client
```


Process flow
data -> parsing -> getconfig -> load module -> QC -> save data
