"""
Toybox Python interface.

To get started,

```
from toybox.toybox import Toybox

with Toybox('breakout') as tb:
    print(tb.to_json())
```

"""

from gym.envs.registration import register
import toybox.toybox as toybox
import toybox.envs as envs

register(
    id='BreakoutToyboxNoFrameskip-v0',
    entry_point='toybox.envs.atari:BreakoutEnv',
    nondeterministic=False
)

register(
    id='AmidarToyboxNoFrameskip-v0',
    entry_point='toybox.envs.atari:AmidarEnv',
    nondeterministic=False
)

print("Loaded Toybox environments.")
