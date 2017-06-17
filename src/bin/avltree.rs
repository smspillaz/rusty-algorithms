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

fn tree_depth_from_node(index: usize, arena: &AVLNodeArena, depth: usize) -> usize {
    let node = unwrap_to_node(Some(index), arena).unwrap();
    let left_depth = match node.left {
        Some(left) => tree_depth_from_node(left, arena, depth + 1),
        None => depth
    };
    let right_depth = match node.right {
        Some(right) => tree_depth_from_node(right, arena, depth + 1),
        None => depth
    };
    return std::cmp::max(left_depth, right_depth);
}

fn maybe_get_parent_of(index: usize, arena: &AVLNodeArena) -> Option<usize> {
    let node = unwrap_to_node(Some(index), arena).unwrap();
    return node.parent;
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
        let mut index = self.bintree_append(value);

        /* Now that we've added the node and have its index, go up the tree
         * to its parent and check the heights of each left and right subtree */
        while let Some(parent_index) = maybe_get_parent_of(index, &self) {
            index = parent_index;
            let left_tree_depth = match self.nodes[index].left {
                Some(left) => tree_depth_from_node(left, &self, 0),
                None => 0
            };
            let right_tree_depth = match self.nodes[index].right {
                Some(right) => tree_depth_from_node(right, &self, 0),
                None => 0
            };

            if std::cmp::max(left_tree_depth, right_tree_depth) -
               std::cmp::min(left_tree_depth, right_tree_depth) > 1 {
                let right_left_child_depth = match unwrap_to_node(self.nodes[index].right, &self) {
                    Some(right) => match right.left {
                        Some(left) => tree_depth_from_node(left, &self, 0),
                        None => 0
                    },
                    None => 0
                };
                let right_right_child_depth = match unwrap_to_node(self.nodes[index].right, &self) {
                    Some(right) => match right.right {
                        Some(right) => tree_depth_from_node(right, &self, 0),
                        None => 0
                    },
                    None => 0
                };

                if right_tree_depth > left_tree_depth {
                    /* Where the left child has a height two less than the right child
                     * and the right child of right has a height one more than the
                     * left child of r */
                    if right_right_child_depth - right_left_child_depth > 0 {
                        /* The current node becomes the left subchild. Assert that we
                         * have a right subchild here since the right tree is longer. Then
                         * the current right child becomes the new local root and the left
                         * subchild of the right child becomes the new right subchild of
                         * the left child */
                        let new_left_child = index;
                        let new_local_root = self.nodes[index].right.unwrap();
                        let new_left_right_child = unwrap_to_node(self.nodes[index].right, &self).unwrap().left;

                        /* Update the parent's child reference now */
                        if let Some(parent_index) = self.nodes[index].parent {
                            self.nodes[new_local_root].parent = Some(parent_index);
                            let parent_node = &mut self.nodes[parent_index];
                            match parent_node.left {
                                Some(left) => {
                                    if left == index {
                                        parent_node.left = Some(new_local_root);
                                    }
                                },
                                None => {}
                            }
                            match parent_node.right {
                                Some(right) => {
                                    if right == index {
                                        parent_node.right = Some(new_local_root);
                                    }
                                },
                                None => {}
                            }
                        } else {
                            self.nodes[new_local_root].parent = None;
                        }

                        /* Now, the new_local_root node must have new_left_child as its left child */
                        self.nodes[new_local_root].left = Some(new_left_child);
                        self.nodes[new_left_child].right = new_left_right_child;
                        self.nodes[new_left_child].parent = Some(new_local_root);

                        if let Some(new_left_right_child_index) = new_left_right_child {
                            self.nodes[new_left_right_child_index].parent = Some(new_left_child);
                        }

                        /* Update root if required */
                        if let Some(root_index) = self.root {
                            if root_index == index {
                                self.root = Some(new_local_root);
                            }
                        }

                        index = new_local_root;
                    } else if right_left_child_depth - right_right_child_depth > 0 {
                        /* Where the left child has height two less than right child
                         * and left child of right has a height one more than the right child
                         * of right. */
                        println!("Would perform right-left rotation on tree");
                    }
                }
            }
        }

        return index;
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
    arena.add(4);
    arena.add(5);
    arena.add(6);

    println!("Tree is {:?}", arena);
}

