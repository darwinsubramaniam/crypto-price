# crypto-price

Build a command line interface (CLI) using Rust that will allow a user to convert any input amount of FX (Function X Token) and PundiX (Pundi X Token) to SGD and USD and vice versa. More information on where you can find the price of FX and Pundi X https://coinmarketcap.com/

- User should able to choose which token they want to know the price [eg Pundi X, Function X - Focus ]
- User should able to choose which currency they want to know the price of the token [eg SGD or USD and vice versa]
- Sample Scenario :
  * 1. user input Function X Token - can be kind of selection of token, but priorites are Functon X and Pundi X.
  ```
   $  Select the token you want to know the price of: select from list below 
       1. Function X 
       2. Pundi X 
       3. ..
    ```
        *  Limiting to number as input to avoid user input error 

  * 2. user select if they want to know in SGD or USD.
  ```
  Select the pair of the FIAT: 
  1. SGD
  2. USD
  ```
  * 3. program show them the value of the token Function X in select value from **2**.
  ```
  Price of {Select Token} = <Ammount>{FIAT}
  ```
  * 4. user can decide to exit or repeat the process.
  ```
  Do you want to exit? (y/n)
  ```

Tools available : Coinmarketcap API , CLI Lib - CLAP.


**DEADLINE** : FEB 25 2022

| NAME | ID |
| --- | ---|
| SGD|2808|
| USD|2781|
| Pundi X|9040|
| Function X|3884|
