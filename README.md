Git plays a vital role in the in the daily workflow of software developers. However, Git uses a single, centralized, monolithic in-memory index, which becomes a bottleneck for cold operations on very large working trees.
So here it come, ShardVCS having sharded index. I benchmarked it against Git on synthetic monorepos up to 100k files. My VCS has sharded index and when benchmarked shows better results.
As a trade-off, the sharded index currently introduces higher overhead during initial staging (add) operations, particularly in freshly initialized repositories.

This project is still in active development. Ongoing work includes further index optimizations, feature additions, and more extensive benchmarking to improve performance and validate correctness across different repository sizes and workloads.
