use super::Builder;

use crate::domain::produto::Produto;

#[derive(Default)]
pub struct ProdutoBuilder {
    produto: Option<Produto>,
}

impl Builder for ProdutoBuilder {
    type OutputType = Produto;

    fn set_produto(&mut self,  produto: Produto) {
        self.produto = Some(produto);
    }

    fn build(self) -> Produto {
        let produto = self.produto.expect("Please, set a Produto type.");

        Produto::new(produto.uuid(),
            produto.descricao(),
            produto.data_cadastro(),
            produto.preco_unitario(),
        )
    }
   
}