use crate::core::{Expander, Graph, Mutation};

pub struct MockExpander<G: Graph> {
    current: usize,
    script: Vec<Vec<Mutation<G>>>,
}

impl<G: Graph> MockExpander<G> {
    pub fn new(script: Vec<Vec<Mutation<G>>>) -> Self {
        MockExpander { current: 0, script }
    }
}

impl<Ctx, G: Graph> Expander<G, Ctx> for MockExpander<G> {
    fn get_mutations(&mut self, _: &Ctx, _: <G as Graph>::Node) -> Vec<Mutation<G>> {
        if self.current >= self.script.len() {
            return Vec::new();
        }
        // Move the vector out instead of cloning
        let step = std::mem::take(&mut self.script[self.current]);
        self.current += 1;
        step
    }
}
