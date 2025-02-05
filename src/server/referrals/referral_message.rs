use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReferralStatsResponse {
    pub total_invite_count: i32,
    pub total_keys: i32,
    pub details: ReferralStatsDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReferralStatsDetail {
    pub twitter: ReferralStats,
    pub telegram: ReferralStats,
    pub others: ReferralStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReferralStats{
    pub referral_count: i32,
    pub keys: i32,
}
