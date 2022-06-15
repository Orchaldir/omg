# Orchaldir's Map Generator

![CI](https://github.com/Orchaldir/omg/workflows/CI/badge.svg)
[![Current Crates.io Version](https://img.shields.io/crates/v/omg_core)](https://crates.io/crates/omg_core)

This project allows the user to create maps with a number of user-defined steps.
Some example steps are :

* Creating user-defined attributes like rainfall or temperature.
* Adding noise or a gradient to an attribute.
* Transforming an attribute.
* Combining multiple attributes.

The steps can be saved to & loaded from files with [serde](https://serde.rs).

A very simple editor uses [rocket](https://rocket.rs) to visualize the generated map.

## How to run the editor?

With cargo:

```
> cd omg_editor
> cargo run
```

For Intellij set the `working directory` of the `run configuration` to `omg_editor`.

Afterwards you need to open the displayed link (e.g. http://127.0.0.1:8000) in a browser.