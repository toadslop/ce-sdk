use anyhow::Result;

pub trait Command<C, R> {
    fn run(self, ctx: &mut C) -> Result<R>;
}
