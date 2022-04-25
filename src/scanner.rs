use crate::abi::ABI;
use ethers::prelude::{gas_oracle::Etherscan, *};
use std::{error::Error, sync::Arc, time::Duration};

pub struct Scanner {
    pub client: Arc<Provider<Ws>>,
    pub etherscan: Etherscan,
}

impl Scanner {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let ws =
            Ws::connect("wss://rinkeby.infura.io/ws/v3/2517f48a19b547bba411dc9ca64c4529").await?;
        let provider = Provider::new(ws).interval(Duration::from_millis(2000));
        let client = Client::new(Chain::Mainnet, "IJYG8HS8XGQDMBNNTVVERIREG3UHPZMN64")?;
        let etherscan = Etherscan::new(client);

        Ok(Self {
            client: Arc::new(provider),
            etherscan: etherscan,
        })
    }

    async fn decode_function_input(&self, data: Bytes, contract: &Contract<Provider<Ws>>) {
        let by = data.to_vec();
        let selector = &by[..4];
        let params = &by[4..];

        let abi = contract.abi();
        let fns_type = abi.functions();

        for _fns in fns_type {
            let inputs = _fns.inputs.clone();
            let mut fn_input_types: Vec<String> = Vec::new();
            for _input in inputs {
                if let Some(_internal_type) = _input.internal_type {
                    fn_input_types.push(_internal_type);
                } 
            }
            let fns_signature = format!("{}{}", _fns.name, fn_input_types.join(","));
            println!("{}", fns_signature);
        }
    }

    pub async fn process_transaction(&self, tx_id: H256) -> Result<(), Box<dyn Error>> {
        if let Some(tx) = self.client.get_transaction(tx_id).await? {
            let tx_type = {
                if let Some(tx_type) = tx.transaction_type {
                    tx_type.as_u64()
                } else {
                    0u64
                }
            };
            if tx_type != 2 {
                return Ok(());
            }

            let to = match tx.to {
                Some(to) => to,
                None => H160::zero(),
            };

            if to.is_zero() {
                return Ok(());
            }

            if let Some(_abi) = ABI::from_address(format!("{:#?}", to).as_str()).await? {
                let _abi_obj = ethers::abi::Abi::load(_abi.as_bytes())?;
                let contract: Contract<Provider<Ws>> =
                    Contract::new(to, _abi_obj, Arc::clone(&self.client));
                self.decode_function_input(tx.input, &contract).await;
            }
        }
        Ok(())
    }

    pub async fn watch_transactions(&self) -> Result<(), Box<dyn Error>> {
        let client = Arc::clone(&self.client);
        let mut subscriber = client.subscribe_pending_txs().await?;
        println!("subscribing");
        while let Some(stream) = subscriber.next().await {
            self.process_transaction(stream).await?;
        }

        Ok(())
    }
}
