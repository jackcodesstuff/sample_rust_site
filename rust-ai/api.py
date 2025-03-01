from fastapi import FastAPI
import numpy as np
from stable_baselines3 import PPO
from pydantic import BaseModel
from typing import List

class StateInput(BaseModel):
    states: List[float]

class PredictRequest(BaseModel):
    states: list[float]

app = FastAPI()

# ✅ Load trained model
model = PPO.load("memory_optimizer.zip")

@app.get("/predict")
def predict_memory(state: float):
    action, probs = model.predict(np.array([state], dtype=np.float32), deterministic=False)
    print(probs)  # Debug output
    print(type(probs))
    if probs is None:
        raise ValueError("Model returned None. Check the model input format and validity.")
    return {
        "recommended_action": int(action),
        "probabilities": probs.tolist()  # Convert NumPy array to list
    }

async def predict_batch(data: PredictRequest):
    try:
        states = np.array(data.states)
        # Replace with your model prediction logic
        probs = model.predict(states)  # Ensure `model.predict` is valid
        return {"probabilities": probs.tolist()}
    except Exception as e:
        return {"error": str(e)}