pub type AstNode = Node<InnerNode, LeafNode>;

pub enum Node<I: ToString, L: ToString> {
    Inner(I),
    Leaf(L),
}

pub struct InnerNode {
    // tab_level: usize,
    pub children: Vec<AstNode>,
}

impl ToString for InnerNode {
    fn to_string(&self) -> String {
        let mut string = "".to_string();
        for node in &self.children {
            string += &node.to_string();
        }
        string
    }
}

pub struct LeafNode {
    pub tabs: usize,
    pub token: String,
    pub new_lines: usize,
}

impl ToString for LeafNode {
    fn to_string(&self) -> String {
        // self.token.clone()
        format!(
            "{}{}{}",
            "\t".repeat(self.tabs),
            self.token,
            "\n".repeat(self.new_lines)
        )
    }
}

impl<I: ToString, L: ToString> Node<I, L> {
    pub fn to_string(&self) -> String {
        match self {
            Node::Inner(info) => info.to_string(),
            Node::Leaf(info) => info.to_string(),
        }
    }

    // fn print(&self) {
    //     println!("{}", self.to_string());
    // }
    //
    // fn write(&self, mut file: &File) -> std::io::Result<()> {
    //     file.write(self.to_string().as_str().as_bytes())?;
    //     Ok(())
    // }
}
