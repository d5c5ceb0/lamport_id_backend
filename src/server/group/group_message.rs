use serde::Deserialize;
use crate::database::entities::groups;
use serde::Serialize;



#[derive(Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub logo: String,
    pub description: Option<String>,
    pub website: String,
    pub twitter: String,
}

//group info struct
#[derive(Debug, Clone,Serialize,Deserialize)]   
pub struct GroupInfo {
    pub group_id: String,
    pub name: String,
    pub logo: String,
    pub description: Option<String>,
    //pub website: String,
    //pub twitter: String,
    //pub updated_at: chrono::DateTime<chrono::Utc>,
    //pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<groups::Model> for GroupInfo {
    fn from(model: groups::Model) -> Self {
        Self {
            group_id: model.group_id,
            name: model.name,
            logo: model.logo,
            description: model.description,
            //website: model.website,
            //twitter: model.twitter,
            //created_at: model.created_at.into(),
            //updated_at: model.updated_at.into(),
        }
    }
}

#[derive(Deserialize)]
pub struct GetGroupListRequest {
    pub offset: i64,
    pub limit: i64,
}
