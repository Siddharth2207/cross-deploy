use graphql_client::{GraphQLQuery};
use std::error::Error;
use reqwest; 
use reqwest::Url;


#[derive(GraphQLQuery)]
#[derive(Debug)]
#[graphql(
  schema_path = "src/schema.json",
  query_path = "src/query.graphql",
  response_derives = "Debug, Serialize, Deserialize",
)]
pub struct ContractQuery;



#[tokio::main] 
async fn main() -> Result<(), Box<dyn Error>> { 


    let variable = contract_query::Variables{
      addr : Some(String::from("0xd14c2ba8779c6c4fba0d0606f4ff65ecd4c5bb38")) ,
    } ;  

    let request_body = ContractQuery::build_query(variable); 
    let client = reqwest::Client::new();
    let res = client.post("https://api.thegraph.com/subgraphs/name/rainprotocol/interpreter-registry-polygon").json(&request_body).send().await?;
    let response_body = res.json().await?;
    println!("{:#?}", response_body);
    Ok(())

}
