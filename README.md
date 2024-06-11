# job server

This repository implements a rudimentary job server in Rust. The idea is to have
a pool of threads that listen for jobs. As soon as one becomes available, the
thread executes it. Once all jobs have been executed, cleanly exit.

The implementation is similar to a green threads style approach, with the
difference that state is associated with each job. Each job action can have
arbitrary arguments passed as inputs, allowing for context and state to be
managed and maintained externally, relative to the job.

I have primarily implemented this as an exercise in preparation for implementing
a compiler. It is designed so that additional jobs can be queued using closures,
which could enable a highly parallelized compilation process.

For example, compiling a single file might include multiple other files. The
process would be split up into logical steps, i.e. lexing, parsing, semantic
analysis, code generation, etc. These steps could be implemented using the job
server so that the process is parallelized and is stateful.

## maintenance

I have zero intention of maintaining this project. If you find it useful, great!
If you want to contribute, cool. Just know, I may not actually look at or do
anything with your contribution. I have licensed the project under the MIT
license, so feel free to use the code in any way you want. Which means, if you
want to turn it into something, feel free to make a fork and build to your
heart's intent.
