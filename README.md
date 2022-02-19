# veb-tree

**Status:** This is a personal project for learning that's not well-tested and
probably incorrect.

An implementation of the [van Emde Boas tree][veb-tree], a fast map
data-structure with integer keys.

[veb-tree]: https://en.wikipedia.org/wiki/Van_Emde_Boas_tree

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

### Benchmarks

Install [`cargo-criterion`](https://github.com/bheisler/cargo-criterion).

```shell
cargo criterion
```

### License

Dual-licensed under the MIT and Apache 2.0 licenses.
