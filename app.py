import asyncio

async def main():
    print('Hello ...')
    await asyncio.sleep(1)
    print('... World!')
   

# Python 3.7+
asyncio.run(main())

# def huge_data():
#     for i in (100_000_000_000):
