import asyncio
from scylla.cluster import Cluster

async def run():
    print("Running scylla python driver smoke test")
    cluster = Cluster(["127.0.0.1"])
    session = await cluster.connect_async()
    res = await session.execute_async("SELECT keyspace_name, table_name FROM system_schema.tables")
    for row in res:
        print(row)

asyncio.run(run())
