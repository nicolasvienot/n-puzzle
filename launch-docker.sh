#!/bin/sh

# This script will build an image for npuzzle and then run it with args "--help"
# You need docker to launch this script, check docker -v

docker build -t npuzzle . 
docker run --rm -it --init npuzzle --help

# You can then use docker run --rm -it --init npuzzle <args>