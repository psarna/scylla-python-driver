import asyncio
from better_python_driver import Session, Cluster

async def run():
    print("Running better python driver smoke test")
    cluster = Cluster(["127.0.0.1"])
    session = await cluster.connect()
    res = await session.execute("SELECT keyspace_name, table_name FROM system_schema.tables")
    print(f"Result: {res}")

asyncio.run(run())
