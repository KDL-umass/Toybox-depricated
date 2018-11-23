from abc import ABC, abstractmethod
from gym import Env, error, spaces, utils
from gym.spaces import np_random
from gym.utils import seeding
from toybox.envs.atari.constants import ACTION_MEANING, ACTION_LOOKUP

import numpy as np

class MockALE():
    def __init__(self, toybox):
        self.toybox = toybox

    def lives(self):
        return self.toybox.get_lives()

    def get_score(self):
        return self.toybox.get_score()

    def game_over(self):
        return self.toybox.game_over()

    def saveScreenPNG(self, name):
        # Has to be bytes for ALE
        name = name.decode('utf-8')
        grayscale = self.toybox.grayscale
        self.toybox.grayscale = False
        self.toybox.save_frame_image(name)
        self.toybox.grayscale = grayscale


class ToyboxBaseEnv(Env, ABC):
    metadata = {'render.modes': ['human']}
    
    def __init__(self, toybox, grayscale=True, alpha=False, actions=None):
        assert(toybox.rstate)
        self.toybox = toybox
        self.score = self.toybox.get_score()
        self.viewer = None

        # Required for compatability with OpenAI Gym's Atari wrappers
        self.np_random = np_random
        self.ale = MockALE(toybox)

        assert(actions is not None)
        self._action_set = actions
        self._obs_type = 'image'
        self._rgba = 1 if grayscale else 4 if alpha else 3
        self._pixel_high = 255

        self._height = self.toybox.get_height()
        self._width = self.toybox.get_width()
        self._dim = (self._height, self._width, self._rgba) # * len(self.toybox.get_state())) 
        
        self.reward_range = (0, float('inf'))
        self.action_space = spaces.Discrete(len(self._action_set))
        self.observation_space = spaces.Box(
            low=0, 
            high=self._pixel_high, 
            shape=self._dim, 
            dtype='uint8')
    
    @abstractmethod
    def _action_to_input(self, action):
        raise NotImplementedError
    
    def seed(self, seed=None):
        """
        This is totally the implementation in AtariEnv in openai/gym.
        """
        self.np_random, seed1 = seeding.np_random(seed)
        # Derive a random seed. This gets passed as a uint, but gets
        # checked as an int elsewhere, so we need to keep it below
        # 2**31.
        # Toybox takes a uint seed, but we're copying the ALE seed for reasons above. 
        # They're unclear who checks, so being safe here.
        seed2 = seeding.hash_seed(seed1 + 1) % 2**31
        self.toybox.set_seed(seed2)
        # Start a new game to ensure that the seed gets used!.
        self.toybox.new_game()
        return [seed1, seed2]

    # This is required to "trick" baselines into treating us as a regular Atari game
    # Implementation copied from baselines
    def get_action_meanings(self):
        #return [ACTION_MEANING[i] for i in self._action_set]
        return list(ACTION_MEANING.values())

    # From OpenAI Gym Baselines
    # https://github.com/openai/baselines/blob/master/baselines/common/atari_wrappers.py
    def _get_obs(self):
        return self.toybox.get_state()

    def step(self, action_index):
        obs = None
        reward = None
        done = False
        info = {}
    
        # Sometimes the action_index is a numpy integer...
        #print('Action index and type', action_index, type(action_index))
        #assert(type(action_index) == int)
        assert(action_index < len(self._action_set))
        assert(type(self._action_set)== list)
    
        # Convert the input action (string or int) into the ctypes struct.
        action = self._action_to_input(\
            self._action_set[int(action_index)])
        action.button1 = True
        frame = self.toybox.apply_action(action)
        obs = self._get_obs()
        
        
        # Compute the reward from the current score and reset the current score.
        score = self.toybox.get_score()
        reward = max(score - self.score, 0)
        self.score = score
    
        # Check whether the episode is done
        done = self.toybox.game_over()
    
        # Send back dignostic information
        info['lives'] = self.toybox.get_lives()
        #info['frame'] = frame
        info['score'] = 0 if done else self.score
    
        return obs, reward, done, info

    def reset(self):
        self.toybox.new_game()
        self.score = self.toybox.get_score()
        obs = self._get_obs()
        return obs

    def render(self, mode='human', close=False):
        if mode == 'human':
            # the following is copied from gym's AtariEnv
            if self.viewer is None:
                from gym.envs.classic_control.rendering import SimpleImageViewer
                self.viewer = SimpleImageViewer()
            self.viewer.imshow(self.toybox.get_rgb_frame())
            return self.viewer.isopen
        elif mode == 'rgb_array':
            return self.toybox.get_rgb_frame()

    def close(self):
        if self.viewer is not None:
            self.viewer.close()
        del self.toybox
        self.toybox = None
