use anyhow::{Result};
use clap::{Parser, Subcommand, Args ,};
use cross_deploy::deploy::{dis::DISpair, registry::RainNetworks, deploy_contract};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Cli { 
    #[command(subcommand)]
    crossdeploy: CrossDeploy,  
}  

#[derive(Subcommand)]
pub enum CrossDeploy{
    /// Cross Deploy a Rain consumer contract 
    DeployConsumer(Consumer)
}

#[derive(Args, Debug)]
pub struct Consumer{

    /// origin Network to deploy contract from
    #[arg(short, long = "from-network")]
    origin_network: RainNetworks, 

    /// origin network interpreter
    #[arg(short ='i' , long = "from-interpreter")]
    from_interpreter: String,

    /// origin network store
    #[arg(short ='s' , long = "from-store")]
    from_store: String,

    /// origin network deployer
    #[arg(short ='d' , long = "from-deployer")]
    from_deployer: String, 

    /// origin network interpreter
    #[arg(short ='I' , long = "to-interpreter")]
    to_interpreter: String,

    /// origin network store
    #[arg(short ='S' , long = "to-store")]
    to_store: String,

    /// origin network deployer
    #[arg(short ='D' , long = "to-deployer")]
    to_deployer: String,

    /// origin network contract address
    #[arg(short ='c' , long = "contract-address")]
    contract_address: String
} 

pub async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::fmt::Subscriber::new())?;

    let cli = Cli::parse() ; 

    let data = match cli.crossdeploy {
        CrossDeploy::DeployConsumer(consumer) => { 

        deploy_contract(
                    consumer.origin_network ,
                    consumer.contract_address,
                    DISpair {
                        interpreter : consumer.from_interpreter , 
                        store : consumer.from_store , 
                        deployer : consumer.from_deployer
                    },
                    DISpair {
                        interpreter : consumer.to_interpreter , 
                        store : consumer.to_store , 
                        deployer : consumer.to_deployer
                    }
                    
            ).await?              
        }
    } ;

    println!("{:?}",data) ;
    Ok(())

}
