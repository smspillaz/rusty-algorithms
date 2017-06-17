struct AVLNode {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    value: u32
}

struct AVLNodeArena {
    nodes: Vec<AVLNode>,
    root: Option<usize>
}

impl AVLNodeArena {
    fn new() -> AVLNodeArena {
        AVLNodeArena {
            nodes: vec![],
            root: None
        }
    }

    fn bintree_append(&mut self, value: u32) -> usize {
        let insert_node_index = self.nodes.len();

        if let Some(root_node_index) = self.root {
            let mut candidate = root_node_index;
            loop {
                if self.nodes[candidate].value == value {
                    return candidate;
                } else {
                    if value < self.nodes[candidate].value {
                        /* Do we have a left subtree? If so, enter it */
                        if let Some(left_subtree_index) = self.nodes[candidate].left {
                            candidate = left_subtree_index;
                        } else {
                            /* We need to insert a new node here, do that now */
                            self.nodes.push(AVLNode {
                                parent: Some(candidate),
                                left: None,
                                right: None,
                                value
                            });
                            self.nodes[candidate].left = Some(insert_node_index);
                            return insert_node_index;
                        }
                    } else {
                        /* Do we have a right subtree? If so, enter it */
                        if let Some(right_subtree_index) = self.nodes[candidate].right {
                            candidate = right_subtree_index;
                        } else {
                            /* We need to insert a new node here, do that now */
                            self.nodes.push(AVLNode {
                                parent: Some(candidate),
                                left: None,
                                right: None,
                                value
                            });
                            self.nodes[candidate].right = Some(insert_node_index);
                            return insert_node_index;
                        }
                    }
                }
            }
        }

        self.nodes.push(AVLNode {
            parent: None,
            left: None,
            right: None,
            value
        });
        self.root = Some(insert_node_index);
        return insert_node_index;
    }

    pub fn add(&mut self, value: u32) -> usize {
        return self.bintree_append(value);
    }
}

fn unwrap_to_node(index_option: Option<usize>, arena: &AVLNodeArena) -> Option<&AVLNode> {
    if let Some(index) = index_option {
        return Some(&arena.nodes[index]);
    }

    return None;
}

fn maybe_recursively_serialize(node_option: Option<&AVLNode>,
                               index: usize,
                               arena: &AVLNodeArena) -> String {
    if let Some(node) = node_option {
        return node.recursively_serialize(index, arena);
    }

    return "(empty)".to_string();
}

impl AVLNode {
    pub fn recursively_serialize(&self, index: usize, arena: &AVLNodeArena) -> String {
        format!("Node ({}, {}): [{}, {}]",
                index,
                self.value,
                maybe_recursively_serialize(unwrap_to_node(self.left, arena),
                                                           self.left.unwrap_or(0),
                                                           arena),
                maybe_recursively_serialize(unwrap_to_node(self.right, arena),
                                                           self.right.unwrap_or(0),
                                                           arena))
    }
}

impl std::fmt::Debug for AVLNodeArena {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(root_node_index) = self.root {
            /* Get the root node and then start recursively printing nodes */
            return write!(f,
                          "{}",
                          self.nodes[root_node_index].recursively_serialize(root_node_index, &self));
        }

        return write!(f, "empty tree");
    }
}

fn main() {
    let mut arena = AVLNodeArena::new();
    arena.add(1);
    arena.add(0);
    arena.add(2);
    arena.add(3);

    println!("Tree is {:?}", arena);
}

