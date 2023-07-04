use contract_query::ResponseData;
use graphql_client::{GraphQLQuery, Response};
use reqwest;
use std::error::Error;

use crate::deploy::registry::{RainNetworks, Ethereum, Polygon, Mumbai};

#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/query.graphql",
    response_derives = "Debug"
)]
pub struct ContractQuery;  

pub async fn get_transaction_hash( 
    network : RainNetworks ,
    contract_address : String
) -> Result<String, Box<dyn Error>> { 

    let variable = contract_query::Variables {
        addr: Some(contract_address),
    };

    let request_body = ContractQuery::build_query(variable);
    let client = reqwest::Client::new(); 

    let url = match network {
        RainNetworks::Polygon => {
            Polygon::default().url
        },  
        RainNetworks::Ethereum => {
            Ethereum::default().url
        }
        RainNetworks::Mumbai => {
            Mumbai::default().url
        }
    } ; 
 
    let res: reqwest::Response = client
        .post(url)
        .json(&request_body)
        .send()
        .await?; 

    let response_body: Response<contract_query::ResponseData> = res.json().await?;

    let transaction_id = response_body
        .data
        .and_then(|data: ResponseData|data.contract.unwrap().deploy_transaction.unwrap().id.into());

    Ok(transaction_id.unwrap()) 

}
