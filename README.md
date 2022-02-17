# cache-oblivious

**Status:** This is a personal project for learning that's not well-tested and
probably incorrect.

An implementation of the [van Emde Boas tree][veb-tree], a cache-oblivious
data-structure.

[veb-tree]: https://en.wikipedia.org/wiki/Van_Emde_Boas_tree

### What is a cache-oblivious data-structure?

With modern processor architectures, runtime is often dominated by the memory
hierarchy.  It would be nice if our data-structures and algorithms were aware of
this.

A [cache-oblivious][cache-oblivious-wiki] data-structure is a data-structure
that takes advantage of modern processor caches, sometimes optimally, without
knowing the size of the cache.  This means you don't need to parameterize by the
size of the cache or recompile for every device to be optimal.

[cache-oblivious-wiki]: https://en.wikipedia.org/wiki/Cache-oblivious_algorithm

### Details

This strives to implement an interface similar to [BTreeMap][btree-map-docs] in
the standard library.

_u_ is the size of the key universe.  For example, if your key needs to be any
64-bit integer, then _u_ = 2<sup>64</sup>.  _n_ is the number of items in the
tree, each in the range [0, _u_ - 1].

The current implementation uses _O_(_n_ * _log_(_log_(_u_))) space.

Operation|Runtime|
---|---
Insert|_O_(_log_(_log_(_u_)))|
Remove|_O_(_log_(_log_(_u_)))|
Lookup|_O_(_log_(_log_(_u_)))|
Successor, Predecessor|_O_(_log_(_log_(_u_)))|
Minimum, Maximum|_O_(1)|

For perspective on what these bounds mean: _log_<sub>2</sub>(_log_<sub>2</sub>(2<sup>64</sup>)) = 6 🤯

Future work:

- More tests
- Benchmarks
- Convenience methods
- Iterators
- Reduce space usage to _O_(_n_)

[btree-map-docs]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html

### Features

This library has the following features:

- 100% Safe Rust
- No runtime dependencies besides the standard library

### License

Dual-licensed under the MIT and Apache 2.0 licenses.
