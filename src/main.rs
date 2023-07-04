
use std::error::Error;
use cross_deploy::deploy::{registry::{ RainNetworks}, deploy_contract, dis::DISpair};

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> { 
  
  // Deploy Network 
  let from_network = RainNetworks::Mumbai ;  

  //Deploy Contract Address
  let contract_address = String::from("0x63c669b31818899cbf25fd6168af7fe91f0edcbd") ;  

  // Source Network DIS
  let from_dis = DISpair::new(
    String::from("0xc0aec5ff928aa0f46059014b8f47dfc5180181f8") , 
    String::from("0x1737406dbd51ce71f53328252e45985fdaf9df86") , 
    String::from("0x6096d73b606c758ae357805d6b5b7fd6c0a2434a") , 
  ) ; 

  // Target Network DIS
  let to_dis = DISpair::new(
    String::from("0xfd1da7eee4a9391f6fcabb28617f41894ba84cdc") , 
    String::from("0x9b8571bd2742ec628211111de3aa940f5984e82b ") , 
    String::from("0x3d7d894afc7dbfd45bf50867c9b051da8eee85e9") , 
  ) ; 
  
  // Get tx data for target network
  let data = deploy_contract(
    from_network ,
    contract_address,
    from_dis,
    to_dis
  ).await? ; 

  println!("data : {}" , data ) ; 

  Ok(())

}
