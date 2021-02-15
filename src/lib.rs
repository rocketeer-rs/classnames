mod attr_class;
mod class;
mod el_class;
mod str_class;

pub use crate::attr_class::AttrClass;
pub use crate::class::Class;
pub use crate::el_class::ElClass;
pub use crate::str_class::StrClass;

/*
 * Internally this crate works by structuring nodes in reverse order.
 *
 * i.e. The following code ...
 *
 *   Node::new("home_page").append("content").append("button")
 *
 * Would produce a structure like this ...
 *
 *  {
 *      base_class: "button",
 *      parent: {
 *          base_class: "content",
 *          parent: {
 *              base_class: "home_page",
 *              parent: None,
 *          }
 *      }
 *  }
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_print_base_class() {
        let class = Class::new("home_page");
        assert_eq!(class.to_string(), "home_page");
    }

    #[test]
    fn it_should_print_base_class_children() {
        let class = Class::new("home_page").el("content");
        assert_eq!(class.to_string(), "home_page__content");
    }

    #[test]
    fn it_should_append_classes() {
        let class = Class::new("page") + "home_page";
        assert_eq!(class.to_string(), "page home_page");
    }

    #[test]
    fn it_should_append_multiple_classes() {
        let class = Class::new("page") + "home_page" + "noscript";
        assert_eq!(class.to_string(), "page home_page noscript");
    }

    #[test]
    fn it_should_print_base_class_with_attributes() {
        let class = Class::new("home_page").attr("wide");
        assert_eq!(class.to_string(), "home_page home_page--wide");
    }

    #[test]
    fn it_should_print_base_class_with_multiple_attributes() {
        let class = Class::new("home_page").attr("wide").attr("bold");
        assert_eq!(
            class.to_string(),
            "home_page home_page--wide home_page--bold"
        );
    }

    #[test]
    fn it_should_print_base_class_children_with_attributes() {
        let class = Class::new("home_page").el("content").attr("large");
        assert_eq!(
            class.to_string(),
            "home_page__content home_page__content--large"
        );
    }

    #[test]
    fn it_should_print_base_class_children_with_attributes_with_additions() {
        let class = Class::new("home_page").el("content").attr("large") + "noscript";
        assert_eq!(
            class.to_string(),
            "home_page__content home_page__content--large noscript"
        );
    }
}
