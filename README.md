# Analysis Tools Benchmark

Example code and output for popular static analysis tools.  
We want to add this output to the [website](https://analysis-tools.dev/) eventually.

## How to contribute

1. Pick any static analysis tool we did't cover yet. [Here's the full list](https://github.com/analysis-tools-dev/static-analysis).
2. Create a folder for your benchmark: `<language>/<tool-name>`.
3. Add the benchmark files. You need at least the following files:
   * `Dockerfile`: A self-contained environment for the tool and its dependencies
   * `Makefile`: Needs at least a `run` target, which will run the Docker image
   * Input file: this can be a source file or any other example input that you want to analyze.
   * `workflow.yml`: Describes the inputs and outputs of the benchmark.

You can take a look at the `example` folder to see what the final structure looks like.
