pub use sgf_parse::*;

//transform sgf_parse::Color to crate::Color
impl From<crate::Color> for Color {
    fn from(c: crate::Color) -> Self {
        match c {
            crate::Color::Black => Color::Black,
            crate::Color::White => Color::White,
        }
    }
}

// transform crate::Color to sgf_parse::Color
impl From<Color> for crate::Color {
    fn from(c: Color) -> Self {
        match c {
            Color::Black => crate::Color::Black,
            Color::White => crate::Color::White,
        }
    }
}

// impl crate::Move function to_sgf;
impl crate::Move {
    pub fn to_sgf(self) -> go::Move {
        match self {
            crate::Move::Pass => go::Move::Pass,
            crate::Move::Coordinate { x, y } => go::Move::Move(go::Point { x, y }),
        }
    }

    pub fn from_sgf(m: go::Move) -> Self {
        match m {
            go::Move::Pass => crate::Move::Pass,
            go::Move::Move(p) => crate::Move::Coordinate { x: p.x, y: p.y },
        }
    }
}

pub struct BranchesIterator<'a> {
    /// Stack of parent nodes to keep track of the current branch
    stack: Vec<(&'a SgfNode<go::Prop>, usize)>,
}

impl<'a> BranchesIterator<'a> {
    pub fn new(root: &'a SgfNode<go::Prop>) -> Self {
        let mut stack = Vec::new();
        stack.push((root, 0));
        BranchesIterator { stack }
    }
}

impl<'a> Iterator for BranchesIterator<'a> {
    type Item = Vec<&'a SgfNode<go::Prop>>;

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
    fn into_moves(self) -> Vec<(Color, go::Move)>;
}

impl<'a, T> IntoMoves for T
where
    T: Iterator<Item = &'a SgfNode<go::Prop>>,
{
    fn into_moves(self) -> Vec<(Color, go::Move)> {
        let mut moves = Vec::new();
        self.for_each(|node| {
            if let Some(prop) = node.get_move() {
                match prop {
                    go::Prop::B(m) => {
                        moves.push((Color::Black, m.clone()));
                    }
                    go::Prop::W(m) => {
                        moves.push((Color::White, m.clone()));
                    }
                    _ => {}
                }
            }
        });
        moves
    }
}
