use sgf_parse::{
    Color, SgfNode,
    go::{Move, Prop},
};

pub struct BranchesIterator<'a> {
    /// Stack of parent nodes to keep track of the current branch
    stack: Vec<(&'a SgfNode<Prop>, usize)>,
}

impl<'a> BranchesIterator<'a> {
    pub fn new(root: &'a SgfNode<Prop>) -> Self {
        let mut stack = Vec::new();
        stack.push((root, 0));
        BranchesIterator { stack }
    }
}

impl<'a> Iterator for BranchesIterator<'a> {
    type Item = Vec<&'a SgfNode<Prop>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (current_node, child_idx) = match self.stack.pop() {
                Some(data) => data,
                None => return None, // stack is empty, return None. iteration is done
            };
            // if current node children is empty, return current node
            if current_node.children.is_empty() {
                let mut current_path = self.stack.iter().map(|(node, _)| *node).collect::<Vec<_>>();
                // Push current node to path
                current_path.push(current_node);
                // backtrack to the last unvisited node
                loop {
                    let (current_node, child_idx) = match self.stack.pop() {
                        Some(data) => data,
                        None => break, // stack is empty, break out of the loop
                    };
                    if current_node.children.len() > child_idx {
                        // Push back
                        self.stack.push((current_node, child_idx));
                        break;
                    }
                }
                return Some(current_path);
            } else {
                self.stack.push((current_node, child_idx + 1));
                let child_node = &current_node.children[child_idx];
                self.stack.push((child_node, 0)); // 
            }
        }
    }
}

pub trait IntoMoves {
    fn into_moves(self) -> Vec<(Color, Move)>;
}

impl<'a, T> IntoMoves for T
where
    T: Iterator<Item = &'a SgfNode<Prop>>,
{
    fn into_moves(self) -> Vec<(Color, Move)> {
        let mut moves = Vec::new();
        self.for_each(|node| {
            if let Some(prop) = node.get_move() {
                match prop {
                    Prop::B(m) => {
                        moves.push((Color::Black, m.clone()));
                    }
                    Prop::W(m) => {
                        moves.push((Color::White, m.clone()));
                    }
                    _ => {}
                }
            }
        });
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sgf_parse::go::parse;
    use std::str::FromStr;

    #[test]
    fn test_single_tree_iterator() {
        let sgf = "(;SZ[9]C[Some comment];B[de];W[fe])";
        let root = parse(sgf).unwrap().into_iter().next().unwrap(); //取出第一个节点
        let mut iter = BranchesIterator::new(&root);
        let first_branch = iter.next().unwrap();
        assert_eq!(first_branch.len(), 3);
    }

    #[test]
    fn test_multi_tree_iterator() {
        let sgf = "(;SZ[9]C[Some comment];B[ee];W[ce](;B[ge](;W[gd])(;W[gf]))(;B[cf]))";
        let root = parse(sgf).unwrap().into_iter().next().unwrap(); //取出第一个节点
        let mut iter = BranchesIterator::new(&root);

        let first_branch = iter.next().unwrap();
        assert_eq!(first_branch.len(), 5);
        for node in first_branch.iter() {
            println!("first node: {:?}", node.get_move().map(|m| m.to_string()));
        }

        let second_branch = iter.next().unwrap();
        assert_eq!(second_branch.len(), 5);
        for node in second_branch.iter() {
            println!("second node: {:?}", node.get_move().map(|m| m.to_string()));
        }
        let third_branch = iter.next().unwrap();
        assert_eq!(third_branch.len(), 4);
        for node in third_branch.iter() {
            println!("third node: {:?}", node.get_move().map(|m| m.to_string()));
        }

        let main = parse(sgf).unwrap().into_iter().next().unwrap(); //取出第一个节点
        let iter = main.main_variation();
        for node in iter {
            println!("main node: {:?}", node.get_move().map(|m| m.to_string()));
        }
    }

    #[test]
    fn test_into_moves() {
        let sgf = "(;SZ[9]C[Some comment];B[de];W[fe])";
        let root = parse(sgf).unwrap().into_iter().next().unwrap();
        let mut iter = BranchesIterator::new(&root);
        let main_node = iter.next().unwrap();
        let moves = main_node.into_iter().into_moves();
        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0], (Color::Black, Move::from_str("de").unwrap()));
        assert_eq!(moves[1], (Color::White, Move::from_str("fe").unwrap()));
    }

    #[test]
    fn test_main_main_variation() {
        let sgf = "(;SZ[9]C[Some comment];B[de];W[fe])";
        let main = parse(sgf).unwrap().into_iter().next().unwrap(); //取出第一个节点
        let iter = main.main_variation();
        let moves = iter.into_moves();
        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0], (Color::Black, Move::from_str("de").unwrap()));
        assert_eq!(moves[1], (Color::White, Move::from_str("fe").unwrap()));
    }
}
