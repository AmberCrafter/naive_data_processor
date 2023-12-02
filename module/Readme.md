# QC module

1. Stateless is the basic design patteren
2. Touch a file as cache to become statefull machine

## Protocol
1. Register function to config file
`module_type` support list:
 - Python


```toml
# Setup level x module, and the double square brackets are important.
[[level_x.module]]    
name = "xxx"
module_type = "python"
path = "xxx"
```



2. Write a module
```python
def run(level:int, datetime: str, data: any):
   ...
   return {
      "res": bool,
   }
```

```c
int run(level: int, datetime: *char, data: *char);
```

