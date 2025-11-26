pub mod vegetables;
pub mod zlatan;

#[derive(Debug)]
pub struct Test {
    pub best: i32
}

impl Test {
    fn private(&mut self, best: i32) {
        self.best = best;
    }

    pub fn factory(best: i32) -> Test {
        Test { best }
    }

    pub fn debug(&self) {
        dbg!(&self);
    }

    pub fn debug2(reference: Test) {
        dbg!(reference);
    }
}
