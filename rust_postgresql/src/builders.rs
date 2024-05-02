use crate::domain::produto::Produto;

pub mod produto;

pub trait Builder {
    type OutputType;
    fn set_produto(&mut self, produto: Produto);
    fn build(self) -> Self::OutputType;
}