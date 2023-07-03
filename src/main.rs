use contract_query::ResponseData;
use graphql_client::{GraphQLQuery, Response};
use reqwest;
use std::error::Error;

#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/schema.json",
    query_path = "src/query.graphql",
    response_derives = "Debug"
)]
pub struct ContractQuery;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let variable = contract_query::Variables {
        addr: Some(String::from("0xd14c2ba8779c6c4fba0d0606f4ff65ecd4c5bb38")),
    };

    let request_body = ContractQuery::build_query(variable);
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.thegraph.com/subgraphs/name/rainprotocol/interpreter-registry-polygon")
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<contract_query::ResponseData> = res.json().await?;

    let transaction_id = response_body
        .data
        .and_then(|data: ResponseData|data.contract.unwrap().deploy_transaction.unwrap().id.into());

    println!("{}", transaction_id.unwrap());

    Ok(())
}
