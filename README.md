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
api_key: xxxxxxxxxxxxxx # Optional BTSE API Key (read) if you don't have, it doesn't matter.
api_secret: xxxxxxxxxxxxxx # Optional BTSE API Secret (read) if you don't have, it doesn't matter.
symbol-pairs: BTC-USD,ETH-USD,DOT-USD,ADA-USD,BNB-USD,LINK-USD,STMX-USD # Mandatory
```

api key and secret APPLY HERE. [DOC](https://btsecom.github.io/docs/spot/en/#generating-api-key)

<br>

## CLI

1. query symbols in settings

```
rate -d
```

<br>

2. query target symbol:

```
rate -s <SYMBOL>
```

like `rate -s BTC`

<br>
<br>

## Release

Windows release-v1.0: [download](https://drive.google.com/file/d/16a72vWqbMfCsEa0OzmFxH70GhJKgrJ1H/view?usp=drive_link)

Mac OS release-v1.0: [download](https://drive.google.com/file/d/1-22AegujgGC5bN7koSEr77dqLWXXygeY/view?usp=drive_link)

  * Unzip quick_rate.tar.gz and put it into the path: `~/lib/`
  * `vim ~/.bashrc` and setup the env path: `export PATH="$HOME/lib/quick_rate:$PATH"`
  * Active the .bashrc: `source ~/.bashrc`

