from collections import deque
import ctypes
import numpy as np
from PIL import Image
import os
import platform
import time
import json

from toybox.clib import _lib, Input, NOOP, LEFT, RIGHT, UP, DOWN, BUTTON1, BUTTON2

class Simulator(object):
    def __init__(self, game_name):
        sim = _lib.simulator_alloc(game_name.encode('utf-8'))
        self.game_name = game_name
        self.__sim = sim
        self.__width = _lib.simulator_frame_width(sim)
        self.__height = _lib.simulator_frame_height(sim)
        self.deleted = False

    def __del__(self):
        if not self.deleted:
            self.deleted = True
            _lib.simulator_free(self.__sim)
            self.__sim = None

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        self.__del__()

    def set_seed(self, value):
        _lib.simulator_seed(self.__sim, value)

    def get_frame_width(self):
        return self.__width

    def get_frame_height(self):
        return self.__height

    def get_simulator(self):
        return self.__sim

    def new_game(self):
        return State(self)

    def from_json(self, js):
        if type(js) is dict:
            js = json.dumps(js)
        elif type(js) is not str:
            raise ValueError('Unknown json type: %s (only str and dict supported)' % type(js))
        state = _lib.from_json(self.get_simulator(), js.encode('utf-8'))
        return State(self, state=state)


class State(object):
    def __init__(self, sim, state=None):
        self.__state = state or _lib.state_alloc(sim.get_simulator())
        self.game_name = sim.game_name
        self.deleted = False

    def __enter__(self):
        return self

    def __del__(self):
        if not self.deleted:
            self.deleted = True
            _lib.state_free(self.__state)
            self.__state = None

    def __exit__(self, exc_type, exc_value, traceback):
        self.__del__()

    def get_state(self):
        return self.__state

    def lives(self):
        return _lib.state_lives(self.__state)

    def score(self):
        return _lib.state_score(self.__state)

    def game_over(self):
        return self.lives() == 0

    def render_frame(self, sim, grayscale=True):
        if grayscale:
            return self.render_frame_grayscale(sim)
        else:
            return self.render_frame_color(sim)

    def render_frame_color(self, sim):
        h = sim.get_frame_height()
        w = sim.get_frame_width()
        rgba = 4
        size = h * w  * rgba
        frame = np.zeros(size, dtype='uint8')
        frame_ptr = frame.ctypes.data_as(ctypes.POINTER(ctypes.c_uint8))
        _lib.render_current_frame(frame_ptr, size, False, sim.get_simulator(), self.__state)
        return np.reshape(frame, (h,w,rgba))

    def render_frame_rgb(self, sim):
        rgba_frame = self.render_frame_color(sim)
        return rgba_frame[:,:,:3]

    def render_frame_grayscale(self, sim):
        h = sim.get_frame_height()
        w = sim.get_frame_width()
        size = h * w
        frame = np.zeros(size, dtype='uint8')
        frame_ptr = frame.ctypes.data_as(ctypes.POINTER(ctypes.c_uint8))
        _lib.render_current_frame(frame_ptr, size, True, sim.get_simulator(), self.__state)
        return np.reshape(frame, (h,w,1))

    def to_json(self):
        json_str = _lib.to_json(self.__state).decode('utf-8')
        return json.loads(str(json_str))

class Toybox(object):
    def __init__(self, game_name, grayscale=True, frameskip=0):
        self.game_name = game_name
        self.frames_per_action = frameskip+1
        self.rsimulator = Simulator(game_name)
        self.rstate = State(self.rsimulator)
        self.grayscale = grayscale
        self.deleted = False

    def new_game(self):
        old_state = self.rstate
        del old_state
        self.rstate = self.rsimulator.new_game()

    def get_height(self):
        return self.rsimulator.get_frame_height()

    def get_width(self):
        return self.rsimulator.get_frame_width()

    def apply_action(self, action_input_obj):
        for _ in range(self.frames_per_action):
            _lib.state_apply_action(self.rstate.get_state(), ctypes.byref(action_input_obj))
        return self.rstate.render_frame(self.rsimulator, self.grayscale)

    def get_state(self):
        return self.rstate.render_frame(self.rsimulator, self.grayscale)

    def set_seed(self, seed):
        self.rsimulator.set_seed(seed)

    def save_frame_image(self, path, grayscale=False):
        img = None
        if grayscale:
            img = Image.fromarray(self.rstate.render_frame_grayscale(self.rsimulator), 'L')
        else:
            img = Image.fromarray(self.rstate.render_frame_color(self.rsimulator), 'RGBA')
        img.save(path, format='png')

    def get_rgb_frame(self):
        return self.rstate.render_frame_rgb(self.rsimulator)

    def get_score(self):
        return self.rstate.score()

    def get_lives(self):
        return self.rstate.lives()

    def game_over(self):
        return self.rstate.game_over()

    def to_json(self):
        return self.rstate.to_json()

    def from_json(self, js):
        return self.rsimulator.from_json(js)

    def write_json(self, js):
        old_state = self.rstate
        del old_state
        self.rstate = self.from_json(js)

    def predicate_met(self, pred):
        return False

    def __del__(self):
        if not self.deleted:
            self.deleted = True
            del self.rstate
            self.rstate = None
            del self.rsimulator
            self.rsimulator = None

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        self.__del__()


if __name__ == "__main__":
    with Toybox('breakout') as tb:
        print(tb.to_json())
