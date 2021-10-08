use crate::graph::prelude::*;

pub struct Constant(pub String);

impl Generator for Constant {
    type Yield = String;
    type Return = Result<String, Error>;

    fn next<R: Rng>(&mut self, _rng: &mut R) -> GeneratorState<Self::Yield, Self::Return> {
        let s = self.0.clone();
        GeneratorState::Complete(Ok(s))
    }


}

