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
    #[clap(short, long, help = "Ammount to be converted")]
    ammount: f64,
    #[clap(
        short('c'),
        long("c2f"),
        help = "Flag this to make conversion from crypto to fiat"
    )]
    crypto_to_fiat: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match dotenv::from_filename("prod.env") {
        Ok(_) => {}
        Err(_) => {
            dotenv::from_filename("sandbox.env").expect(
                "Missing env file, you will need prod.env(Production) or sandbox.env(Dev).",
            );
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
                true => display(MainTokenFiat::FUNCTIONX, lib::MainTokenFiat::USD, args).await,
                _ => display(MainTokenFiat::FUNCTIONX, MainTokenFiat::SGD, args).await,
            };
        }
        _ => {
            match args.usd {
                true => display(MainTokenFiat::PUNDIX, lib::MainTokenFiat::USD, args).await,
                _ => display(MainTokenFiat::PUNDIX, MainTokenFiat::SGD, args).await,
            };
        }
    }

    Ok(())
}

async fn display(crypto: MainTokenFiat, fiat: MainTokenFiat, args: Args) {
    let price = lib::latest_price::Price::get_quotes(crypto, fiat)
        .await
        .unwrap();
    let price = round_2(price);
    
    println!("Price of 1 {crypto} :: [{price} {fiat}]");

    match &args.crypto_to_fiat {
        true => {
            let conversion = conversion(args.ammount, price, false);
            println!(
                "With {} {} -> you will get {} {}",
                args.ammount, &crypto, conversion, &fiat
            )
        }
        false => {
            let conversion = conversion(args.ammount, price, true);
            println!(
                "With {} {} -> you will get {} {}",
                args.ammount, &fiat, conversion, &crypto
            )
        }
    };
}

fn conversion(ammount: f64, price: f64, reverse: bool) -> f64 {
    match reverse {
        true => round_2(ammount / price),
        false => round_2(price * ammount),
    }
}

fn round_2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

#[cfg(test)]
mod test {
    use super::conversion;

    #[test]
    fn conversion_test() {
        let convert = conversion(100.0, 0.5, false);
        assert_eq!(50.0, convert);
        let convert = conversion(100.0, 0.5, true);
        assert_eq!(200.0, convert);
        let convert = conversion(45.1, 0.51474, false);
        assert_eq!(23.21, convert);
        let convert = conversion(45.1, 0.51474, true);
        assert_eq!(87.62, convert);
    }
}
