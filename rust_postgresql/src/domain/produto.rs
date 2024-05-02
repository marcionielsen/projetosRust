
pub struct Produto {
    uuid: String,
    descricao: String,
    data_cadastro: String,
    preco_unitario: String,
}

impl Produto {

    pub fn new(uuid: String, descricao: String, data_cadastro: String, preco_unitario: String) -> Self {
        Self { uuid, descricao, data_cadastro, preco_unitario }
    }

    pub fn uuid(&self) -> String {
        self.uuid.to_string()
    }
    
    pub fn set_uudi(&mut self, uuid: &str) {
        self.uuid = uuid.to_string();
    }

    pub fn descricao(&self) -> String {
        self.descricao.to_string()
    }

    pub fn set_descricao(&mut self, descricao: &str) {
        self.descricao = descricao.to_string();
    }

    pub fn data_cadastro(&self) -> String {
        self.data_cadastro.to_string()
    }

    pub fn set_data_cadastro(&mut self, data_cadastro: &str) {
        self.data_cadastro = data_cadastro.to_string();
    }

    pub fn preco_unitario(&self) -> String {
        self.preco_unitario.to_string()
    } 

    pub fn set_preco_unitario(&mut self, preco_unitario: &str) {
        self.preco_unitario = preco_unitario.to_string();
    }

}