<p align="center">
  <a href="https://docs.rs/sdsl"><img src="https://docs.rs/sdsl/badge.svg" alt="Docs"></a>
  <a href="https://matrix.to/#/#sdsl-rs:matrix.org"><img src="https://img.shields.io/matrix/sdsl-rs:matrix.org?label=chat&logo=matrix" alt="Matrix"></a>
  <a href="https://github.com/sdsl-rs/sdsl-rs/actions/workflows/main.yaml"><img src="https://github.com/sdsl-rs/sdsl-rs/actions/workflows/main.yaml/badge.svg" alt="CI"></a>
  <a href="https://github.com/sdsl-rs/sdsl-rs/discussions"><img src="https://img.shields.io/badge/github-discussions-red" alt="Discussions"></a>
</p>

# SDSL-RS

> A Rust interface for the Succinct Data Structure Library ([SDSL-lite](https://github.com/simongog/sdsl-lite)).

## Introduction

[SDSL-lite](https://github.com/simongog/sdsl-lite) is a C++11 library which implements succinct data structures. Example structures include: arbitrary width integer vectors, wavelet trees, and compressed suffix arrays. The library is commonly used within bioinformatics among other fields.

Many of the data structures provided by the library are defined using C++ [templates](https://en.cppreference.com/w/cpp/language/class_template). This poses a challenge when interfacing with the library from languages other than C++. The primary aim of SDSL-RS is to take on the heavy lifting of interfacing with the library from Rust!

## Progress

The following list includes all structures which will be supported. The list was derived from the cheatsheet found [here](https://simongog.github.io/assets/data/sdsl-cheatsheet.pdf).

Checked entries in the list are currently supported. The near term goal is to support one data structure from each section.

Please contact the [chat channel](https://matrix.to/#/#sdsl-rs:matrix.org) for prioritization requests.

<details>
  <summary>Full Structures Status List</summary>

### Integer vectors

* [x] IntVector

### Bit vectors

* [x] BitVector (plain bit vector)
* [ ] BitVectorIl (interleaved bit vector)
* [x] RrrVector (H<sub>0</sub> compressed bit vector)
* [ ] SdVector (sparse bit vector)
* [ ] HybVector (hybrid bit vector)

### Rank Supports

* [x] RankSupportV
* [ ] RankSupportV5
* [ ] RankSupportScan
* [ ] RankSupportIl
* [ ] RankSupportRrr
* [ ] RankSupportSd
* [ ] RankSupportHyb

### Select Supports

* [x] SelectSupportMcl
* [ ] SelectSupportScan
* [ ] SelectSupportIl
* [ ] SelectSupportRrr
* [ ] SelectSupportSd

### Wavelet Trees

* [x] WtHuff
* [x] WtInt
* [ ] WtRlmn
* [ ] WtGmr
* [ ] WtAp
* [ ] WmInt
* [ ] WtBlcd
* [ ] WtHutu

### Compressed Suffix Arrays

* [ ] CsaBitcompressed
* [ ] CsaSada
* [ ] CsaWt

### Longest Common Prefix Arrays

* [ ] LcpBitcompressed
* [ ] LcpDac
* [ ] LcpByte
* [ ] LcpWt
* [ ] LcpVlc
* [ ] LcpSupportSada
* [ ] LcpSupportTree
* [ ] LcpSupportTree2

### Balanced Parentheses Supports

* [ ] BpSupportG
* [ ] BpSupportGg
* [ ] BpSupportSada

### Compressed Suffix Trees

* [ ] CstSada
* [ ] CstSct3

### Range Min/Max Query

* [ ] RmqSupportSparseTable
* [ ] RmqSuccintSada
* [ ] RmqSuccintSct

</details>

## Requirements

### SDSL-lite

SDSL-lite must be compilable within the development environment. Requirements can be found [here](https://github.com/simongog/sdsl-lite#requirements).

Commonly missing dependencies include `libdivsufsort-dev`.

### SDSL-RS

`SDSL-RS` uses [const generics](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html) and therefore may require the `beta` Rust [toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html).

Projects which use `SDSL-RS` must include a [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) (`build.rs`) with contents such as:

```rust
// build.rs
fn main() {
    match sdsl::build() {
        Ok(_) => {}
        Err(e) => panic!("Error: {}", e),
    };
}
```

The project's `Cargo.toml` file must therefore include a `build-dependencies` section such as:

```toml
[dependencies]
sdsl = "0.1.0"
# ... other dependencies ...

[build-dependencies]
sdsl = "0.1.0"
```

The `sdsl::build()` function call allows `SDSL-RS` to analyse the current project's code base (via [MIR](https://rustc-dev-guide.rust-lang.org/mir/index.html)) and build an appropriate interface in the top level `target` directory. The initial compilation of the project after adding `SDSL-RS` takes a while because `SDSL-lite` is compiled as a dependency. Subsequent compilations should be quick.

## Examples

An example project can be found [here](https://github.com/sdsl-rs/sdsl-rs/tree/master/examples). It contains examples for all supported data structures.

This example shows how to construct a H<sub>0</sub> compressed bit vector (`sdsl::bit_vectors::RrrVector`):

```rust
let bv = sdsl::bit_vector! {1, 1, 0, 1};
let rv = sdsl::bit_vectors::RrrVector::<sdsl::int_vectors::IntVector<0>, 10, 2>::new(&bv)?;

let result = rv.get_bv_element(2);
let expected = 0;
assert_eq!(result, expected);
```
