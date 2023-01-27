from fastapi import FastAPI
from pydantic import BaseModel
import sys, uvicorn
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI()

app.add_middleware(CORSMiddleware, allow_origins=["*"], allow_credentials=True, allow_methods=["*"], allow_headers=["*"])

class HelloRequest(BaseModel):
    name: str

class HelloResponse(BaseModel):
    message: str

@app.post("/hello")
async def hell_world(req: HelloRequest) -> HelloResponse:
    
    return HelloResponse(message=f"Hello from FastAPI, {req.name}")

if __name__ == "__main__":
    if getattr(sys, 'frozen', False) and hasattr(sys, '_MEIPASS'):
        uvicorn.run(app,port=8800)
    else:
        uvicorn.run("main:app",port=8800, reload=True, workers=2)