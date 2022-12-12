use std::rc::Rc;

use aoclib::geometry::Point;

#[derive(Debug, Clone)]
pub struct PathNode {
    pub location: Point,
    pub prev: Option<Rc<PathNode>>,
}

impl PathNode {
    pub fn iter(&self) -> Iter {
        let mut nodes = Vec::new();
        nodes.push(self);

        let mut current = self;
        while let Some(prev) = current.prev.as_ref() {
            current = &**prev;
            nodes.push(current);
        }

        Iter { nodes }
    }
}

pub struct Iter<'a> {
    nodes: Vec<&'a PathNode>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a PathNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.pop()
    }
}
