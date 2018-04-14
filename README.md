# Daily Coding Problem: Problem #3

[![Build Status](https://travis-ci.org/DCP-solved-with-Rust/dcp_00003.svg?branch=master)](https://travis-ci.org/DCP-solved-with-Rust/dcp_00003?branch=master)

This repository is part of the [DCP Solved with Rust](https://dcp-solved-with-rust.github.io/) series.

> Good morning. Here's your coding interview problem for today.
>
> This problem was asked by Google.
>
> Given the root to a binary tree, implement serialize(root), which serializes the
> tree into a string, and deserialize(s), which deserializes the string back into
> the tree.
>
> For example, given the following Node class
>
> ```
> class Node:
>     def __init__(self, val, left=None, right=None):
>         self.val = val
>         self.left = left
>         self.right = right
> ```
>
> The following test should pass:
>
> ```
> node = Node('root', Node('left', Node('left.left')), Node('right'))
> assert deserialize(serialize(node)).left.left.val == 'left.left'
> ```

## Solution

Solved with Rust 1.28.0 nightly-2018-06-13. https://rustup.rs/

### Tests

The original problem statement included the following test:

> ```
> node = Node('root', Node('left', Node('left.left')), Node('right'))
> assert deserialize(serialize(node)).left.left.val == 'left.left'
> ```

A test has been written for this. You can run the test with

```
cargo test
```

Output:

```
(output)
```
