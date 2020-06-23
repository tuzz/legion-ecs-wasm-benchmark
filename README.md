## Legion WASM Benchmark

This is a test page that can be used to measure how long it takes
[Legion](https://github.com/tomGillen/legion) to create some entities and add
components for a varying number of entities. I created this page because I'm
seeing very different times between my 13" MacBook Pro 2017 (2.5GHz i7) and my
11" iPad Pro 2018. You can run the benchmark
[here](https://tuzz.github.io/legion-ecs-wasm-benchmark/).

On the [PSPDFKit benchmark](https://pspdfkit.com/webassembly-benchmark/) my
scores are:

- MacBook Pro: 2512
- iPad Pro: 1500

This implies the iPad Pro is faster at WASM as a lower score is better.

On this benchmark my combined times for ~2 million entities are:

- MacBook Pro: ~118ms
- iPad Pro: ~2676ms

In this benchmark, the iPad is ~23x slower than the MacBook so I've opened [an
issue](https://github.com/TomGillen/legion/issues/158) on the Legion ECS
project to try and understand why this is. For anyone wondering, this page was
created using my [minimal-rust-wasm](https://github.com/tuzz/minimal-rust-wasm)
template.
