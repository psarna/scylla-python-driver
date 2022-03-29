## How to run the smoke test

1. Run scylla
2. Set up virtualenv, e.g. in `~/local/venv` directory
3. Enter the virtual env via `. ~/local/venv/bin/activate`
3. Run:
 ```bash
  pip install --user maturin
  mkdir -p ~/local/venv
  VIRTUALENV=~/local/venv maturin develop
 ```
4. Run the example python code which calls Rust bindings - it works!
 ```python
import asyncio
import better_python_driver

async def run():
    print("Running better python driver smoke test")
    x = await better_python_driver.smoke_test()
    print(f"Result: {x}")

asyncio.run(run())
 ```
