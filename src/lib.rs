use reqwest::Client;
use structs::{ClanApiResponse, Player, PlayerByID, PlayersApiResponse, SeasonData, SeasonResponseApi};


mod structs;

pub struct PubgRs {
    key: &'static str
}

impl PubgRs {

    pub fn new(key: impl Into<&'static str>) -> Self {
        Self { key: key.into() }
    }


    pub async fn get_player(&self, nickname: &str) -> Option<Player> {
        let client = Client::new();
    
        let response = client
            .get(format!("https://api.pubg.com/shards/steam/players?filter[playerNames]={}", nickname))
            .header("Authorization", format!("Bearer {}", self.key))
            .header("Accept", "application/vnd.api+json")
            .send()
            .await.unwrap()
            .json::<PlayersApiResponse>().await;
    
    
            match response {
                Err(_) => None,
                Ok(player) => {

                    if player.data.len() >= 1 {
                        Some(player.data[0].clone())
                    }
                    else {None}
                }
            }
    
    }


    pub async fn get_player_by_id(&self, id: &str) -> Option<Player> {
        let client = Client::new();
    
        let response = client
            .get(format!("https://api.pubg.com/shards/steam/players/{}", id))
            .header("Authorization", format!("Bearer {}", self.key))
            .header("Accept", "application/vnd.api+json")
            .send()
            .await.unwrap()
            .json::<PlayerByID>()
            .await;
    
    
            match response {
                Err(_) => None,
                Ok(player) => Some(player.data)
            }
    
    }
    
    
    pub async fn get_clan(&self, id: &str) -> Option<ClanApiResponse> {
        let client = Client::new();
    
        let response = client
            .get(format!("https://api.pubg.com/shards/steam/clans/{}", id))
            .header("Authorization", format!("Bearer {}", self.key))
            .header("Accept", "application/vnd.api+json")
            .send()
            .await.unwrap()
            .json::<ClanApiResponse>()
            .await;
    
    
            match response {
                Err(_) => None,
                Ok(player) => Some(player)
            }
    
    }

    pub async fn get_season_info(&self, player_id: &str, season_id: &str) -> Option<SeasonData> {
        let client = Client::new();
    
        let response = client
            .get(format!("https://api.pubg.com/shards/steam/players/{}/seasons/{}/ranked", player_id, season_id))
            .header("Authorization", format!("Bearer {}", self.key))
            .header("Accept", "application/vnd.api+json")
            .send()
            .await.unwrap()
            
            .json::<SeasonResponseApi>()
            .await;
    
    
            match response {
                Err(err) => {
                    println!("Erro: {}", err);
                    None},
                Ok(player) => Some({
                    player.data
                })
            }
    
    }
    
}


