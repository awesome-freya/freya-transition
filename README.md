## `freya-motion`

`freya-motion` is a library created specifically for a GUI framework called `freya`. It allows you to develop more complex animations as well as create transitions.

### API

- **`func:use_transition`**: Creates transitions from one value to another.
- **`func:use_trait_animation`**: allows the use of animations based on implementations of the `trait:Animation`.
- **`func:use_segmented_animation`**: designed to create segmented animations, but unlike **`func:use_trait_animation`** they are created in a `|context| { .... }` closure and cannot be changed afterwards.

- **`trait:Animation`**: simple trait for implementing your own animation methods not included in the library.
  - **`struct:KeyFrameAnimation`**: consists of frames, each of which is located in the range from `0.0` (beginning) to `1.0` (end) and has its own value and Curve.
  - **`struct:PathAnimation`**: "dynamic" version of **`func:use_segmented_animation`**.

### Examples

At the moment there are only three examples, each playing the same animation in different ways (`use_transition`, `use_trait_animation`, `use_segmented_animation`).

![Example](assets/example.gif)

### License

According to `freya`, the project uses an MIT license.

### Acknowledgements
- [**Marc Espin**](https://github.com/marc2332) and the [`freya`](https://crates.io/crates/freya) made by him for this even existing.
- [`keyframe`](https://crates.io/crates/keyframe) made by [**Hannes Mann**](https://github.com/hannesmann) (it's source code helped with creating keyframe animations)