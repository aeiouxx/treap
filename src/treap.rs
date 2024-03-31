#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::logger::{LogLevel, Logger, StdOutLogger};
use std::{
    cmp::{self, Ordering},
    fmt::Display,
};
// > Aliases
type NodePtr<K, V, P> = Box<ValueNode<K, V, P>>;
type OptNodePtr<K, V, P> = Option<Box<ValueNode<K, V, P>>>;
// < Aliases
// > Traits
// > Node
#[derive(Debug)]
struct ValueNode<K, V, P>
where
    K: Ord + Display,
    P: Ord + Display,
    V: Display,
{
    key: K,
    value: V,
    priority: P,

    left: Option<Box<ValueNode<K, V, P>>>,
    right: Option<Box<ValueNode<K, V, P>>>,
}
impl<K, V, P> ValueNode<K, V, P>
where
    K: Ord + Display,
    P: Ord + Display,
    V: Display,
{
    fn new(key: K, value: V, priority: P) -> Self {
        ValueNode {
            key,
            value,
            priority,
            left: None,
            right: None,
        }
    }
}
impl<K, V, P> Display for ValueNode<K, V, P>
where
    K: Ord + Display,
    P: Ord + Display,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.key, self.value, self.priority)
    }
}
// < Node
// > Priority generator
pub trait PriorityGenerator<P: Ord> {
    fn generate(&self) -> P;
}
impl PriorityGenerator<u64> for u64 {
    fn generate(&self) -> u64 {
        rand::random::<u64>()
    }
}
impl PriorityGenerator<u8> for u8 {
    fn generate(&self) -> u8 {
        rand::random::<u8>()
    }
}
// < Priority generator
// > Treap
pub struct Treap<K, V, P = u64, G = u64>
where
    K: Ord + Display,
    P: Ord + Display,
    V: Display,
    G: PriorityGenerator<P>,
{
    root: OptNodePtr<K, V, P>,
    generator: G,
}
impl<K, V, P, G> Treap<K, V, P, G>
where
    K: Ord + Display,
    P: Ord + Display,
    V: Display,
    G: PriorityGenerator<P>,
{
    pub fn new(generator: G) -> Self {
        Treap {
            root: None,
            generator,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let priority = self.generator.generate();
        let key_str = key.to_string();
        let value_str = value.to_string();
        let prio_str = priority.to_string();
        self.root = Self::insert_impl(self.root.take(), key, value, priority);
    }
    // This is the actual recursive insert function. It returns the new root of the subtree.
    // Call stack unwinding will assure that the newly inserted node will be bubbled up if
    // necessary.
    fn insert_impl(
        node: OptNodePtr<K, V, P>,
        key: K,
        value: V,
        priority: P,
    ) -> OptNodePtr<K, V, P> {
        match node {
            // Only traversal and rotation
            Some(mut node) => {
                match key.cmp(&node.key) {
                    Ordering::Less => {
                        let left_child = node.left.take();
                        node.left = Self::insert_impl(left_child, key, value, priority);
                        if node
                            .left
                            .as_ref()
                            .map_or(false, |left| left.priority < node.priority)
                        {
                            let rotated = Self::rotate_right(node);
                            node = rotated;
                        }
                    }
                    Ordering::Equal => {
                        node.value = value;
                    }
                    Ordering::Greater => {
                        let right_child = node.right.take();
                        node.right = Self::insert_impl(right_child, key, value, priority);
                        if node
                            .right
                            .as_ref()
                            .map_or(false, |right| right.priority < node.priority)
                        {
                            let rotated = Self::rotate_left(node);
                            node = rotated;
                        }
                    }
                }
                Some(node)
            }
            // This is where the new node is actually created (we traversed all the way down to an
            // empty slot for our new node).
            None => Some(Box::new(ValueNode::new(key, value, priority))),
        }
    }
    // Actually rotates the left child of the node right, not the node itself.
    fn rotate_right(mut parent: NodePtr<K, V, P>) -> NodePtr<K, V, P> {
        match parent.left.take() {
            Some(mut left_child) => {
                let child_right_subtree = left_child.right.take();
                parent.left = child_right_subtree;
                left_child.right = Some(parent);
                left_child
            }
            None => parent,
        }
    }
    // Actually rotates the right child of the node left, not the parent node itself.
    fn rotate_left(mut parent: NodePtr<K, V, P>) -> NodePtr<K, V, P> {
        match parent.right.take() {
            Some(mut right_child) => {
                let child_left_subtree = right_child.left.take();
                parent.right = child_left_subtree;
                right_child.left = Some(parent);
                right_child
            }
            None => parent,
        }
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        Self::get_mut_impl(&mut self.root, key)
    }
    fn get_mut_impl(node: &mut OptNodePtr<K, V, P>, key: K) -> Option<&mut V> {
        node.as_mut().and_then(|node| match key.cmp(&node.key) {
            Ordering::Less => Self::get_mut_impl(&mut node.left, key),
            Ordering::Equal => Some(&mut node.value),
            Ordering::Greater => Self::get_mut_impl(&mut node.right, key),
        })
    }
    pub fn get(&self, key: K) -> Option<&V> {
        Self::get_impl(&self.root, key)
    }
    fn get_impl(node: &OptNodePtr<K, V, P>, key: K) -> Option<&V> {
        node.as_ref().and_then(|node| match key.cmp(&node.key) {
            Ordering::Less => Self::get_impl(&node.left, key),
            Ordering::Equal => Some(&node.value),
            Ordering::Greater => Self::get_impl(&node.right, key),
        })
    }
    pub fn height(&self) -> usize {
        Self::node_height(self.root.as_ref())
    }
    fn node_height(node: Option<&NodePtr<K, V, P>>) -> usize {
        match node {
            Some(node) => {
                1 + cmp::max(
                    Self::node_height(node.left.as_ref()),
                    Self::node_height(node.right.as_ref()),
                )
            }
            None => 0,
        }
    }

    fn has_collision(&self, priority: &P) -> bool {
        Self::has_collision_impl(&self.root, priority)
    }

    fn has_collision_impl(node: &OptNodePtr<K, V, P>, priority: &P) -> bool {
        node.as_ref()
            .map_or(false, |node| match priority.cmp(&node.priority) {
                Ordering::Less => Self::has_collision_impl(&node.left, &priority),
                Ordering::Equal => true,
                Ordering::Greater => Self::has_collision_impl(&node.right, &priority),
            })
    }

    pub fn print(&self) {
        self.print_node(
            &self.root,
            0,
            "",
            true,
            self.root
                .as_ref()
                .map_or_else(|| false, |node| node.left.is_some()),
        );
    }

    fn print_node(
        &self,
        node: &OptNodePtr<K, V, P>,
        depth: usize,
        prefix: &str,
        is_right: bool,
        had_left: bool,
    ) {
        if let Some(node) = node {
            let indent = " ";
            let node_prefix = if depth > 0 {
                if is_right {
                    if had_left {
                        "├R"
                    } else {
                        "└R"
                    }
                } else {
                    "└L"
                }
            } else {
                ""
            };
            println!("{}{}{}{}", prefix, indent, node_prefix, node);
            let has_left = node.left.is_some();
            let prefix = (prefix.to_owned()
                + &indent.to_owned()
                + if is_right && had_left && depth > 0 {
                    "│"
                } else {
                    ""
                })
            .to_owned();
            self.print_node(&node.right, depth + 1, &prefix, true, has_left);
            self.print_node(&node.left, depth + 1, &prefix, false, has_left);
        }
    }
}
// < Treap
