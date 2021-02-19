//!
//! # Classnames
//!
//! An opinionated lLibrary for generating BEM style classnames, in Rust.
//!
//! If you don't know BEM, [BEM](http://getbem.com/naming/) it is a set
//! of naming conventions for CSS names.
//!
//! ## Using Classnames
//!
//! There are two main things to import ...
//!
//!  * `::classnames::classname` - A function for creating classnames.
//!  * `::classnames::Class` - A trait all classnames implement. Use this for when you want to pass classnames to places that you want to use them.
//!
//! The way classnames works, is internally it wraps each Class with
//! a different Class type. Adding on "\_\_child" and "--attr" classes,
//! and so on.
//!
//! This is to avoid needing to build lots of strings internally,
//! in order to make it more efficient.
//!
//! ### The crux of using Classnames ...
//!
//!  1. You call `classname` to create a base classname.
//!  2. You may then call `.el` to generate a child classname.
//!  3. You may also call `.attr` to add on any BEM attributes.
//!  4. You can then add classes together, to allow printing multiple different classnames in a component.
//!  5. Finally all of the above; the base class, the child names, attributes, and classes added together. All support being turned into a `String`, or being printed with `::std::fmt::Display`. That's how you get the formatted classname out.
//!
//! Here is the above again in code ...
//!
//! ```
//! use ::classnames::Class;
//! use ::classnames::classname;
//!
//! fn example() {
//!   // 1. Prints "banner"
//!   let base_class = classname("banner");
//!   println!("{}", base_class);
//!
//!   // 2. Prints "banner__header"
//!   let header = base_class.el("header");
//!   println!("{}", header);
//!
//!   // 3. Prints "banner__header banner__header--bold"
//!   let bold_header = base_class.el("header").attr("bold");
//!   println!("{}", bold_header);
//!
//!   // 4. Prints "banner pricing-banner"
//!   let pricing_banner = base_class + classname("pricing-banner");
//!   println!("{}", pricing_banner);
//!
//!   // 5. Prints out HTML with the classes included.
//!   format!(r#"
//!       <div class="{base_class}">
//!         <h1 class="{header_class}">Pricing Information</h1>
//!
//!         <p>Example copy</p>
//!       </div>
//!   "#, base_class = pricing_banner, header_class = bold_header);
//! }
//! ```
//!
//! ### Passing classnames to other functions
//!
//! All of the internal Classname types implement `::classnames::Class`.
//! They can be passed by using this type, which you can also wrap in an `Option` for convenience.
//!
//! For example ...
//!
//! ```
//! use ::classnames::Class;
//! use ::classnames::classname;
//! use ::render::{component, rsx, Render};
//!
//! #[component]
//! pub fn Card<C: Class, Children: Render>(
//!   class: Option<C>,
//!   children: Children,
//! ) -> impl Render {
//!   let base_class = classname("card");
//!
//!   rsx! {
//!     <div
//!       class={base_class + class}
//!     >
//!       {children}
//!     </div>
//!   }
//! }
//! ```
//!

//
// Internally this crate works by structuring nodes in reverse order.
//
// i.e. The following code ...
//
//   Node::new("home_page").append("content").append("button")
//
// Would produce a structure like this ...
//
//  {
//      base_class: "button",
//      parent: {
//          base_class: "content",
//          parent: {
//              base_class: "home_page",
//              parent: None,
//          }
//      }
//  }
//

mod class;
pub mod classes;

pub use crate::class::classname;
pub use crate::class::Class;

#[cfg(test)]
mod integration {
    use super::*;

    #[test]
    fn it_should_print_base_class() {
        let class = classname("home_page");
        assert_eq!(class.to_string(), "home_page");
    }

    #[test]
    fn it_should_print_base_class_children() {
        let class = classname("home_page").el("content");
        assert_eq!(class.to_string(), "home_page__content");
    }

    #[test]
    fn it_should_append_classes() {
        let class = classname("page") + "home_page";
        assert_eq!(class.to_string(), "page home_page");
    }

    #[test]
    fn it_should_append_different_classes() {
        let class = (classname("page") + classname("home_page").attr("large"))
            + classname("noscript").el("pane");
        assert_eq!(
            class.to_string(),
            "page home_page home_page--large noscript__pane"
        );
    }

    #[test]
    fn it_should_append_multiple_classes() {
        let class = classname("page") + "home_page" + "noscript";
        assert_eq!(class.to_string(), "page home_page noscript");
    }

    #[test]
    fn it_should_print_base_class_with_attributes() {
        let class = classname("home_page").attr("wide");
        assert_eq!(class.to_string(), "home_page home_page--wide");
    }

    #[test]
    fn it_should_print_base_class_with_multiple_attributes() {
        let class = classname("home_page").attr("wide").attr("bold");
        assert_eq!(
            class.to_string(),
            "home_page home_page--wide home_page--bold"
        );
    }

    #[test]
    fn it_should_print_base_class_children_with_attributes() {
        let class = classname("home_page").el("content").attr("large");
        assert_eq!(
            class.to_string(),
            "home_page__content home_page__content--large"
        );
    }

    #[test]
    fn it_should_print_base_class_children_with_attributes_with_additions() {
        let class = classname("home_page").el("content").attr("large") + "noscript";
        assert_eq!(
            class.to_string(),
            "home_page__content home_page__content--large noscript"
        );
    }

    #[test]
    fn it_should_not_print_added_optional_classes_when_none() {
        let base: Option<classes::BaseClass> = None;
        let class = classname("page") + "home_page" + base + "noscript";
        assert_eq!(class.to_string(), "page home_page  noscript");
    }

    #[test]
    fn it_should_print_added_optional_classes_when_some() {
        let base = Some(classname("mobile"));
        let class = classname("page") + "home_page" + base + "noscript";
        assert_eq!(class.to_string(), "page home_page mobile noscript");
    }
}
