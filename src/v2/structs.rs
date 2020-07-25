pub use crate::common::structs::*;

/// Details about a user. See [`Session::user_details`](super::Session::user_details)
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserDetails {
	pub username: String,
	pub about_me: String,
	pub is_moderator: bool,
	pub is_patreon: bool,
	pub avatar_url: String,
	pub country_code: String,
	pub player_rating: f32,
	pub default_modifiers: Option<String>,
	pub rating: UserSkillsets,
}

/// Score from a top scores enumeration like [`Session::user_top_10_scores`](super::Session::user_top_10_scores)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TopScore {
	pub scorekey: String,
	pub song_name: String,
	pub ssr_overall: f32,
	pub wifescore: f32,
	pub rate: f32,
	pub difficulty: Difficulty,
	pub chartkey: String,
	pub base_msd: ChartSkillsets,
}

/// Score from a latest scores enumeration like [`Session::user_latest_scores`](super::Session::user_latest_scores)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LatestScore {
	pub scorekey: String,
	pub song_name: String,
	pub ssr_overall: f32,
	pub wifescore: f32,
	pub rate: f32,
	pub difficulty: Difficulty,

}

/// Score from a [top scores per skillset enumeration](super::Session::user_top_scores_per_skillset)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TopScorePerSkillset {
	pub song_name: String,
	pub rate: f32,
	pub wifescore: f32,
	pub chartkey: String,
	pub scorekey: String,
	pub difficulty: Difficulty,
	pub ssr: ChartSkillsets,
}

/// User's best scores in each skillset category. See [`Session::user_top_scores_per_skillset`](super::Session::user_top_scores_per_skillset)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UserTopScoresPerSkillset {
	pub overall: Vec<TopScorePerSkillset>,
	pub stream: Vec<TopScorePerSkillset>,
	pub jumpstream: Vec<TopScorePerSkillset>,
	pub handstream: Vec<TopScorePerSkillset>,
	pub stamina: Vec<TopScorePerSkillset>,
	pub jackspeed: Vec<TopScorePerSkillset>,
	pub chordjack: Vec<TopScorePerSkillset>,
	pub technical: Vec<TopScorePerSkillset>,
}

/// Generic information about a score
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScoreData {
	pub scorekey: String,
	pub modifiers: String,
	pub ssr: ChartSkillsets,
	pub wifescore: f32,
	pub rate: f32,
	pub max_combo: u32,
	pub is_valid: bool,
	pub has_chord_cohesion: bool,
	pub judgements: Judgements,
	pub replay: Option<Replay>,
	pub user: ScoreUser,
	pub song_name: String,
	pub artist: String,
	pub song_id: u32,
}

/// User information contained within a score information struct
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScoreUser {
	pub username: String,
	pub avatar: String,
	pub country_code: String,
	pub overall_rating: f32,
}

/// Score information in the context of a [chart leaderboard](super::Session::chart_leaderboard)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ChartLeaderboardScore {
	pub scorekey: String,
	pub ssr: ChartSkillsets,
	pub wifescore: f32,
	pub rate: f32,
	pub max_combo: u32,
	pub is_valid: bool,
	pub has_chord_cohesion: bool,
	pub datetime: String,
	pub modifiers: String,
	pub has_replay: bool,
	pub judgements: Judgements,
	pub user: ScoreUser,
}

/// Entry in a score leaderboard
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LeaderboardEntry {
	pub user: ScoreUser,
	pub rating: UserSkillsets,
}

/// Score goal
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScoreGoal {
	pub chartkey: String,
	pub rate: f32,
	pub wifescore: f32,
	pub time_assigned: String,
	pub time_achieved: Option<String>,
}