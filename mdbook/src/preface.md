# Motivation

In this blog, we will reimplement Redpanda's `iobuf` library as introduced in their [blog post](https://redpanda.com/blog/tpc-buffers).

To understand `iobuf`, we need to first understand Redpanda's threading and memory model. Redpanda uses a [thread-per-core (TpC) architecture](https://www.datadoghq.com/blog/engineering/introducing-glommio/). TpC is an programming model that address the two shortcomings of threaded programming:

- Threads executing on the same data requires synchronization mechanisms like locks, which are expensive.
- Context switching is required when a thread suspends itself and lets another thread run. Context switching is expensive.

In TpC, each application thread is pinned to a CPU. Since the OS cannot move the threads around, there are no context switches. Furthermore, under TpC each thread relies on message passing instead of shared memory to share data. This elimitates synchronization overheads from locks. To learn more about TpC, check out [this article by Seastar](https://seastar.io/shared-nothing/) or this [blog by Glommio](https://www.datadoghq.com/blog/engineering/introducing-glommio/).

Under [Seastar](https://seastar.io/), Redpanda's TpC framework, the full memory of the computer is split evenly across the cores during system bootup. As stated in Redpanda's [blog](https://redpanda.com/blog/tpc-buffers), "memory allocated on core-0 [core-N], *must* be deallocated on core-0 [core-N]. However, there is no way to guarantee that a Java or Go client connecting to Redpanda will actually communicate with the exact core that owns the data". Therefore, Redpanda created `iobuf`, "a ref-counted, fragmented-buffer-chain with deferred deletes that allows Redpanda to simply share a view of a remote core’s parsed messages as the fragments come in, without incurring a copy overhead". In other words, `iobuf` is a way for Redpanda to share data across cores cheaply and deallocate the data in the core that owns the data when no cores need that data.

Now, let's look at how it works under the hood by examining my toy implementation!
