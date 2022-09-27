use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contador{
    valor: i8,
}

#[near_bindgen]
impl Contador {
    // obtener contador
    pub fn get_num(&self) -> i8 {
        return self.valor;
    }
    // incrementar contador
    pub fn increment(&mut self) {
        self.valor += 1;
        let log_message = format!("Incrementando el contador a {}", self.valor);
        env::log(log_message.as_bytes());
    }
    // decrementar contador
    pub fn decrement(&mut self) {
        self.valor -= 1;
        let log_message = format!("Decrementando el contador a {}", self.valor);
        env::log(log_message.as_bytes());
    }
    // resetear contador
    pub fn reseat(&mut self) {
        self.valor = 0;
        let log_message = format!("Reseteando el contador a {}", self.valor);
        env::log(log_message.as_bytes());
    }
}

fn despues_de_que_cambie(){
    env::log("El valor a cambiado".as_bytes());
}

//pruebas unitarias

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext{
        VMContext{
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0,1,2],
            predecessor_account_id: "jaine.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0,1,2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn incrementar(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contador{valor: 0};
        contract.increment();
        println!("Valor despues del incremento: {}", contract.valor);
        assert_eq!(1, contract.get_num());
    }
}