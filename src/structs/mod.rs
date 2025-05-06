use reqwest::Client;
use serde::{Serialize, Deserialize};



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerApiResponse {
    pub data: PlayerAttributes
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayersApiResponse {
    pub data: Vec<Player>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClanApiResponse {
    pub data: ClanData
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClanData {
        pub id: String,
        pub attributes: ClanDataAttributes
            
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClanDataAttributes {
    
    #[serde(rename = "clanName")]
    pub clan_name: String,
    #[serde(rename = "clanTag")]
    pub clan_tag: String,
    #[serde(rename = "clanLevel")]
    pub clan_level: i64,
    #[serde(rename = "clanMemberCount")]
    pub clan_member_count: i64
        
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: String,
    pub attributes: Attributes,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attributes {
    #[serde(rename = "banType")]
    pub ban_type: String,
    #[serde(rename = "clanId")]
    pub clan_id: String,
    pub name: String,
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttributesClan {
    
    #[serde(rename = "clanLevel")]
    pub clan_level: String,

    #[serde(rename = "clanName")]
    pub clan_name: String,

    #[serde(rename = "clanTag")]
    pub clan_tag: String,
    
}



#[derive(Debug, Deserialize, Serialize)]
pub struct ClanSchematic {
    pub name: String,
    pub tag: String,
    #[serde(rename = "clanMemberCount")]
    pub clan_member_count: Option<i64>,
    pub level: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerDatabaseSchematic {
    pub player_id: String,
    pub expire: f64,
    pub ban_type: String,
    pub clan: Option<ClanSchematic>,
    pub low_nickname: String,
    pub nicknames: Vec<String>,
    pub rank_old: Option<SeasonInfo>,
    pub rank_lasted: Option<SeasonInfo>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SeasonResponseApi {
    #[serde(rename = "data")]
    pub data: SeasonData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SeasonData {
    #[serde(rename = "attributes")]
    pub attributes: SeasonAttributes,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SeasonAttributes {
    #[serde(rename = "rankedGameModeStats")]
    pub ranked_game_mode_stats: RankedGameModeStats,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RankedGameModeStats {
    #[serde(rename = "squad")]
    pub squad: Option<SeasonInfo>,
    #[serde(rename = "squad-fpp")]
    pub squad_fpp: Option<SeasonInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SeasonInfo {
    #[serde(rename = "currentRankPoint")]
    pub current_rank_point: i64,

    #[serde(rename = "bestRankPoint")]
    pub best_rank_point: i64,

    #[serde(rename = "currentTier")]
    pub current_tier: SeasonTierInfo,
    #[serde(rename = "bestTier")]
    pub best_tier: SeasonTierInfo,

    #[serde(rename = "roundsPlayed")]
    pub rounds_played: i64,

    // → estes podem ter decimal
    #[serde(rename = "avgRank")]
    pub avg_rank: f64,
    #[serde(rename = "avgSurvivalTime")]
    pub avg_survival_time: f64,
    #[serde(rename = "top10Ratio")]
    pub top10_ratio: f64,
    #[serde(rename = "winRatio")]
    pub win_ratio: f64,
    #[serde(rename = "kda")]
    pub kda: f64,
    #[serde(rename = "kdr")]
    pub kdr: f64,

    pub assists: i64,
    //pub knocks: i64,
    pub wins: i64,
    pub kills: i64,
    pub deaths: i64,

    #[serde(rename = "roundMostKills")]
    pub round_most_kills: i64,

    #[serde(rename = "longestKill")]
    pub longest_kill: f64,

    #[serde(rename = "headshotKills")]
    pub headshot_kills: i64,
    #[serde(rename = "headshotKillRatio")]
    pub headshot_kill_ratio: f64,

    #[serde(rename = "damageDealt")]
    pub damage_dealt: f64,
    #[serde(rename = "dBNOs")]
    pub dbnos: i64,
    #[serde(rename = "reviveRatio")]
    pub revive_ratio: f64,
    #[serde(rename = "revives")]
    pub revives: i64,
    #[serde(rename = "heals")]
    pub heals: i64,
    #[serde(rename = "boosts")]
    pub boosts: i64,
    #[serde(rename = "weaponsAcquired")]
    pub weapons_acquired: i64,
    #[serde(rename = "teamKills")]
    pub team_kills: i64,

    #[serde(rename = "playTime")]
    pub play_time: f64,
    #[serde(rename = "killStreak")]
    pub kill_streak: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SeasonTierInfo {
    #[serde(rename = "tier")]
    pub tier: String,
    #[serde(rename = "subTier")]
    pub sub_tier: String,
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerByID {
    pub data: Player
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerAttributes {
    #[serde(rename = "banType")]
    pub ban_type: String,
    pub clan_id: String,
    pub  name: String
}