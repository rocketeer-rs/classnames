use ::classnames::Class;
use ::render::{component, rsx, Render};

use ::xmltree::{Element, EmitterConfig};

fn main() {
    let base_class = Class::new("upgrade-modal");

    let html = rsx! {
      <Modal class={base_class} title={"Upgrade your account"} is_important={true}>
        <button class={base_class.el("button")}>{"Click to Upgrade"}</button>
      </Modal>
    };

    let html_str = html.render();
    let pretty_html = prettify_html(&html_str);
    println!("{}", pretty_html);
}

#[allow(unused_braces)]
#[component]
pub fn Modal<Children: Render>(
    class: Class,
    title: &'static str,
    is_important: bool,
    children: Children,
) -> impl Render {
    let base_class = Class::new("modal");

    rsx! {
      <div
        class={base_class.maybe_attr("important", is_important) + class.into()}
      >
        <h2 class={base_class.el("title")}>{title}</h2>

        <div class={base_class.el("children")}>{children}</div>
      </div>
    }
}

fn prettify_html<'a>(html: &'a str) -> String {
    let el = Element::parse(html.as_bytes()).expect("parsexml");

    let mut cfg = EmitterConfig::new();
    cfg.perform_indent = true;

    // Sadly this seems to do nothing : (
    cfg.write_document_declaration = false;

    let mut formatted_html_raw = vec![];
    el.write_with_config(&mut formatted_html_raw, cfg)
        .expect("writexml");

    String::from_utf8(formatted_html_raw).expect("xml output should be valid utf-8")
}
