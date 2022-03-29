## How to run the smoke test

1. Run scylla
2. Set up virtualenv, e.g. in `~/local/venv` directory
3. Enter the virtual env via `. ~/local/venv/bin/activate`
4. Run:
 ```bash
  pip install --user maturin
  mkdir -p ~/local/venv
  virtualenv ~/local/venv
  . ~/local/venv/bin/activate
  maturin develop
 ```
5. Run the example python code - it works!
```bash
  python example.py
```

Here's the example code:
 ```python
import asyncio
from better_python_driver import Session, Cluster

async def run():
    print("Running better python driver smoke test")
    cluster = Cluster("127.0.0.1")
    session = await cluster.connect()
    res = await session.execute("SELECT keyspace_name, table_name FROM system_schema.tables")
    print(f"Result: {res}")

asyncio.run(run())
 ```
