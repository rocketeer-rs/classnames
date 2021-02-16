# Classnames
[![Current Crates.io Version](https://img.shields.io/crates/v/classnames.svg)](https://crates.io/crates/classnames)

A lLibrary for generating BEM style classnames, in Rust.

If you don't know BEM, [BEM](http://getbem.com/naming/) it is a set of naming conventions for CSS names.
Please read the DEM guide linked for an explination of the naming conventions.

Why?

 * Enforces the BEM layout for CSS naming.
 * It's built in a way to avoid uneccessary intermediate string concatonations. It only generates strings when needed.

## Using Classnames

There are two main things to import ...

 * `::classnames::classname` - A function for creating classnames.
 * `::classnames::Class` - A trait all classnames implement. Use this for when you want to pass classnames around.

### The crux of using Classnames ...

Calling each of these creates `Class` types (or similar). These can then be printed, or turned into a string, to retrieve the full class name as a string.

 * Use `classname` for creating a new base classname. It returns a class.
 * Call `.el` on it to generate a child classname.
 * Call `.attr` to generate the class for that component, and the componnt with an attribute.
 * Finally you can add two classes together, to allow printing multiple different classnames for a component.

## Basic usage example ...

```
use ::classnames::Class;

fn example() {
  // ".my-component"
  let base_class = classname("my-component");

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
For example with the [render](https://crates.io/crates/render) crate, which is used in the example here.

This is an example for the HTML to a hypothetical `TextInput` component, which can be optionally disabled.

```
  use ::classnames::Class;
  use ::render::{rsx, Render, component};

  #[component]
  pub fn TextInput(
    is_disabled: bool,
    error: &'static str,
  ) -> impl Render {
    let base_class = classname("text-input");

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

## Passing classnames to other functions

All of the internal Classname types implement `::classnames::Class`. They can be passed by using this type, which you can also wrap in an `Option` for convenience.

For example ...

```
  use ::classnames::Class;
  use ::classnames::classname;

  #[component]
  pub fn Card<C: Class, Children: Render>(
    class: Option<C>,
    children: Children,
  ) -> impl Render {
    let base_class = classname("card");

    rsx! {
      <div
        class={base_class + class}
      >
        {children}
      </div>
    }
  }
```
