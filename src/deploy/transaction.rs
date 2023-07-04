
use ethers::providers::{Provider, Middleware, Http} ;
use ethers::types::{ H256,};
use std::error::Error;
use std::str::FromStr;

use super::registry::{RainNetworks, Ethereum, Mumbai, Polygon};

pub async fn get_transaction_data(
    from_network : RainNetworks ,
    tx_hash : String
) -> Result<String, Box<dyn Error>> { 

    let url = match from_network {
        RainNetworks::Ethereum => {
            Ethereum::default().provider
        } ,
        RainNetworks::Polygon => {
            Polygon::default().provider
        },
        RainNetworks::Mumbai => {
            Mumbai::default().provider
        }
    } ; 

    let provider = Provider::<Http>::try_from(url)?;  
    let h: H256 = H256::from_str(&tx_hash)?; 

    let tx_data = provider.get_transaction(h).await?.unwrap().input.to_string(); 
    Ok(tx_data) 

}