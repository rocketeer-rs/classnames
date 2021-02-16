# Classnames
[![Current Crates.io Version](https://img.shields.io/crates/v/classnames.svg)](https://crates.io/crates/classnames)

A lLibrary for generating BEM style classnames, in Rust.

If you don't know BEM, [BEM](http://getbem.com/naming/) it is a set of naming conventions for CSS names.
Please read the DEM guide linked for an explination of the naming conventions.

## Using Classnames

With Classnames you import `::classnames::Class`, and that is used as the root for creating new classnames.

### The crux of using Classnames ...

Calling each of these creates `Class` types (or similar). These can then be printed, or turned into a string, to retrieve the full class name as a string.

 * Use `Class::new` for creating a base class.
 * Call `.el` on it to generate a child class.
 * Call `.attr` to generate the class for that component, and the componnt with an attribute.
 * Finally you can add two classes together, to allow printing multiple different classnames for a component.

## Basic usage example ...

```
use ::classnames::Class;

fn example() {
  // ".my-component"
  let base_class = Class::new("my-component");

  // ".my-component .my-component--large"
  let base_class_large = base_class.attr("large");

  // ".my-component__child-name"
  let child_class = base_class.el("child-name");

  // ".my-component__child-name .my-component__child-name--red"
  let child_class_red = child_class.attr("red");
}
```

## Example with RSX

Classnames is intended to be used with something else for rendering HTML.
For example with the [render](https://crates.io/crates/render) crate, which is used here.

This is an example for the HTML to a hypothetical `TextInput` component, which can be optionally disabled.

```
  use ::classnames::Class;
  use ::render::{rsx, Render, component};

  #[component]
  pub fn TextInput(
    is_disabled: bool,
    error: &'static str,
  ) -> impl Render {
    let base_class = Class::new("text-input");

    rsx! {
      <div
        class={base_class}
      >
        <input class={base_class.el("input").maybe_attr("disabled", is_disabled)} type={"text"} />

        <div class={base_class.el("error")}>
          {error}
        </div>

        <img src={"/input-icon.svg"} class={base_class.el("icon").attr("large")} />
      </div>
    }
  }
```

Running `render( true, &"Some error has occured" )` will produce HTML like ...

```
  <div
    class="text-input"
  >
    <input class="text-input__input text-input__input--disabled" type={"text"} />

    <div class="text-input__error">
      {"Some error has occured"}
    </div>

    <img src={"/input-icon.svg"} class="text-input__icon text-input__icon--large" />
  </div>
```
