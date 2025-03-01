import gym
import torch
from stable_baselines3 import PPO

# Define a simple memory management environment
class MemoryEnv(gym.Env):
    def __init__(self):
        super(MemoryEnv, self).__init__()
        self.observation_space = gym.spaces.Box(low=0, high=100, shape=(1,), dtype=float) # Memory usage %
        self.action_space = gym.spaces.Discrete(3)  # Actions: Reduce, Maintain, Increase

    def step(self, action):
        reward = -abs(50 - self.state)  # Encourage staying near 50% usage
        self.state = min(max(self.state + (action - 1) * 10, 0), 100)
        return [self.state], reward, False, {}

    def reset(self):
        self.state = 50  # Start at 50% memory usage
        return [self.state]

# Train the model
env = MemoryEnv()
model = PPO("MlpPolicy", env, verbose=1)
model.learn(total_timesteps=10000)
model.save("memory_optimizer")