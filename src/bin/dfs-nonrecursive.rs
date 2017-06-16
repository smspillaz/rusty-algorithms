struct NodeId {
    index: usize
}

struct Node {
    value: i32,
    parent: Option<NodeId>,
    children: Vec<NodeId>
}

fn nonrecursive_dfs(start: NodeId, nodes: Vec<Node>, value: i32) -> Option<NodeId> {
    let mut traversal = Vec::new();

    if nodes[start.index].value == value {
        return Some(NodeId {
            index: start.index
        });
    }

    let mut current = start.index;
    traversal.push(0);
    traversal.push(0);
    loop {
        /* Go to the child specified on the stack and inspect it */
        let ref children = nodes[current].children;
        println!("Stack {:?}", traversal.len());
        let currently_traversed_index = traversal.len() - 1;
        current = children[traversal[currently_traversed_index]].index;
        println!("Visit node {:?} {:?}",
                 current,
                 nodes[current].value);
        if nodes[current].value == value {
            return Some(NodeId {
                index: current
            });
        }

        /* Now, can we push another traversal on to the stack? If so,
         * push it, otherwise, don't. */
        println!("Node {:?}", current);
        if nodes[current].children.len() > 0 {
            traversal.push(0);
        } else {
            /* Pop this node and go to the next child of the parent */
            if let Some(ref our_parent) = nodes[current].parent {
                current = our_parent.index;
                traversal.pop();
                if traversal.len() != 0 {
                    let last = traversal.len() - 1;
                    traversal[last] += 1
                }
            } else {
                break;
            }
        }
    }

    return None
}

struct Arena {
    nodes: Vec<Node>
}

impl Arena {
    pub fn new_node(&mut self, value: i32) -> NodeId {
        let index = self.nodes.len();
        self.nodes.push(Node {
            parent: None,
            value: value,
            children: Vec::new()
        });

        return NodeId {
            index: index
        };
    }
}

impl std::fmt::Debug for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "NodeId: {}", self.index)
    }
}

fn main() {
    /* Assemble a tree */
    let mut arena = Arena {
        nodes: Vec::new()
    };

    let root = arena.new_node(1);
    let left = arena.new_node(2);
    let right = arena.new_node(3);
    let right_left_child = arena.new_node(4);
    let right_right_child = arena.new_node(5);

    arena.nodes[left.index].parent = Some(NodeId {
        index: root.index
    });
    arena.nodes[right.index].parent = Some(NodeId {
        index: root.index
    });
    arena.nodes[right_left_child.index].parent = Some(NodeId {
        index: right.index
    });
    arena.nodes[right_right_child.index].parent = Some(NodeId {
        index: right.index
    });

    arena.nodes[right.index].children.push(right_left_child);
    arena.nodes[right.index].children.push(right_right_child);
    arena.nodes[root.index].children.push(left);
    arena.nodes[root.index].children.push(right);

    println!("Found node {:?}", nonrecursive_dfs(root, arena.nodes, 5));
}