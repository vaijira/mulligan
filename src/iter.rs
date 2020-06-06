use crate::types::Concept;
use std::slice;

/// Iterator over concepts.
pub struct Iter<'a> {
    root: &'a Concept,
    root_visited: bool,
    stack: Vec<slice::Iter<'a, Concept>>,
}

impl<'a> Iter<'a> {
    // TODO: make this private somehow (and same for the other iterators).
    pub fn new(root: &'a Concept) -> Iter<'a> {
        Iter {
            root: root,
            root_visited: false,
            stack: vec![],
        }
    }
}

enum IterAction<'a, Concept> {
    Push(&'a Concept),
    Pop,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Concept;

    fn next(&mut self) -> Option<Self::Item> {
        use self::IterAction::*;

        // Visit each node as it is reached from its parent (with special root handling).
        if !self.root_visited {
            self.root_visited = true;
            self.stack.push(self.root.children.iter());
            return Some(self.root);
        }

        loop {
            let action = match self.stack.last_mut() {
                Some(stack_top) => match stack_top.next() {
                    Some(child) => Push(child),
                    None => Pop,
                },
                None => return None,
            };

            match action {
                Push(c) => {
                    self.stack.push(c.children.iter());
                    return Some(c);
                }
                Pop => {
                    self.stack.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fed;
    use crate::*;

    #[test]
    fn concept_iterator_test() {
        let mut assets = Concept::new(ASSETS_PATH, fed::FED_ASSETS_SERIES_NAME);
        assets.insert_concept("Assets/Liquidity and Credit Facilities/Net portfolio holdings of Commercial Paper Funding Facility LLC", types::UNDEFINED_SERIES_NAME);
        let mut it = assets.iter();
        assert_eq!(it.next().unwrap().name(), "Assets");
        assert_eq!(it.next().unwrap().name(), "Liquidity and Credit Facilities");
        assert_eq!(
            it.next().unwrap().name(),
            "Net portfolio holdings of Commercial Paper Funding Facility LLC"
        );
    }
}
