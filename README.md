# The Machine Learning Toybox

A set of games designed for testing deep RL agents.

This is the public release repository. It contains:

1. The rust code for building Breakout.
2. The Python API to the Rust game.
3. The Python OpenAI Environment for training RL agents.

The current release *does not* contain the testing framework, nor the tests themselves. We will release the testing framework, tests, and a larger suite of games upon acceptance as an archival, peer-reviewed venue (i.e., conference proceedings or journal publication).

If you use this code, or otherwise are inspired by our white-box testing approach, please cite our [NeurIPS workshop paper](pubs/foley2018toybox.pdf):

```
@inproceedings{foley2018toybox,
  title={{Toybox: Better Atari Environments for Testing Reinforcement Learning Agents}},
  author={Foley, John J. and Tosch, Emma and Clary, Kaleigh and Jensen, David},
  booktitle={{NeurIPS 2018 Workshop on Critiquing and Correcting Trends in Machine Learning}},
  year={2018}
}
```

# Setup and Dependencies

## Target Rust Version

For building on older GPU clusters, we target rustc 1.28:
```bash
rustup override set 1.28.0
```

## Mac Dev Setup Instructions
* `brew install rustup`
* `rustup-init` with the default install
* clone this repo
* `source $HOME/.cargo/env`

## Lints and Formatting in Rust

The best rust tools require the nightly compiler (because they don't want to stabilize the compiler internals yet). Follow the readme instructions to get [rustfmt](https://github.com/rust-lang-nursery/rustfmt) and [clippy](https://github.com/rust-lang-nursery/rust-clippy).

Then you can check automatically format your files with ``cargo +nightly fmt`` and peruse the best lints with ``cargo +nightly clippy``.

A pre-commit hook will ensure that your code is always properly formatted. To do this, run

`git config core.hooksPath .githooks`

from the top-level directory. This will ensure that your files are formatted properly pior to committing.

## Python

Tensorflow, OpenAI Gym, OpenCV, and other libraries may or may not break with various Python versions. We have confirmed that the code in this repository will work with the following Python versions:

* 3.5
* 3.6

# Build

`cargo build`


# Developing New Games

## Get starting images for reference from ALE

`./scripts/utils/start_images --help` 
