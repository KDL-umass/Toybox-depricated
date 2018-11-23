======
Toybox
======

The Toybox python package provides an API for interacting with the Rust implementations of the Toybox game suite. Provided in this package is an implementation of an OpenAI environment that can be used as a drop-in replacement for the Arcade Learning Environment Breakout game.

# Installation from Source

This package uses a binary compiled from Rust. To build from source, first execute `cargo build --release` in this directory. 



In order to call toybox from python, you will first need to build the ctoybox library. This can be done from the top level of the repository either by executing `cargo build -p ctoybox` or from this directory by executing `cargo build`. The default setting for `cargo` is to build a debug version of the Rust executable. If you would like faster performance, you can run the build command with the `--release` option.

Executables live in the `target` directory of the top level of this repository. 

`toybox/toybox/toybox.py` is hard-coded to look up the compiled rust library for OSX. This is because different users' security settings may prevent using DYLD_LIBRARY_PATH on OSX. The equivalent environment variable for Linux is LD_LIBRARY_PATH. See https://github.com/jjfiv/toybox/blob/e62014ce067e598c5e0dd4819f2c78a9fc2ff027/openai/toybox/toybox/toybox.py#L7.


When running on Linux, you should be able to remove the path part of the Python code that loads in. The executable will have a different name, i.e., `libopenai.so`. When you run the Python code that uses this library, you can set LD_LIBRARY_PATH locally:
`LD_LIBRARY_PATH=<path_to_folder_containing_libopenai.so> python some_program_in_python.py`


Finally, compile the R code using `cargo build --release`.

## Using OpenAI Gym baselines right now

In this directory, checkout the baselines repo.

    git clone https://github.com/openai/baselines.git

Follow their instructions for installing dependencies. Then, run:

    ./start_python toybox_baselines.py --alg=acer --env=toybox-breakout-v0 --num_timesteps=10000 --save_path=$PWD/breakout1e4.model

FPS is quite low on my Air because tensorflow-CPU is slow on it.
