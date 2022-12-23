#[derive(Debug, Default, Clone)]
pub struct Node {
    pub idx: usize,
    pub parents: Vec<usize>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(idx: usize) -> Self {
        Node {
            idx,
            ..Node::default()
        }
    }

    pub fn add_child(&mut self, value: usize) -> &mut Self {
        let mut child = Node::new(value);
        child.parents.append(&mut self.parents.clone());
        child.parents.push(self.idx);

        self.append(child);

        self.children.last_mut().unwrap()
    }

    fn append(&mut self, node: Node) -> &mut Self {
        self.children.push(node);

        self
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn contains(&self, value: usize) -> bool {
        if self.idx == value {
            return true;
        }

        self.parents.contains(&value)
    }

    pub fn get_leaves(&self) -> Vec<&Node> {
        if self.is_leaf() {
            return vec![self];
        }

        let mut leaves = Vec::new();

        let children = self.children.iter();

        for child in children {
            let mut child_leaves = child.get_leaves();
            leaves.append(&mut child_leaves);
        }

        leaves
    }

    pub fn get_len(&self) -> usize {
        self.parents.len()
    }
}
