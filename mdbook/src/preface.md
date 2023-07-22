# Motivation

I recently came across the [thread-per-core (TpC) architecture](https://www.datadoghq.com/blog/engineering/introducing-glommio/). TpC is an architecture in which each application thread is pinned to a specific CPU and uses message passing instead of message sharing to share data. This architecture removes the need for expensive synchronization mechanisms such as locks and eliminates costly context switches.

I had some questions about memory management in TpC. If a core wants to share data with another core, does the source core need to copy data to the destination core each time? Isn’t that expensive?

This [engineering blog](https://redpanda.com/blog/tpc-buffers) written by Redpanda answered my question. In the blog, Alexander Gallego talks about buffer management in a TpC environment. He introduced `iobuf`, Redpanda’s 0-copy buffer management for TpC which allows cores to share data without incurring a copy overhead.

To understand `iobuf` better, I studied `iobuf`'s source code and built a toy version of it in Rust. Full source code for my toy implementation, `mini-iobuf`, is available [here](https://github.com/brianshih1/mini-iobuf).

Also let me preface this by saying that I am a novice in the world of systems programming. So please let me know if I’m making any incorrect statements!
