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

Operation|Bound|
---|---
Space|_O_(_n_ * log(log(_u_)))|
Insert|_O_(log(log(_u_)))|
Remove|_O_(log(log(_u_)))|
Lookup|_O_(log(log(_u_)))|
Successor, Predecessor|_O_(log(log(_u_)))|
Minimum, Maximum|_O_(1)|

For perspective on what these bounds mean: log<sub>2</sub>(log<sub>2</sub>(2<sup>64</sup>)) = 6 🤯

![Single Run of Successor Comparison with BTreeMap](/doc/successor_single_comparison.svg)

In the graph above, lower is better.  The x-axis is the number of items in the
tree.  The successor operation is run a single time.

The graph shows that the van Emde Boas tree does less work due to the
log(log(_u_)) bound.  However, that's not the whole story.

![Multiple Runs of Successor Comparison with BTreeMap](/doc/successor_multiple_random_order_comparison.svg)

In the above graph, lower is better.  The x-axis is the number of items in the
tree.  The successor operation is run on 100,000 different keys, with keys in
random order.

The graph shows that the runtime for the van Emde Boas tree levels off since
it's proportional to the key size _u_, not the number of items in the tree _n_.
BTreeMap's runtime, on the other hand, continues to grow with _n_.

Depending on your processor, reducing the size of the keys by using u32 or
smaller for the key type may make VebTreeMap even faster.

[btree-map-docs]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html

### Features

This library has the following features:

- 100% Safe Rust
- No runtime dependencies besides the standard library
- Property tests
- Benchmarks measuring statistical significance

Future work:

- More tests
- More benchmarks
- Convenience methods
- Iterators, Range API, Entry API
- Reduce space usage to _O_(_n_)

### Benchmarks

Install [`cargo-criterion`](https://github.com/bheisler/cargo-criterion).

```shell
cargo criterion
```

### License

Dual-licensed under the MIT and Apache 2.0 licenses.
