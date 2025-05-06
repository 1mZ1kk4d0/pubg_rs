
# PUBG PUBLIC LIBRARY 

- A library made to reduce code in a personal project ended up becoming a public lib for everyone to use.

- the library will be constantly updated.
## Install

To install just use the command.

```bash
  cargo add pubg_rs
```


## HOW TO USE?

```rs
use pubg_rs::PubgRs;

async fn main() {

    let pubg = PubgRs::new("api_key_here");

    pubg.get_player("1mZ1kk4d0").await; //Case-sensitive
    pubg.get_player_by_id("player_id").await;
    pubg.get_season_info("player_id", "season_id").await;
    pubg.get_clan("clan_id").await;

}
```


## Source

 - [Github](https://github.com/1mZ1kk4d0/pubg_rs)

