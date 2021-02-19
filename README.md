# Classnames
[![Current Crates.io Version](https://img.shields.io/crates/v/classnames.svg)](https://crates.io/crates/classnames)

An opinionated lLibrary for generating BEM style classnames, in Rust.

If you don't know BEM, [BEM](http://getbem.com/naming/) it is a set
of naming conventions for CSS names.

## Using Classnames

There are two main things to import ...

 * `::classnames::classname` - A function for creating classnames.
 * `::classnames::Class` - A trait all classnames implement. Use this for when you want to pass classnames to places that you want to use them.

The way classnames works, is internally it wraps each Class with
a different Class type. Adding on "\_\_child" and "--attr" classes,
and so on.

This is to avoid needing to build lots of strings internally,
in order to make it more efficient.

### The crux of using Classnames ...

 1. You call `classname` to create a base classname.
 2. You may then call `.el` to generate a child classname.
 3. You may also call `.attr` to add on any BEM attributes.
 4. You can then add classes together, to allow printing multiple different classnames in a component.
 5. Finally all of the above; the base class, the child names, attributes, and classes added together. All support being turned into a `String`, or being printed with `::std::fmt::Display`. That's how you get the formatted classname out.

Here is the above again in code ...

```
use ::classnames::Class;
use ::classnames::classname;

fn example() {
  // 1. Prints "banner"
  let base_class = classname("banner");
  println!("{}", base_class);

  // 2. Prints "banner__header"
  let header = base_class.el("header");
  println!("{}", header);

  // 3. Prints "banner__header banner__header--bold"
  let bold_header = base_class.el("header").attr("bold");
  println!("{}", bold_header);

  // 4. Prints "banner pricing-banner"
  let pricing_banner = base_class + classname("pricing-banner");
  println!("{}", pricing_banner);

  // 5. Prints out HTML with the classes included.
  format!(r#"
      <div class="{base_class}">
        <h1 class="{header_class}">Pricing Information</h1>

        <p>Example copy</p>
      </div>
  "#, base_class = pricing_banner, header_class = bold_header);
}
```

### Passing classnames to other functions

All of the internal Classname types implement `::classnames::Class`.
They can be passed by using this type, which you can also wrap in an `Option` for convenience.

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

## Example with RSX

Classnames is intended to be used with something else for rendering HTML.
For example with the [render](https://crates.io/crates/render) crate.

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

