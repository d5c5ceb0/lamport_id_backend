use crate::common::consts;

//get proposal status: little than start_time, between start_time and end_time, greater than end_time
pub fn get_proposal_status(start_time: chrono::DateTime<chrono::Utc>, end_time: chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    if now < start_time {
        consts::PROPOSAL_STATUS_PENDING.to_string()
    } else if now >= start_time && now <= end_time {
        consts::PROPOSAL_STATUS_ACTIVE.to_string()
    } else {
        consts::PROPOSAL_STATUS_PASSED.to_string()
    }
}

