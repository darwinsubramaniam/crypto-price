mod lib;
use clap::Parser;
use lib::MainTokenFiat;

#[derive(Parser, Debug)]
#[clap(author,version,about,long_about=None)]
struct Args {
    #[clap(short, long, help = "Choose USD as Fiat")]
    usd: bool,
    #[clap(short, long, help = "Choose SGD as Fiat")]
    sgd: bool,
    #[clap(short, long, help = "Choose PUNDIX as Crypto")]
    pundix: bool,
    #[clap(short, long, help = "Choose FUNCTIONX as Crypto")]
    functionx: bool,
    #[clap(short, long, help= "Ammount to be converted")]
    ammount: f64,
    #[clap(short, long, help= "Default=True for Crypto to FIAT , -ctf=false Fiat to Crypto conversion is required.")]
    cryptoToFiat:bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match dotenv::from_filename("prod.env") {
        Ok(_) => {},
        Err(_) => { 
            dotenv::from_filename("sandbox.env")
            .expect("Missing env file, you will need prod.env(Production) or sandbox.env(Dev).");
            println!("Application is running in Sandbox Mode.");
        }
    };

    let args: Args = Args::parse();
    if args.functionx && args.pundix {
        panic!("Choose either PUNDIX or FUNCTIONX as Crypto");
    }
    if args.usd && args.sgd {
        panic!("Choose either USD or SGD as Fiat");
    }

    match args.functionx {
        true => {
            match args.usd {
                true => display(MainTokenFiat::FUNCTIONX, lib::MainTokenFiat::USD).await,
                _ => display(MainTokenFiat::FUNCTIONX, MainTokenFiat::SGD).await,
            };
        }
        _ => {
            match args.usd {
                true => display(MainTokenFiat::PUNDIX, lib::MainTokenFiat::USD).await,
                _ => display(MainTokenFiat::PUNDIX, MainTokenFiat::SGD).await,
            };
        }
    }

    Ok(())
}

async fn display(crypto: MainTokenFiat, fiat: MainTokenFiat) {
    let price = lib::latest_price::Price::get_quotes(crypto, fiat)
        .await
        .unwrap();

    println!("Price of {crypto} :: [{price} {fiat}]");

}
