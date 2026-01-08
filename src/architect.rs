use colored::*;

pub enum ProofType {
    Private,
    Tube,
    Merge,
    Root,
}

pub struct ProofNode {
    pub proof_type: ProofType,
    pub proof_id: String,
    pub children: Vec<ProofNode>,
}

impl ProofNode {
    pub fn new(proof_type: ProofType, proof_id: &str) -> Self {
        Self {
            proof_type,
            proof_id: proof_id.to_string(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: ProofNode) {
        self.children.push(child);
    }
}

/// Recursively prints the proof tree to the terminal
pub fn visualize_tree(node: &ProofNode, depth: usize) {
    let indent = "  ".repeat(depth);
    let icon = match node.proof_type {
        ProofType::Private => "[P]".cyan(),
        ProofType::Tube => "[T]".magenta(),
        ProofType::Merge => "[M]".yellow(),
        ProofType::Root => "[R]".green().bold(),
    };

    println!("{}{} ID: {}", indent, icon, node.proof_id);

    for child in &node.children {
        visualize_tree(child, depth + 1);
    }
}

pub fn generate_mock_tree() -> ProofNode {
    let mut root = ProofNode::new(ProofType::Root, "0xroot_rollup_7720761");
    
    let mut merge_1 = ProofNode::new(ProofType::Merge, "0xmerge_batch_A");
    merge_1.add_child(ProofNode::new(ProofType::Tube, "0xtube_wrapper_1"));
    merge_1.add_child(ProofNode::new(ProofType::Private, "0xuser_tx_private_1"));

    root.add_child(merge_1);
    root
}
