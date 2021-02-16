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
mod tests {
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
        let class = (classname("page") + classname("home_page").attr("large")) + classname("noscript").el("pane");
        assert_eq!(class.to_string(), "page home_page home_page--large noscript__pane");
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
        let base : Option<classes::BaseClass> = None;
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
