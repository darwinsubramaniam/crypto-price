# crypto-price

Build a command line interface (CLI) using Rust that will allow a user to convert any input amount of FX (Function X Token) and PundiX (Pundi X Token) to SGD and USD and vice versa. More information on where you can find the price of FX and Pundi X https://coinmarketcap.com/

How it works : 
- requires prod.env file (sandbox.env can be used as reference) , replece API_KEY with your own api_key from CoinMarketCap.
  
  ``` bash
  crypto-price 0.1.0

  USAGE:
    crypto-price [OPTIONS]

  OPTIONS:
    -f, --functionx    Choose FUNCTIONX as Crypto
    -h, --help         Print help information
    -p, --pundix       Choose PUNDIX as Crypto
    -s, --sgd          Choose SGD as Fiat
    -u, --usd          Choose USD as Fiat
    -V, --version      Print version information

  ```

Tools available : Coinmarketcap API , CLI Lib - CLAP.


**DEADLINE** : FEB 25 2022

| NAME | ID |
| --- | ---|
| SGD|2808|
| USD|2781|
| Pundi X|9040|
| Function X|3884|
