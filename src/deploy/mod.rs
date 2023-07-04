
use std::error::Error;
use crate::subgraph::get_transaction_hash;
use self::{registry::RainNetworks, dis::{DISpair, replace_dis_pair}, transaction::get_transaction_data};

pub mod registry; 
pub mod transaction; 
pub mod dis; 


pub async fn deploy_contract(
    from_network : RainNetworks ,
    contract_address : String ,
    from_dis : DISpair , 
    to_dis : DISpair
) -> Result<String, Box<dyn Error>> {
    
    // Get tx hash
    let tx_data = get_transaction_hash(from_network,contract_address).await? ; 

    // Get tx data
    let tx_data = get_transaction_data(from_network, tx_data).await? ;  
    
    // Replace DIS instances 
    let tx_data = replace_dis_pair(tx_data,from_dis,to_dis).unwrap() ; 

    Ok(tx_data)
    
} 