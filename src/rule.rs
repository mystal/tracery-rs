use crate::Execute;
use crate::Grammar;
use crate::Node;
use crate::Result;

static POP: &str = "POP";

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Rule(pub(crate) Vec<Node>);

impl Rule {
    pub(crate) fn new(nodes: Vec<Node>) -> Rule {
        Rule(nodes)
    }

    pub(crate) fn is_pop(&self) -> bool {
        // TODO: Clean this up.
        self.0.len() == 1 && matches!(self.0.first().unwrap().text(), Some(text) if text == POP)
    }
}

impl Execute for Rule {
    fn execute<R: ?Sized + rand::Rng>(&self, grammar: &mut Grammar, rng: &mut R) -> Result<String> {
        let parts = self
            .0
            .iter()
            .map(|n| n.execute(grammar, rng))
            .collect::<Result<Vec<String>>>()?;
        Ok(parts.join(""))
    }
}
