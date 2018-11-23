from distutils.core import setup, Extension

import os
import subprocess

cwd = os.path.abspath(os.path.dirname(__file__))
builddir = os.path.abspath('..')
bindir = os.path.abspath('..' + os.path.sep + '..')

def generate_rust():
    print("Compiling rust")
    p = subprocess.call(['cargo', 'build', '--release'],
                         cwd=bindir)
    if p != 0:
        raise RuntimeError("Running Rust failed!")

generate_rust()
setup(
    name='Toybox',
    version='0.0.1',
    author='Emma Tosch',
    author_email='etosch@cs.umass.edu',
    packages=['toybox', 'toybox.envs', 'toybox.envs.atari'],
    license='LICENSE.txt',
    description='Toybox interface',
    long_description=open('README.md').read(),
)

print('Run:\n\texport LIBCTOYBOX=%s' % bindir)
