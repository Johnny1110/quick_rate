# Quick Rate

<br>

---


<br>

## Desc

It's a fast crypto price query application, powered by Rust and BTSE API.


<br>

## How 

put quick_rate bin file and settings.yml together.

settings.yml example:

```yml
api_key: xxxxxxxxxxxxxx # BTSE API Key (read)
api_secret: xxxxxxxxxxxxxx # BTSE API Secret (read)
symbol-pairs: BTC-USD,ETH-USD,DOT-USD,ADA-USD,BNB-USD,LINK-USD,STMX-USD
```

api key and secret APPLY HERE. [DOC](https://btsecom.github.io/docs/spot/en/#generating-api-key)

<br>

## cli

1. query symbols in settings

```
quick_rate -d
```

<br>

2. query target symbol:

```
quick_rate -s <SYMBOL>
```

like `quick_rate -s BTC`

