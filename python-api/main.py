import redis
from fastapi import FastAPI

app = FastAPI()
redis_client = redis.Redis(host="redis", port=6379, decode_responses=True)

@app.get("/price")
async def get_price():
    price = redis_client.get("btc_price")
    return {"btc_price": price}