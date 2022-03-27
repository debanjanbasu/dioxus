use std::cell::Cell;

use dioxus_core::VNode;
use dioxus_core::*;
use dioxus_core_macro::*;
use dioxus_html as dioxus_elements;
use dioxus_native_core::client_tree::ClientTree;

#[test]
fn tree_initial_build_simple() {
    use std::cell::Cell;

    #[allow(non_snake_case)]
    fn Base(cx: Scope) -> Element {
        rsx!(cx, div {})
    }

    let vdom = VirtualDom::new(Base);

    let mutations = vdom.create_vnodes(rsx! {
        div{}
    });

    let mut tree: ClientTree<(), ()> = ClientTree::new();

    let _to_update = tree.apply_mutations(vec![mutations]);
    let root_div = VElement {
        id: Cell::new(Some(ElementId(1))),
        key: None,
        tag: "div",
        namespace: None,
        parent: Cell::new(Some(ElementId(0))),
        listeners: &[],
        attributes: &[],
        children: &[],
    };
    assert_eq!(tree.size(), 1);
    assert!(&tree.contains_node(&VNode::Element(&root_div)));
    assert_eq!(tree[1].height, 1);
}

#[test]
fn tree_initial_build_with_children() {
    #[allow(non_snake_case)]
    fn Base(cx: Scope) -> Element {
        rsx!(cx, div {})
    }

    let vdom = VirtualDom::new(Base);

    let mutations = vdom.create_vnodes(rsx! {
        div{
            div{
                "hello"
                p{
                    "world"
                }
                "hello world"
            }
        }
    });

    let mut tree: ClientTree<(), ()> = ClientTree::new();

    let _to_update = tree.apply_mutations(vec![mutations]);
    let first_text = VText {
        id: Cell::new(Some(ElementId(3))),
        text: "hello",
        is_static: true,
    };
    let first_text_node = VNode::Text(&first_text);
    let child_text = VText {
        id: Cell::new(Some(ElementId(5))),
        text: "world",
        is_static: true,
    };
    let child_text_node = VNode::Text(&child_text);
    let child_p_el = VElement {
        id: Cell::new(Some(ElementId(4))),
        key: None,
        tag: "p",
        namespace: None,
        parent: Cell::new(Some(ElementId(2))),
        listeners: &[],
        attributes: &[],
        children: &[child_text_node],
    };
    let child_p_node = VNode::Element(&child_p_el);
    let second_text = VText {
        id: Cell::new(Some(ElementId(6))),
        text: "hello world",
        is_static: true,
    };
    let second_text_node = VNode::Text(&second_text);
    let child_div_el = VElement {
        id: Cell::new(Some(ElementId(2))),
        key: None,
        tag: "div",
        namespace: None,
        parent: Cell::new(Some(ElementId(1))),
        listeners: &[],
        attributes: &[],
        children: &[first_text_node, child_p_node, second_text_node],
    };
    let child_div_node = VNode::Element(&child_div_el);
    let root_div = VElement {
        id: Cell::new(Some(ElementId(1))),
        key: None,
        tag: "div",
        namespace: None,
        parent: Cell::new(Some(ElementId(0))),
        listeners: &[],
        attributes: &[],
        children: &[child_div_node],
    };
    assert_eq!(tree.size(), 6);
    assert!(&tree.contains_node(&VNode::Element(&root_div)));
    assert_eq!(tree[1].height, 1);
    assert_eq!(tree[2].height, 2);
    assert_eq!(tree[3].height, 3);
    assert_eq!(tree[4].height, 3);
    assert_eq!(tree[5].height, 4);
    assert_eq!(tree[6].height, 3);
}
