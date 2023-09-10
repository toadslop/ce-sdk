use anyhow::Result;

pub mod load_api;

pub trait Command<C, R> {
    fn run(self, ctx: &mut C) -> Result<R>;
}
