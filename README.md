# physics2d-rs

[![Crates.io](https://img.shields.io/crates/v/physics2d.svg?style=flat-square)](https://crates.io/crates/physics2d)
[![Build Status](https://img.shields.io/travis/eviltak/physics2d-rs.svg?style=flat-square)](https://travis-ci.org/eviltak/physics2d-rs)

Yet another 2D physics engine, but with Iron power.

physics2d-rs is an educational project accompanying [my 2D physics article series](https://www.codeproject.com/Articles/1029858/Making-a-D-Physics-Engine-The-Math) and does not intend to compete with the likes of [nphysics](https://github.com/sebcrozet/nphysics/), but can still be used as part of 2D games and simulations post the initial release.

## Features
- Convex polygon and circle collisions with restitution and friction
- `O(n log n)` broad-phase collision detection using an AABB tree (bounding volume hierarchy)
- Spring joints

To check the project's current progress, take a look at the available examples. New examples are generally created for every major feature addition.

## Examples
The examples use the [`sfml` crate](https://crates.io/crates/sfml), and hence are dependent on both the [SFML](https://www.sfml-dev.org/) and [CSFML](https://www.sfml-dev.org/download/csfml/) development libraries. macOS and GNU/Linux users should have both packages available with their respective package managers. Windows users should follow [these](https://github.com/jeremyletang/rust-sfml/wiki/How-to-use-rust-sfml-on-Windows) instructions to set up the `sfml` crate.

To run an example, use the Cargo `run` command:

    cargo run --example=EXAMPLE-NAME
 
