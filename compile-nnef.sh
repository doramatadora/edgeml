#!/bin/bash
# Output a compiled NNEF from a Tensorflow graph.
# Not all models are supported! Check that all required operations are implemented first: https://github.com/sonos/tract#tensorflow

# 1. Download a model from: https://www.tensorflow.org/lite/guide/hosted_models
# 2. Expand the archive. 
# 3. Select the FROZEN TensorFlow model (_frozen.pb).
# 4. Run this script with the name of the .pb file as an argument.

FROZEN_GRAPH=$1
# N.B. You may need to change the input shape and types (--input).
# TensorFlow uses the NHWC convention, and the variant of Mobilenet picked for this example
# is a floating point model that operates on inputs of 224x224 pixels.
INPUT=f32

# $> tract dump --help

tract ./models/$FROZEN_GRAPH --input 1,224,224,3,$INPUT dump --nnef-tar ./models/compiled.nnef
# This will override the compiled model in the ./src.
