# Toybox Python Package


The Toybox python package provides an API for interacting with the Rust implementations of the Toybox game suite. Provided in this package is an implementation of an OpenAI environment that can be used as a drop-in replacement for the Arcade Learning Environment Breakout game.

## Installation from Source

This package uses a binary compiled from Rust. From this directory, run

`python3 setup.py install`

This will build the rust and install the python package. When python has finished building, it will print a message that looks like this:
```
Run:
        export LIBCTOYBOX=/path/to/the/repo
```

You will need to set this environment variable in order to use the python package.

### Note
`toybox/toybox/toybox.py` is hard-coded to look up the compiled rust library for OSX. This is because different users' security settings may prevent using DYLD_LIBRARY_PATH on OSX. The equivalent environment variable for Linux is LD_LIBRARY_PATH.

When running on Linux, you should be able to remove the path part of the Python code that loads in. The executable will have a different name, i.e., `libopenai.so`. When you run the Python code that uses this library, you can set LD_LIBRARY_PATH locally:
`LD_LIBRARY_PATH=<path_to_folder_containing_libopenai.so> python some_program_in_python.py`

## Using OpenAI Gym Baselines Right Now

With the toybox package installed and the LIBCTOYBOX environment set, you can now start using toybox in your local OpenAI baselines fork.

Navigate to your baselines directory. To check that toybox has installed properly, fire up a Python REPL and execute the following:

```
import toybox
from toybox.toybox import Toybox

with Toybox('breakout') as tb:
     print(tb.to_json())
```

If this script executes and prints out a wall of json, you are ready to modify your training script.

