# Design Goals of iobuf

The [Redpanda blog](https://redpanda.com/blog/tpc-buffers) says that iobuf is “Redpanda’s 0-copy buffer management for TpC”. Let’s summarize what they mean by that.

### Thread-per-core

Firstly, what even is TpC?

TpC is an programming model that address the two shortcomings of threaded programming:

- Threads executing on the same data requires synchronization mechanisms like locks, which are expensive.
- Context switching is required when a thread suspends itself and lets another thread run. Context switching is expensive.

In TpC, each application thread is pinned to a CPU. Since the OS cannot move the threads around, there are no context switches. Furthermore, the computer memory is sharded evenly across all cores. Each thread relies on message passing instead of shared memory to share information. This means that there are no locks in TpC.

To learn more about TpC, check out [this article by Seastar](https://seastar.io/shared-nothing/) or this [blog by Glommio](https://www.datadoghq.com/blog/engineering/introducing-glommio/).

### Sharing Data Between Cores

Redpanda is a Kafka-compatible, streaming system built on top of a TpC architecture. Since Redpanda is a streaming system, there could be many clients (producer or consumer) connected to it at any time. There is no guarantee that the clients are connected to the core that owns the data.

Therefore, Redpanda needs an efficient way to share a view of the parsed messages across cores. In addition, Redpanda wants to deallocate data when none of the cores need it. These two requirements gave birth to `iobuf`, a “ref-counted, fragmented-buffer-chain with deferred deletes”.

Now, let's look at how it works under the hood by examining my toy implementation!
