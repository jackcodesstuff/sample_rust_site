import gym
import numpy as np
from gym import spaces
from stable_baselines3 import PPO
from stable_baselines3.common.vec_env import DummyVecEnv

# Define a proper Gymnasium environment
class YourCustomMemoryEnv(gym.Env):
    def __init__(self):
        super(YourCustomMemoryEnv, self).__init__()

        # Define action and observation space
        # Example: action space with discrete values (0, 1, 2)
        self.action_space = spaces.Discrete(3)
        
        # Example: observation space is a single floating-point value between 0 and 1
        self.observation_space = spaces.Box(low=0, high=1, shape=(1,), dtype=np.float32)
        
        self.state = np.array([0.5], dtype=np.float32)  # Example initial state

    def reset(self):
        self.state = np.array([0.5], dtype=np.float32)  # Reset to initial state
        return self.state

    def step(self, action):
        # Define how the environment responds to actions
        reward = 1.0 if action == 1 else -1.0  # Example reward function
        done = False  # Set True if episode should end
        return self.state, reward, done, {}

# Wrap environment in DummyVecEnv
env = DummyVecEnv([lambda: YourCustomMemoryEnv()])

# Train PPO model
print("Training PPO model...")
model = PPO("MlpPolicy", env, verbose=1)
model.learn(total_timesteps=10000)

# Save model
model.save("memory_optimizer.zip")
print("Model saved as memory_optimizer.zip")