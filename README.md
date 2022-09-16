# Machine learning (ML) inference at the edge

It's [here](https://developer.fastly.com/solutions/demos/edgeml/)! 

This targets `wasm32-wasi` for [Fastly's Compute@Edge](https://www.fastly.com/products/edge-compute/serverless). It uses an extrinsic tool, [`wasm-opt`](https://github.com/WebAssembly/binaryen#tools), to squeeze - among other things – an ML [inference engine](https://en.wikipedia.org/wiki/Inference_engine) into a 35MB-ish [wasm](https://webassembly.org/) binary.

This demo showcases image classification using a [top-tier MobileNetV2 checkpoint](https://github.com/tensorflow/models/tree/master/research/slim/nets/mobilenet). Owing to the flexibility of [`tract`](https://github.com/sonos/tract) under the hood, the [TensorFlow Lite](https://www.tensorflow.org/lite/guide/hosted_model) model deployed can be swapped for another, including open interchange formats ([ONNX](https://onnx.ai/) / [NNEF](https://www.khronos.org/nnef)).

This demo was created to push the boundaries of the platform and inspire new ideas. 

## Publishing end-to-end

Using the Fastly CLI, publish the root package and note the `[funky-domain].edgecompute.app`:

```sh
fastly compute publish
```

Update L54 in [`docs/script.js`](./docs/script.js) to `[funky-domain].edgecompute.app` you just noted, and publish the static demo site separately:

```sh
cd static-host
fastly compute publish
```