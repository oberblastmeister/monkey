use la_arena::Arena;

pub trait ShrinkToFit {
    fn shrink_to_fit(&mut self);
}

impl<T> ShrinkToFit for Vec<T> {
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit()
    }
}

impl<T> ShrinkToFit for Arena<T> {
    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit()
    }
}
